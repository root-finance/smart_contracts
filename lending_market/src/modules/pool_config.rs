use crate::modules::utils::is_valid_rate;
use scrypto::prelude::*;

/// Input to update liquidation threshold
#[derive(ScryptoSbor)]
pub enum UpdatePoolConfigInput {
    ProtocolInterestFeeRate(Decimal),
    ProtocolFlashloanFeeRate(Decimal),
    ProtocolLiquidationFeeRate(Decimal),

    FlashloanFeeRate(Decimal),

    DepositLimit(Option<Decimal>),
    BorrowLimit(Option<Decimal>),
    UtilizationLimit(Option<Decimal>),

    AssetType(u8),
    LiquidationBonusRate(Decimal),
    LoanCloseFactor(Decimal),

    InterestUpdatePeriod(i64),
    PriceUpdatePeriod(i64),
    PriceExpirationPeriod(i64),

    OptimalUsage(Decimal),
}

// Allow to set limit on deposit, borrow and usage limits for a pool
pub enum CheckPoolConfigLimitInput {
    DepositLimit(Decimal),
    BorrowLimit(Decimal),
    UtilizationLimit(Decimal),
}

#[derive(ScryptoSbor, Clone)]
pub struct PoolConfig {
    /// Rate to store in reserve when distributing interests for a period
    pub protocol_interest_fee_rate: Decimal,
    pub protocol_flashloan_fee_rate: Decimal,
    /// Rate to store in reserve when liquidating a position. Beware that the sum of this and `liquidation_bonus_rate` 
    /// is reflected on how much collateral is removed and setting it too high may actually prevent reduction of LTV and
    /// cause trigger of subsequent partial liquidations up to the complete liquidation of the liquidable CDP ()
    pub protocol_liquidation_fee_rate: Decimal,

    pub flashloan_fee_rate: Decimal,

    /// Arbitrary flag for asset type, 0=crypto 1=stable
    pub asset_type: u8,

    /// bonus amount to give to the liquidator
    pub liquidation_bonus_rate: Decimal,
    pub loan_close_factor: Decimal,

    /// Limit on deposit amount
    pub deposit_limit: Option<Decimal>,
    /// Limit on borrow amount
    pub borrow_limit: Option<Decimal>,
    /// Limit on pool utilization after which borrowing is disallowed
    pub utilization_limit: Option<Decimal>,

    /// Period in minutes after which interest is recomputed
    pub interest_update_period: i64,
    /// Period in minutes after which price is updated
    pub price_update_period: i64,
    /// Period in minutes after which all operations are disallowed since price information is not actual
    pub price_expiration_period: i64,

    /// Desired pool utilization, used to compute interest rate in order to encourage borrowing or supplying so that pool is this much used
    pub optimal_usage: Decimal,
}
impl PoolConfig {
    /// Perform a check on the pool configuration
    /// 
    /// *Error*
    /// - If the configuration is invalid
    pub fn check(&self) -> Result<(), String> {
        if !is_valid_rate(self.protocol_interest_fee_rate) {
            return Err("Lending fee rate must be between 0 and 1".into());
        }

        if !is_valid_rate(self.protocol_flashloan_fee_rate) {
            return Err("Flashloan fee rate must be between 0 and 1".into());
        }

        if !is_valid_rate(self.protocol_liquidation_fee_rate) {
            return Err("Liquidation fee rate must be between 0 and 1".into());
        }

        if !is_valid_rate(self.flashloan_fee_rate) {
            return Err("Flashloan fee rate must be between 0 and 1".into());
        }

        if !is_valid_rate(self.liquidation_bonus_rate) {
            return Err("Liquidation bonus rate must be between 0 and 1".into());
        }

        if !is_valid_rate(self.loan_close_factor) {
            return Err("Loan close factor must be between 0 and 1".into());
        }

        if self.deposit_limit.is_some() && self.deposit_limit.unwrap() < dec!(0) {
            return Err("Deposit limit must be positive".into());
        }

        if self.borrow_limit.is_some() && self.borrow_limit.unwrap() < dec!(0) {
            return Err("Borrow limit must be positive".into());
        }

        if self.utilization_limit.is_some() && !is_valid_rate(self.utilization_limit.unwrap()) {
            return Err("Utilization limit must be between 0 and 1".into());
        }

        if self.interest_update_period <= 0 {
            return Err("Interest update period must be greater than 0".into());
        }

        if self.price_update_period <= 0 {
            return Err("Price update period must be greater than 0".into());
        }

        if self.price_expiration_period <= 0 {
            return Err("Price expiration period must be greater than 0".into());
        }

        if self.price_expiration_period <= self.price_update_period {
            return Err("Price expiration period must be greater than price update period".into());
        }

        if !is_valid_rate(self.optimal_usage) {
            return Err("Optimal usage must be between 0 and 1".into());
        }

        Ok(())
    }

    /// Update the pool configuration
    /// 
    /// *Params*
    /// - `pool_config_input`: The input structure for the update
    /// 
    /// *Errors*
    /// - If update of the internal state fails
    pub fn update(&mut self, pool_config_input: UpdatePoolConfigInput) -> Result<(), String> {
        match pool_config_input {
            UpdatePoolConfigInput::DepositLimit(deposit_limit) => {
                self.deposit_limit = deposit_limit;
            }

            UpdatePoolConfigInput::BorrowLimit(borrow_limit) => {
                self.borrow_limit = borrow_limit;
            }

            UpdatePoolConfigInput::UtilizationLimit(utilization_limit) => {
                self.utilization_limit = utilization_limit;
            }

            UpdatePoolConfigInput::FlashloanFeeRate(flashloan_fee_rate) => {
                self.flashloan_fee_rate = flashloan_fee_rate;
            }

            UpdatePoolConfigInput::ProtocolInterestFeeRate(fee_rate) => {
                self.protocol_interest_fee_rate = fee_rate;
            }

            UpdatePoolConfigInput::ProtocolFlashloanFeeRate(fee_rate) => {
                self.protocol_flashloan_fee_rate = fee_rate;
            }

            UpdatePoolConfigInput::ProtocolLiquidationFeeRate(fee_rate) => {
                self.protocol_liquidation_fee_rate = fee_rate;
            }

            UpdatePoolConfigInput::LiquidationBonusRate(liquidation_bonus_rate) => {
                self.liquidation_bonus_rate = liquidation_bonus_rate;
            }

            UpdatePoolConfigInput::LoanCloseFactor(loan_close_factor) => {
                self.loan_close_factor = loan_close_factor;
            }

            UpdatePoolConfigInput::AssetType(asset_type) => {
                self.asset_type = asset_type;
            }

            UpdatePoolConfigInput::InterestUpdatePeriod(interest_update_period) => {
                self.interest_update_period = interest_update_period;
            }

            UpdatePoolConfigInput::PriceUpdatePeriod(price_update_period) => {
                self.price_update_period = price_update_period;
            }

            UpdatePoolConfigInput::PriceExpirationPeriod(price_expiration_period) => {
                self.price_expiration_period = price_expiration_period;
            }
            UpdatePoolConfigInput::OptimalUsage(optimal_usage) => {
                self.optimal_usage = optimal_usage;
            }
        };

        self.check()?;

        Ok(())
    }

    /// Perform a check on pool configuration limit
    /// 
    /// *Error*
    /// - If the configuration is invalid
    pub fn check_limit(&self, input: CheckPoolConfigLimitInput) -> Result<(), String> {
        match input {
            CheckPoolConfigLimitInput::DepositLimit(current_deposit) => {
                if let Some(limit) = self.deposit_limit {
                    if current_deposit > limit {
                        return Err(
                            "Deposit limit reached. Please try again with a smaller amount.".into(),
                        );
                    }
                }
            }

            CheckPoolConfigLimitInput::BorrowLimit(current_borrow) => {
                if let Some(limit) = self.borrow_limit {
                    if current_borrow > limit {
                        return Err(
                            "Borrow limit reached. Please try again with a smaller amount.".into(),
                        );
                    }
                }
            }

            CheckPoolConfigLimitInput::UtilizationLimit(current_utilization) => {
                if let Some(limit) = self.utilization_limit {
                    if current_utilization > limit {
                        return Err(
                            "Utilization limit reached. Please try again with a smaller amount."
                                .into(),
                        );
                    }
                }
            }
        };

        Ok(())
    }
}
