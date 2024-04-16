use super::operation_status::*;
use crate::lending_market::lending_market::*;
use crate::modules::{interest_strategy::*, liquidation_threshold::*, pool_config::*, utils::*};
use scrypto::blueprints::consensus_manager::*;
use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub enum LendingPoolUpdatedEventType {
    DepositState,
    LoanState,
    CollateralState,
    Interest,
    Price,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct LendingPoolUpdatedEvent {
    pub pool_res_address: ResourceAddress,
    pub event_type: LendingPoolUpdatedEventType,
    pub amount: Decimal,
}

#[derive(ScryptoSbor)]
pub struct MarketStatsPool {
    pub asset_address: ResourceAddress,
    pub total_liquidity: Decimal,
    pub total_supply: PreciseDecimal,
    pub total_borrow: Decimal,
    pub supply_apy: PreciseDecimal,
    pub borrow_apy: PreciseDecimal,
    pub deposit_limit: Option<Decimal>,
    pub borrow_limit: Option<Decimal>,
    pub utilization_limit: Option<Decimal>,
    pub optimal_usage: Decimal,
    pub ltv_limit: Decimal,
}

#[derive(ScryptoSbor)]
pub struct MarketStatsAllPools {
    pub total_supply_all_pools: PreciseDecimal,
    pub total_borrow_all_pools: Decimal,
    pub market_stats_pools: Vec<MarketStatsPool>,
}

#[derive(ScryptoSbor)]
pub struct LendingPoolState {
    /// Global pool component holding all the liquidity
    pub pool: Global<SingleResourcePool>,

    /// Vaults holding pool units locked as collateral
    pub collaterals: Vault,

    /// Reserve retention collected by the the protocol
    pub reserve: Vault,

    ///
    pub pool_res_address: ResourceAddress,

    ///* State *///

    ///
    pub price: Decimal,

    ///
    pub price_updated_at: i64,

    ///
    pub interest_rate: Decimal,

    ///
    pub interest_updated_at: i64,

    ///* Loan State *///

    ///
    pub total_loan: Decimal,

    ///
    pub total_loan_unit: Decimal,

    ///* Configs *///

    ///
    pub price_feed_comp: Global<AnyComponent>,

    ///
    pub interest_strategy: InterestStrategy,

    ///
    pub liquidation_threshold: LiquidationThreshold,

    ///
    pub pool_config: PoolConfig,

    ///
    pub operating_status: OperatingStatus,
}

impl LendingPoolState {
    ///* OPERATING STATUS METHODS *///

    pub fn check_operating_status(&self, value: OperatingService) -> Result<(), String> {
        if !self.operating_status.check(value) {
            return Err("Operation not allowed".to_string());
        }

        Ok(())
    }

    /// Get the current loan unit ratio ///

    pub fn get_loan_unit_ratio(&self) -> Result<PreciseDecimal, String> {
        // convert total_loan_unit and total_loan to PreciseDecimal to improve precision and reduce rounding errors
        let ratio = if self.total_loan != 0.into() {
            PreciseDecimal::from(self.total_loan_unit) / PreciseDecimal::from(self.total_loan)
        } else {
            1.into()
        };

        if ratio > 1.into() {
            return Err("Loan unit ratio cannot be greater than 1".to_string());
        }

        Ok(ratio)
    }

    ///* CORE LOGIC AND UTILITY METHODS *///

    pub fn contribute_proxy(&self, assets: Bucket) -> Result<Bucket, String> {
        let amount = assets.amount();

        let (pool_available_amount, _, pool_contributed_amount) = self.pool.get_pooled_amount();

        // Check if the pool deposit limit is reached
        self.pool_config
            .check_limit(CheckPoolConfigLimitInput::DepositLimit(
                pool_available_amount + pool_contributed_amount + amount,
            ))?;

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::DepositState,
            amount
        });

        Ok(self.pool.contribute(assets))
    }

    pub fn redeem_proxy(&self, assets: Bucket) -> Bucket {
        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::DepositState,
            amount: assets.amount()
        });

        self.pool.redeem(assets)
    }

    pub fn add_pool_units_as_collateral(&mut self, pool_units: Bucket) -> Result<(), String> {
        let pool_units_amount = pool_units.amount();
        if pool_units_amount == 0.into() {
            return Ok(());
        }

        if pool_units.resource_address() != self.collaterals.resource_address() {
            return Err("Pool unit resource address mismatch".into());
        }

        self.collaterals.put(pool_units);

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::CollateralState,
            amount: pool_units_amount
        });

        Ok(())
    }

    pub fn remove_pool_units_from_collateral(
        &mut self,
        pool_unit_amount: Decimal,
    ) -> Result<Bucket, String> {
        if pool_unit_amount == 0.into() {
            return Err("Pool unit amount must be positive".into());
        }

        if pool_unit_amount > self.collaterals.amount() {
            return Err("Not enough pool units to remove from collateral".into());
        }

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::CollateralState,
            amount: pool_unit_amount
        });

        Ok(self.collaterals.take_advanced(
            pool_unit_amount,
            WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
        ))
    }

    /// Handle request to increase borrowed amount.
    /// it remove requested liquidity and updated the pool loan state based on input interest strategy
    pub fn withdraw_for_borrow(&mut self, amount: Decimal) -> Result<(Bucket, Decimal), String> {
        if amount == 0.into() {
            return Err("Amount must be positive".into());
        }

        let (pool_available_amount, pool_borrowed_amount, _) = self.pool.get_pooled_amount();

        // Check if the borrow limit is reached
        self.pool_config
            .check_limit(CheckPoolConfigLimitInput::BorrowLimit(
                pool_borrowed_amount + amount,
            ))?;

        // Check if utilization rate is not exceeded

        self.pool_config
            .check_limit(CheckPoolConfigLimitInput::UtilizationLimit(
                (pool_borrowed_amount + amount)
                    / ((pool_available_amount + pool_borrowed_amount) + amount),
            ))?;

        let loan_unit = self._update_loan_unit(amount)?;

        let result = (
            self.pool.protected_withdraw(
                amount,
                WithdrawType::TemporaryUse,
                WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
            ),
            loan_unit,
        );

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::LoanState,
            amount
        });

        Ok(result)
    }

    /// Handle request to decrease borrowed amount.
    /// it add back liquidity and updated the pool loan state based on input interest strategy
    pub fn deposit_for_repay(&mut self, payment: Bucket) -> Result<Decimal, String> {
        let payment_amount = payment.amount();
        if payment.resource_address() != self.pool_res_address {
            return Err("Payment resource address mismatch".into());
        }

        let loan_unit = self._update_loan_unit(-payment.amount())?;

        self.pool
            .protected_deposit(payment, DepositType::FromTemporaryUse);

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::LoanState,
            amount: payment_amount
        });

        // returned unit should be negative or 0
        // Send back positive loan_unit to evoid confusion at higher level in the stack
        Ok(-loan_unit)
    }

    pub fn update_interest_and_price(
        &mut self,
        bypass_debounce: Option<(bool, bool)>,
    ) -> Result<(), String> {
        let now: i64 = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;

        let (bypass_price_debounce, bypass_interest_debounce) =
            bypass_debounce.unwrap_or((false, false));

        /* UPDATING PRICE */

        // Debounce price update to configured period (in minutes)
        if ((now - self.price_updated_at) / SECOND_PER_MINUTE)
            >= self.pool_config.price_update_period
            || bypass_price_debounce
        {
            let price_feed_result = get_price(self.price_feed_comp, self.pool_res_address)?;

            // Handle price update too old
            if ((now - price_feed_result.timestamp) / SECOND_PER_MINUTE)
                >= self.pool_config.price_expiration_period
            {
                return Err("Price info is too old".to_string());
            }

            self.price_updated_at = now;
            self.price = price_feed_result.price;

            Runtime::emit_event(LendingPoolUpdatedEvent {
                pool_res_address: self.pool_res_address,
                event_type: LendingPoolUpdatedEventType::Price,
                amount: Decimal::zero()
            });
        }

        /* UPDATING INTEREST RATE */

        // Debounce interest update to configured period (in minutes)
        let period_in_minute = (now - self.interest_updated_at) / SECOND_PER_MINUTE;
        if period_in_minute >= self.pool_config.interest_update_period || bypass_interest_debounce {
            let (passive_pool_utilization, active_pool_utilization) = self._get_pool_utilization();

            self.interest_updated_at = now;

            self.interest_rate = self.interest_strategy.get_interest_rate(passive_pool_utilization, self.pool_config.optimal_usage)?;

            // Calculate interest rate down to a minute (1 YEAR = 525600 minutes)
            let minute_interest_rate = PreciseDecimal::ONE + (self.interest_rate / MINUTE_PER_YEAR);

            let new_total_loan_amount =
                self.total_loan * minute_interest_rate.checked_powi(period_in_minute).unwrap();

            let passive_accrued_interest_amount = new_total_loan_amount - self.total_loan;

            self.total_loan = new_total_loan_amount
                .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                .unwrap();

            let active_accrued_interest_amount: PreciseDecimal = active_pool_utilization * passive_accrued_interest_amount * (1 - self.pool_config.protocol_interest_fee_rate);
            // Virtually increase pooled liquidity with accrued interest amount
            self.pool.increase_external_liquidity(
                passive_accrued_interest_amount
                    .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                    .unwrap(),
                    InterestType::Passive
            );
            self.pool.increase_external_liquidity(
                active_accrued_interest_amount
                    .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                    .unwrap(),
                    InterestType::Active
            );


            Runtime::emit_event(LendingPoolUpdatedEvent {
                pool_res_address: self.pool_res_address,
                event_type: LendingPoolUpdatedEventType::Interest,
                amount: Decimal::zero()
            });
        }

        Ok(())
    }

    pub fn collect_reserve(&mut self) -> Bucket {
        let (_, passive_interest_amount, _) = self.pool.get_pooled_amount();
        self.update_interest_and_price(Some((true, true)))
            .expect("update interest and price before collect reserve");
        let (_, active_pool_utilization) = self._get_pool_utilization();

        let reserve_amount = active_pool_utilization * passive_interest_amount * self.pool_config.protocol_interest_fee_rate;
        self.reserve.put(self.pool.protected_withdraw(
            reserve_amount,
            WithdrawType::LiquidityWithdrawal,
            WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
        ));
        
        self.reserve.take_all()
    }

    ///* PRIVATE UTILITY METHODS *///

    fn _get_pool_utilization(&self) -> (Decimal, Decimal) {
        let (available_amount, passive_interest_amount, active_interest_amount) = self.pool.get_pooled_amount();
        let total_passive_liquidity = passive_interest_amount + available_amount;
        let passive_utilization = if total_passive_liquidity == 0.into() {
            Decimal::ZERO
        } else {
            passive_interest_amount / total_passive_liquidity
        };

        let total_active_liquidity = active_interest_amount + available_amount;
        let active_utilization = if total_active_liquidity == 0.into() {
            Decimal::ZERO
        } else {
            active_interest_amount / total_active_liquidity
        };
        (passive_utilization, active_utilization)
    }

    fn _update_loan_unit(&mut self, amount: Decimal) -> Result<Decimal, String> {
        let unit_ratio = self.get_loan_unit_ratio()?;

        let units = (amount * unit_ratio) //
            .checked_truncate(RoundingMode::ToNearestMidpointToEven)
            .unwrap();

        self.total_loan += amount;

        self.total_loan_unit += units;

        if self.total_loan_unit < 0.into() {
            return Err("Total loan unit cannot be negative".to_string());
        }

        if self.total_loan < 0.into() {
            return Err("Total loan cannot be negative".to_string());
        }

        Ok(units)
    }
}
