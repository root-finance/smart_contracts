use scrypto::prelude::*;

/// Input to update market configuration
#[derive(ScryptoSbor)]
pub enum UpdateMarketConfigInput {
    MaxCDPPosition(u8),
    MaxLiquidableValue(Decimal),
    LiquidationDexSwapRate(Decimal),
}

/// The lending market configuration
#[derive(ScryptoSbor, Clone)]
pub struct MarketConfig {
    /// Max positions per CDP per user
    pub max_cdp_position: u8,
    /// Max liquidable value to take out of collateral when partially liquidating a CDP (rate)
    pub max_liquidable_value: Decimal,
    /// Dex swap efficiency, where 1 means the whole collateral is converted to loan to liquidate, but often this is lesser than 1 so 
    /// a certain tolerance on the fact that not all collateral is used to extinguish the loan is given this way
    pub liquidation_dex_swap_rate: Decimal
}
impl MarketConfig {
    /// Perform a check on the market configuration
    /// 
    /// *Error*
    /// - If the configuration is invalid
    pub fn check(&self) -> Result<(), String> {
        if self.max_cdp_position == 0 {
            return Err("Max CDP position must be greater than 0".into());
        }
        if self.max_liquidable_value < dec!(0) || self.max_liquidable_value > dec!(1) {
            return Err("Max liquidable value must be in range 0..1".into());
        }
        if self.liquidation_dex_swap_rate < dec!(0) || self.liquidation_dex_swap_rate > dec!(1) {
            return Err("Liquidation dex swap rate value must be in range 0..1".into());
        }

        Ok(())
    }

    /// Update the liquidation threshold configuration
    /// 
    /// *Params*
    /// - `pool_config_input`: The input structure for the update
    /// 
    /// *Errors*
    /// - If update of the internal state fails
    pub fn update(&mut self, pool_config_input: UpdateMarketConfigInput) -> Result<(), String> {
        match pool_config_input {
            UpdateMarketConfigInput::MaxCDPPosition(max_cdp_position) => {
                self.max_cdp_position = max_cdp_position;
            }
            UpdateMarketConfigInput::MaxLiquidableValue(max_liquidable_value) => {
                self.max_liquidable_value = max_liquidable_value;
            }
            UpdateMarketConfigInput::LiquidationDexSwapRate(liquidation_dex_swap_rate) => {
                self.liquidation_dex_swap_rate = liquidation_dex_swap_rate;
            }
        }

        self.check()?;

        Ok(())
    }
}
