use super::operation_status::*;
use crate::lending_market::lending_market::*;
use crate::modules::{interest_strategy::*, liquidation_threshold::*, pool_config::*, utils::*};
use scrypto::blueprints::consensus_manager::*;
use scrypto::prelude::*;

/// Type of event occurring on pool update
#[derive(ScryptoSbor)]
pub enum LendingPoolUpdatedEventType {
    DepositState,
    LoanState,
    CollateralState,
    Interest,
    Price,
}

/// Event occurring on pool update
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct LendingPoolUpdatedEvent {
    /// The pool resource address
    pub pool_res_address: ResourceAddress,
    /// The type of event occurred
    pub event_type: LendingPoolUpdatedEventType,
    /// Amount associated to the event, like deposited or loaned asset
    pub amount: Decimal,
}

/// Market stats of a pool
#[derive(ScryptoSbor)]
pub struct MarketStatsPool {
    /// The pool resource address
    pub asset_address: ResourceAddress,
    /// The liquidity available for borrow
    pub available_liquidity: Decimal,
    /// The liquidity available for borrow plus the quantity generated out of interests
    pub total_liquidity: Decimal,
    /// The total amount of contributions
    pub total_supply: Decimal,
    /// The total amount of borrows
    pub total_borrow: Decimal,
    /// The effective APY compound interest for supply
    pub supply_apy: PreciseDecimal,
    /// The effective APY compound interest for borrow
    pub borrow_apy: PreciseDecimal,
    /// The limit on the maximum deposit amount possible
    pub deposit_limit: Option<Decimal>,
    /// The limit on the maximum borrowable amount possible
    pub borrow_limit: Option<Decimal>,
    /// The limit on the maximum utilization ratio
    pub utilization_limit: Option<Decimal>,
    /// The desired utilization ratio
    pub optimal_usage: Decimal,
    /// Liquidation threshold
    pub ltv_limit: Decimal,
}

/// Market stats of all pools
#[derive(ScryptoSbor)]
pub struct MarketStatsAllPools {
    /// Cumulated supply
    pub total_supply_all_pools: Decimal,
    /// Cumulated borrow
    pub total_borrow_all_pools: Decimal,
    /// The list of stats for every pool
    pub market_stats_pools: Vec<MarketStatsPool>,
}

/// The pool state
#[derive(ScryptoSbor)]
pub struct LendingPoolState {
    /// Global pool component holding all the liquidity
    pub pool: Global<SingleResourcePool>,

    /// Vaults holding pool units locked as collateral
    pub collaterals: Vault,

    /// Reserve retention collected by the the protocol
    pub reserve: Vault,

    /// The pool resource address
    pub pool_res_address: ResourceAddress,

    ///* State *///

    /// The asset price
    pub price: Decimal,

    /// The timestamp when the price update happened
    pub price_updated_at: i64,

    /// The interest rate
    pub interest_rate: Decimal,

    /// The timestamp when the interest was updated
    pub interest_updated_at: i64,

    ///* Loan State *///

    /// The total loan amount
    pub total_loan: PreciseDecimal,

    /// The total deposit amount
    pub total_deposit: PreciseDecimal,

    /// The total loan unit
    pub total_loan_unit: PreciseDecimal,

    /// The total deposit unit
    pub total_deposit_unit: PreciseDecimal,

    ///* Configs *///

    /// The price oracle component
    pub price_feed_comp: Global<AnyComponent>,

    /// The pool interest strategy
    pub interest_strategy: InterestStrategy,

    /// The pool liquidation threshold
    pub liquidation_threshold: LiquidationThreshold,

    /// The pool config
    pub pool_config: PoolConfig,

    /// The operating status
    pub operating_status: OperatingStatus,

    /// The pool utilization
    pub pool_utilization: Decimal,

    /// The total reserve amount from pool operation fees
    pub total_reserved_amount: Decimal,
}

impl LendingPoolState {
    /* OPERATING STATUS METHODS */

    /// Perform a check on the operating status of the pool
    /// 
    /// *Params*
    /// - `value`: The wanted operating status
    /// 
    /// *Error*
    /// - If the operating status is not allowed at this time
    pub fn check_operating_status(&self, value: OperatingService) -> Result<(), String> {
        if !self.operating_status.check(value) {
            return Err("Operation not allowed".to_string());
        }

        Ok(())
    }

    /// Getter of the current loan unit ratio
    /// 
    /// *Error*
    /// - If the loan to unit ratio indicates an unhealthy pool
    pub fn get_loan_unit_ratio(&self) -> Result<PreciseDecimal, String> {
        let ratio = if self.total_loan != 0.into() {
            self.total_loan_unit / self.total_loan
        } else {
            1.into()
        };

        if ratio > 1.into() {
            return Err(format!("Loan unit ratio cannot be greater than 1, was {} (total_loan is {})", ratio, self.total_loan));
        }

        Ok(ratio)
    }

    /// Getter of the current deposit unit ratio
    /// 
    /// *Error*
    /// - If the deposit to unit ratio indicates an unhealthy pool

    pub fn get_deposit_unit_ratio(&self) -> Result<PreciseDecimal, String> {
        let ratio = if self.total_deposit != 0.into() {
            self.total_deposit_unit / self.total_deposit
        } else {
            1.into()
        };

        if ratio > 1.into() {
            return Err(format!("Deposit unit ratio cannot be greater than 1, was {} (total_deposit is {})", ratio, self.total_deposit));
        }

        Ok(ratio)
    }

    /* CORE LOGIC AND UTILITY METHODS */

    /// Proxy of the pool component contribute method
    /// 
    /// *Params*
    /// - `asset`: assets to contribute
    /// 
    /// *Output*
    /// - pool unit equivalent of the contribution
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn contribute_proxy(&mut self, assets: Bucket) -> Result<Bucket, String> {
        let amount = assets.amount();

        let (pool_available_amount, pool_borrowed_amount) = self.pool.get_pooled_amount();

        // Check if the pool deposit limit is reached
        self.pool_config
            .check_limit(CheckPoolConfigLimitInput::DepositLimit(
                pool_available_amount + pool_borrowed_amount + amount,
            ))?;

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::DepositState,
            amount
        });

        let contributed = self.pool.contribute(assets);
        self._update_deposit_unit(amount)?;
        self.update_interest_and_price(Some((true, true)))?;
        Ok(contributed)
    }

    /// Proxy of the pool component redeem method
    /// 
    /// *Params*
    /// - `pool_units`: pool units to redeem
    /// - `bypass_state_update`: flag that allows to bypass the pool state update, reducing transaction cost
    /// 
    /// *Output*
    /// - pool unit equivalent of the contribution
    pub fn redeem_proxy(&mut self, pool_units: Bucket, bypass_state_update: bool) -> Bucket {
        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::DepositState,
            amount: -pool_units.amount()
        });

        let unit_ratio = pool_units.amount() / self.total_deposit_unit;
        
        // Reseved amount has already been deducted from the pool. Remove it from the redeemed amount
        let reserved_amount_for_position = (unit_ratio * self.total_reserved_amount)
            .checked_truncate(RoundingMode::ToNearestMidpointToEven)
            .unwrap();
        // Logger::debug(format!("REDEEM unit_ratio * total_reserved_amount {:?} = reserved_amount_for_position {:?}", self.total_reserved_amount, reserved_amount_for_position));

        let mut redeemed = self.pool.redeem(pool_units);
        let reserve_amount = redeemed.take(reserved_amount_for_position);
        self.total_reserved_amount -= reserved_amount_for_position;
        self.reserve.put(reserve_amount);

        // Logger::debug(format!("REDEEMED AMOUNT = {:?} RESERVE AMOUNT = {:?}", redeemed.amount(), self.reserve.amount()));

        self._update_deposit_unit(-redeemed.amount())
            .expect("update deposit unit for redeem");
        if !bypass_state_update {
            self.update_interest_and_price(Some((true, true)))
                .expect("update interest and price for redeem");
        }
        redeemed
    }

    /// Add pool uints to the Vault of locked collaterals
    /// 
    /// *Params*
    /// - `pool_units`: pool units to add to the vault
    /// 
    /// *Error*
    /// - If update of the internal state fails
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

    /// Return pool uints from the Vault of locked collaterals
    /// 
    /// *Params*
    /// - `pool_units`: pool units to remove from the vault
    /// 
    /// *Output*
    /// - The removed pool units
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn remove_pool_units_from_collateral(
        &mut self,
        pool_unit_amount: PreciseDecimal,
    ) -> Result<Bucket, String> {
        let pool_unit_amount = pool_unit_amount
            .checked_truncate(RoundingMode::ToNearestMidpointToEven)
            .unwrap();
        if pool_unit_amount > self.collaterals.amount() {
            return Err("Not enough pool units to remove from collateral".into());
        }

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::CollateralState,
            amount: -pool_unit_amount
        });

        if pool_unit_amount == 0.into() {
            Ok(Bucket::new(self.pool_res_address))
        } else {
            Ok(self.collaterals.take_advanced(
                pool_unit_amount,
                WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
            ))
        }
    }

    /// Handle request to increase borrowed amount.
    /// it remove requested liquidity and updated the pool loan state based on input interest strategy
    /// 
    /// *Params*
    /// - `amount`: asset to withdraw
    /// 
    /// *Output*
    /// - Withdrawn asset
    /// - Equivalent amount of pool units loaned
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn withdraw_for_borrow(&mut self, amount: Decimal) -> Result<(Bucket, Decimal), String> {
        if amount == 0.into() {
            return Err("Amount must be positive".into());
        }

        let (pool_available_amount, pool_borrowed_amount) = self.pool.get_pooled_amount();

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
            loan_unit
                .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                .unwrap(),
        );

        self.update_interest_and_price(Some((true, true)))?;

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::LoanState,
            amount: -amount
        });

        Ok(result)
    }

    /// Handle request to decrease borrowed amount.
    /// it add back liquidity and updated the pool loan state based on input interest strategy
    /// 
    /// *Params*
    /// - `payment`: asset to deposit
    /// - `bypass_state_update`: flag that allows to bypass the pool state update, reducing transaction cost
    /// 
    /// *Output*
    /// - Equivalent amount of loan pool units repayed
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn deposit_for_repay(&mut self, payment: Bucket, bypass_state_update: bool) -> Result<PreciseDecimal, String> {
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

        if !bypass_state_update {
            self.update_interest_and_price(Some((true, true)))?;
        }

        // returned unit should be negative or 0
        // Send back positive loan_unit to evoid confusion at higher level in the stack
        Ok(-loan_unit)
    }

    /// Update interest and price, keeping the pool state in sync
    /// The update is costly and can be executed at fixed intervals even if the calls are more
    /// frequent, which is called debouncing.
    /// 
    /// *Params*
    /// - `bypass_debounce`: Optional tuple that indicates
    ///     1. bypass price debounce
    ///     2. bypass interest debounce
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn update_interest_and_price(
        &mut self,
        bypass_debounce: Option<(bool, bool)>,
    ) -> Result<(), String> {
        let now: i64 = Clock::current_time(TimePrecision::Second).seconds_since_unix_epoch;

        let (bypass_price_debounce, bypass_interest_debounce) =
            bypass_debounce.unwrap_or((false, false));

        /* UPDATING PRICE */

        // Debounce price update to configured period (in minutes)
        if ((now - self.price_updated_at) / SECOND_PER_MINUTE)
            >= self.pool_config.price_update_period
            || bypass_price_debounce
        {
            self._update_price(now)?;
        }

        /* UPDATING INTEREST RATE */

        // Debounce interest update to configured period (in minutes, Radix time resolution)
        
        let period_in_seconds = now - self.interest_updated_at;
        if period_in_seconds >= self.pool_config.interest_update_period || bypass_interest_debounce {
            self._update_interest(now, period_in_seconds)?;
        }

        Ok(())
    }

    /// Getter of the pool utilization
    pub fn get_pool_utilization(&self) -> Decimal {
        let (available_amount, borrowed_amount) = self.pool.get_pooled_amount();
        let pool_total_liquidity = available_amount + borrowed_amount;
        if pool_total_liquidity == 0.into() {
            Decimal::ZERO
        } else {
            borrowed_amount / pool_total_liquidity
        }
    }

    /* PRIVATE UTILITY METHODS */

    fn _update_price(&mut self, now: i64) -> Result<(), String> {
        let price_feed_result = get_price(self.price_feed_comp, self.pool_res_address)?;

        // Handle price update too old
        if ((now - price_feed_result.timestamp) / SECOND_PER_MINUTE)
            >= self.pool_config.price_expiration_period
        {
            return Err("Price info is too old".to_string());
        }

        self.price_updated_at = now;
        self.price = price_feed_result.price.into();

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::Price,
            amount: Decimal::zero()
        });

        Ok(())
    }

    fn _update_interest(&mut self, now: i64, period_in_seconds: i64) -> Result<(), String> {
        let one_year_in_seconds: PreciseDecimal = 31_556_952.into();
        self.pool_utilization = if self.pool_utilization == Decimal::ZERO { self.get_pool_utilization() } else { self.pool_utilization };
            
        let active_interest_rate = self.pool_utilization * self.interest_rate * (PreciseDecimal::ONE - self.pool_config.protocol_interest_fee_rate);

        // Logger::debug(format!("INTEREST period_in_seconds: {:?}", period_in_seconds)); 

        // Logger::debug(format!("INTEREST interest_rate {:?} - pool_utilization {:?} - active_interest_rate {:?}", self.interest_rate, self.pool_utilization, active_interest_rate));

        let new_total_loan_amount = self.interest_rate * (PreciseDecimal::ONE + self.total_loan) * period_in_seconds / one_year_in_seconds;
        let new_total_deposit_amount = active_interest_rate * (PreciseDecimal::ONE + self.total_deposit) * period_in_seconds / one_year_in_seconds;

        let reserve_delta: Decimal = (new_total_loan_amount - new_total_deposit_amount)
            .checked_truncate(RoundingMode::ToNearestMidpointToEven)
            .unwrap();
        self.total_reserved_amount += reserve_delta;

        let mut accrued_interest_amount = if new_total_loan_amount - self.total_loan < PreciseDecimal::ZERO { new_total_loan_amount } else { new_total_loan_amount - self.total_loan };

        // Logger::debug(format!("INTEREST new_total_loan_amount {:?} ; new_total_deposit_amount {:?}; reserve_delta {:?}; accrued_interest_amount {:?}", new_total_loan_amount, new_total_deposit_amount, reserve_delta, accrued_interest_amount));

        if self.total_loan + new_total_loan_amount > self.total_deposit {
            self.total_deposit += new_total_deposit_amount;
            let new_total_loan_amount = self.total_deposit - self.total_loan;
            accrued_interest_amount = if new_total_loan_amount - self.total_loan < PreciseDecimal::ZERO { new_total_loan_amount } else { new_total_loan_amount - self.total_loan };
            self.total_loan = self.total_deposit;
        } else {
            self.total_loan += new_total_loan_amount;
            self.total_deposit += new_total_deposit_amount;
        }

        // Logger::debug(format!("INTEREST updated totals: total_loan {:?} - total_deposit {:?}", self.total_loan, self.total_deposit));

        // Virtually increase pooled liquidity with accrued interest amount
        if accrued_interest_amount > PreciseDecimal::ZERO {
            self.pool.increase_external_liquidity(accrued_interest_amount
                .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                .unwrap());
        }

        self.pool_utilization = self.get_pool_utilization();
        let interest_rate = self.interest_strategy.get_interest_rate(self.pool_utilization, self.pool_config.optimal_usage)?;
        if interest_rate != self.interest_rate {
            self.interest_updated_at = now;
            self.interest_rate = interest_rate;
            // Logger::debug(format!("INTEREST update: now {:?} - pool_utilization {:?} - interest_rate {:?}", self.interest_updated_at, self.pool_utilization, self.interest_rate));
        }

        Runtime::emit_event(LendingPoolUpdatedEvent {
            pool_res_address: self.pool_res_address,
            event_type: LendingPoolUpdatedEventType::Interest,
            amount: Decimal::zero()
        });

        Ok(())
    }

    fn _update_loan_unit(&mut self, amount: Decimal) -> Result<PreciseDecimal, String> {
        let unit_ratio = self.get_loan_unit_ratio()?;

        let units = amount * unit_ratio;

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

    fn _update_deposit_unit(&mut self, amount: Decimal) -> Result<PreciseDecimal, String> {
        let unit_ratio = self.get_deposit_unit_ratio()?;

        let units = amount * unit_ratio;

        self.total_deposit += amount;

        self.total_deposit_unit += units;

        if self.total_deposit_unit < 0.into() {
            self.total_deposit_unit = 0.into();
        }

        if self.total_deposit < 0.into() {
            self.total_deposit = 0.into();
        }

        Ok(units)
    }
}
