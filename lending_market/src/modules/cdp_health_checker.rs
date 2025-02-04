use super::{cdp_data::*, liquidation_threshold::*, pool_state::*};
use scrypto::prelude::*;

// Amount at which a position is considered zeroed
pub const ZERO_EPSILON: Decimal = dec!(0.000000001);

/// Type of position
pub enum LoadPositionType {
    Collateral,
    Loan,
}

/// Type of load
pub enum LoadDataType {
    Own,
}
/// Position data
#[derive(ScryptoSbor, Debug, Clone)]
pub struct PositionData {
    /// Unit amount
    pub units: PreciseDecimal,
    /// Asset amount
    pub amount: Decimal,
    /// Value of the position according to the asset price
    pub value: Decimal,
    /// Pool unit ratio from the respective pool
    pub unit_ratio: PreciseDecimal,
}
impl PositionData {
    /// Load the position data
    /// 
    /// *Params*
    /// - `units``: The position units
    /// - `load_type`: The load type
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn load_onledger_data(
        &mut self,
        units: PreciseDecimal,
        load_type: LoadDataType,
    ) -> Result<(), String> {
        match load_type {
            LoadDataType::Own => self.units += units,
        }

        Ok(())
    }

    /// Update the position data
    /// 
    /// *Params*
    /// - `price``: This position's asset price
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn update_data(&mut self, price: Decimal) -> Result<(), String> {
        self.amount = (self.units / self.unit_ratio)
            .checked_truncate(RoundingMode::ToNearestMidpointToEven)
            .unwrap();
        self.value = self.amount * price;
        Ok(())
    }
}

/// Extends the collateral position with necessary information for the CDP health check
#[derive(ScryptoSbor, Clone, Debug)]
pub struct ExtendedCollateralPositionData {
    /// The pool resource address
    pub pool_res_address: ResourceAddress,
    /// Price of the asset
    pub price: Decimal,
    /// Type of the asset from pool state. This is defined at the time pool component is instantiated
    /// but it's expected to have 0 for cryptocurrencies and 1 for stable coins
    pub asset_type: u8,
    /// Liquidation threshold information from pool state
    pub liquidation_threshold: LiquidationThreshold,
    /// Liquidation bonus rate information from pool state
    pub liquidation_bonus_rate: Decimal,
    /// The position data
    pub data: PositionData,
}
impl ExtendedCollateralPositionData {
    /// Load the `ExtendedCollateralPositionData` data
    /// 
    /// *Params*
    /// - `units``: The position units
    /// - `load_type`: The load type
    /// - `pool_state`: Reference to the on-chain key-value storage entry where key is asset resource 
    ///                 address and value is the current pool state
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn load_onledger_data(
        &mut self,
        units: PreciseDecimal,
        load_type: LoadDataType,
        pool_state: &KeyValueEntryRef<'_, LendingPoolState>,
    ) -> Result<(), String> {
        self.data.load_onledger_data(units, load_type)?;

        if self.data.unit_ratio == pdec!(0) {
            self.data.unit_ratio = pool_state.pool.get_pool_unit_ratio();
        };

        Ok(())
    }

    /// Update the `ExtendedCollateralPositionData` data by mean of the current asset price
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn update_data(&mut self) -> Result<(), String> {
        self.data.update_data(self.price)
    }
}

/// Extends the loan position with necessary information for the CDP health check
#[derive(ScryptoSbor, Clone, Debug)]
pub struct ExtendedLoanPositionData {
    /// The pool resource address
    pub pool_res_address: ResourceAddress,
    /// Price of the asset
    pub price: Decimal,
    /// Type of the asset from pool state. This is defined at the time pool component is instantiated
    /// but it's expected to have 0 for cryptocurrencies and 1 for stable coins
    pub asset_type: u8,
    /// Ratio to be repaid before the loan can be closed
    pub loan_close_factor: Decimal,
    /// The position data
    pub data: PositionData,
    /// The amount of collateral needed to sustain this loan so that the position is not liquidable
    pub discounted_collateral_value: Decimal,
}
impl ExtendedLoanPositionData {
    /// Load the `ExtendedLoanPositionData` data
    /// 
    /// *Params*
    /// - `units``: The position units
    /// - `load_type`: The load type
    /// - `pool_state`: Reference to the on-chain key-value storage entry where key is asset resource 
    ///                 address and value is the current pool state
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn load_onledger_data(
        &mut self,
        units: PreciseDecimal,
        load_type: LoadDataType,
        pool_state: &KeyValueEntryRef<'_, LendingPoolState>,
    ) -> Result<(), String> {
        self.data.load_onledger_data(units, load_type)?;

        if self.data.unit_ratio == pdec!(0) {
            self.data.unit_ratio = pool_state.get_loan_unit_ratio()?;
        };

        Ok(())
    }

    /// Update the `ExtendedLoanPositionData` data by mean of the respective collateral positions
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn update_data(
        &mut self,
        collateral_positions: &IndexMap<ResourceAddress, ExtendedCollateralPositionData>,
    ) -> Result<(), String> {
        self.data.update_data(self.price)?;

        self.discounted_collateral_value = collateral_positions.iter().fold(
            Decimal::ZERO,
            |mut discounted_collateral_value, (_, collateral_position)| {
                let mut liquidation_threshold =
                    collateral_position.liquidation_threshold.get_ratio(
                        collateral_position.pool_res_address,
                        collateral_position.asset_type,
                        self.pool_res_address,
                        self.asset_type,
                    );

                liquidation_threshold = (Decimal::ONE - collateral_position.liquidation_bonus_rate)
                    .min(liquidation_threshold);

                discounted_collateral_value += liquidation_threshold
                    * collateral_position.data.value;

                discounted_collateral_value
            },
        );

        Ok(())
    }
}

///
/// Extends the CDP with necessary information for the CDP health check and call method of the related lending pool
/// In addition the Extended CDP can combine multiple CDP and perform health check on the batch.
///
#[derive(ScryptoSbor, Clone, Debug)]
pub struct CDPHealthChecker {
    /// The type of the CDP
    pub cdp_type: CDPType,

    /// The total value of the loan in the CDP 
    pub total_loan_value: Decimal,

    /// The loan to value ratio is the ratio between the total loan value, and the total collateral value.
    pub total_loan_to_value_ratio: Decimal,

    /// Max loan value in the CDP that can be repaid
    pub self_closable_loan_value: Decimal,
    
    /// IndexMap of all the collateral positions in the CDP. The key is the resource address of the asset used as collateral.
    pub collateral_positions: IndexMap<ResourceAddress, ExtendedCollateralPositionData>,

    /// IndexMap of all the loan positions in the CDP. the key is the resource address of borrowed the asset
    pub loan_positions: IndexMap<ResourceAddress, ExtendedLoanPositionData>,
}
impl CDPHealthChecker {
    /// Constructor with pool state update side effect
    /// 
    /// *Params*
    /// - `wrapped_cdp_data``: The CDP to check
    /// - `load_type`: The load type
    /// - `pool_state`: Reference to the on-chain key-value storage where key is asset resource 
    ///                 address and value is the current pool state
    /// 
    /// *Output*
    /// `CDPHealthChecker`
    pub fn new(
        wrapped_cdp_data: &WrappedCDPData,
        pool_states: &mut KeyValueStore<ResourceAddress, LendingPoolState>,
    ) -> CDPHealthChecker {
        Self::update_interest_and_price(wrapped_cdp_data, pool_states).expect("Error updating interest and price for CDP health checker");
        Self::create_health_checker(wrapped_cdp_data, pool_states)
            .expect("Error creating CDP health checker")
    }

    /// Constructor without pool state update side effect
    /// 
    /// *Params*
    /// - `wrapped_cdp_data``: The CDP to check
    /// - `load_type`: The load type
    /// - `pool_state`: Reference to the on-chain key-value storage where key is asset resource 
    ///                 address and value is the current pool state
    /// 
    /// *Output*
    /// `CDPHealthChecker`
    pub fn new_without_update(
        wrapped_cdp_data: &WrappedCDPData,
        pool_states: &KeyValueStore<ResourceAddress, LendingPoolState>,
    ) -> CDPHealthChecker {
        Self::create_health_checker(wrapped_cdp_data, pool_states)
            .expect("Error creating CDP health checker")
    }

    fn update_interest_and_price(wrapped_cdp_data: &WrappedCDPData, pool_states: &mut KeyValueStore<ResourceAddress, LendingPoolState>) -> Result<(), String> {
        let cdp_data: CollaterizedDebtPositionData = wrapped_cdp_data.get_data();

        // Load the collateral positions
        cdp_data
            .collaterals
            .iter()
            .for_each(|(pool_res_address, _)| {
                if let Some(mut pool_state) = pool_states.get_mut(pool_res_address) {
                    pool_state.update_interest_and_price(None).expect("update interest and price");
                }
            });

        // Load the loan positions
        cdp_data
            .loans
            .iter()
            .for_each(|(pool_res_address, _)| {
                if let Some(mut pool_state) = pool_states.get_mut(pool_res_address) {
                    pool_state.update_interest_and_price(None).expect("update interest and price");
                }
            });

        Ok(())
    }

    fn create_health_checker(
        wrapped_cdp_data: &WrappedCDPData,
        pool_states: &KeyValueStore<ResourceAddress, LendingPoolState>,
    ) -> Result<CDPHealthChecker, String> {
        let cdp_data: CollaterizedDebtPositionData = wrapped_cdp_data.get_data();

        let mut extended_cdp = CDPHealthChecker {
            cdp_type: cdp_data.cdp_type.clone(),
            collateral_positions: IndexMap::new(),
            loan_positions: IndexMap::new(),
            total_loan_value: Decimal::ZERO,
            total_loan_to_value_ratio: Decimal::ZERO,
            self_closable_loan_value: Decimal::ZERO,
        };

        // Function to load collateral or loan positions
        let mut load_data = |pool_res_address: &ResourceAddress,
                             units: PreciseDecimal,

                             position_type: LoadPositionType| {
            let wrapped_pool_state = pool_states.get(pool_res_address);
            if wrapped_pool_state.is_none() {
                return Err("Pool state not found".to_string());
            };

            let pool_state = wrapped_pool_state.unwrap();

            match position_type {
                LoadPositionType::Collateral => {
                    let collateral_position =
                        extended_cdp.get_collateral_position(&pool_state)?;
                    collateral_position.load_onledger_data(
                        units,
                        LoadDataType::Own,
                        &pool_state,
                    )?
                }
                LoadPositionType::Loan => {
                    let loan_position = extended_cdp._get_loan_position(&pool_state)?;
                    loan_position.load_onledger_data(units, LoadDataType::Own, &pool_state)?;
                }
            }

            Ok(())
        };

        // Load the collateral positions
        cdp_data
            .collaterals
            .iter()
            .try_for_each(|(pool_res_address, units)| {
                load_data(pool_res_address, *units, LoadPositionType::Collateral)
            })?;

        // Load the loan positions
        cdp_data
            .loans
            .iter()
            .try_for_each(|(pool_res_address, units)| {
                load_data(pool_res_address, *units, LoadPositionType::Loan)
            })?;

        // Return the extended CDP
        Ok(extended_cdp)
    }

    /// Perform health check
    /// 
    /// *Error*
    /// - If the health check fails
    pub fn check_cdp(&mut self) -> Result<(), String> {
        self.update_health_check_data()?;

        if self.total_loan_to_value_ratio > Decimal::ONE {
            return Err(format!(
                "total_loan_to_value_ratio need to be lower than 1, but is {}.",
                self.total_loan_to_value_ratio
            ));
        }

        //

        Ok(())
    }

    /// Check if CDP can be liquidated
    /// 
    /// *Error*
    /// - If the CDP is not liquidable
    pub fn can_liquidate(&mut self) -> Result<(), String> {
        self.update_health_check_data()?;

        if self.total_loan_to_value_ratio <= Decimal::ONE {
            return Err(format!(
                "CDP can not be liquidated: LTV ratio of {} is lower than 1",
                self.total_loan_to_value_ratio
            ));
        }

        Ok(())
    }

    fn get_collateral_position(
        &mut self,
        pool_state: &KeyValueEntryRef<'_, LendingPoolState>,
    ) -> Result<&mut ExtendedCollateralPositionData, String> {

        if !self
            .collateral_positions
            .contains_key(&pool_state.pool_res_address)
        {
            self.collateral_positions.insert(
                pool_state.pool_res_address,
                ExtendedCollateralPositionData {
                    pool_res_address: pool_state.pool_res_address,
                    asset_type: pool_state.pool_config.asset_type,
                    liquidation_bonus_rate: pool_state.pool_config.liquidation_bonus_rate,
                    liquidation_threshold: pool_state.liquidation_threshold.clone(),
                    price: pool_state.price,
                    data: PositionData {
                        units: pdec!(0),
                        amount: dec!(0),
                        value: dec!(0),
                        unit_ratio: pdec!(0),
                    },
                },
            );
        };

        Ok(self
            .collateral_positions
            .get_mut(&pool_state.pool_res_address)
            .unwrap())
    }

    fn _get_loan_position(
        &mut self,
        pool_state: &KeyValueEntryRef<'_, LendingPoolState>,
    ) -> Result<&mut ExtendedLoanPositionData, String> {
        if !self
            .loan_positions
            .contains_key(&pool_state.pool_res_address)
        {

            self.loan_positions.insert(
                pool_state.pool_res_address,
                ExtendedLoanPositionData {
                    pool_res_address: pool_state.pool_res_address,

                    price: pool_state.price,

                    asset_type: pool_state.pool_config.asset_type,

                    loan_close_factor: pool_state.pool_config.loan_close_factor,
                    data: PositionData {
                        units: pdec!(0),
                        amount: dec!(0),
                        value: dec!(0),
                        unit_ratio: pdec!(0),
                    },

                    discounted_collateral_value: Decimal::ZERO,
                },
            );
        };
        Ok(self
            .loan_positions
            .get_mut(&pool_state.pool_res_address)
            .unwrap())
    }

    /// Perform update of the data required for the health check, in order that the result is 
    /// coherent with the on-chain state
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn update_health_check_data(&mut self) -> Result<(), String> {
        // Update the collateral positions data and calculate the total solvency value
        self.collateral_positions
            .iter_mut()
            .try_for_each(|(_, extended_collateral)| extended_collateral.update_data())?;

        // Update the loan positions data and calculate the total loan value.
        // We also calculate the  discounted collateral value for each loan position weighted by the loan value
        // let (total_weighted_discounted_collateral_value, total_loan_value, self_loan_value) =

        let (
            total_weighted_discounted_collateral_value,
            total_loan_value,
            self_closable_loan_value,
        ) = self.loan_positions.iter_mut().fold(
            Ok((Decimal::ZERO, Decimal::ZERO, Decimal::ZERO)),
            |result: Result<(Decimal, Decimal, Decimal), String>, (_, extended_loan)| {
                result.and_then(
                    |(
                        mut total_weighted_discounted_collateral_value,
                        mut total_loan_value,
                        mut self_closable_loan_value,
                    )| {
                        extended_loan.update_data(&self.collateral_positions)?;
                        //

                        //

                        let position_total_loan_value =
                            extended_loan.data.value;

                        total_loan_value += position_total_loan_value;

                        total_weighted_discounted_collateral_value +=
                            extended_loan.discounted_collateral_value * position_total_loan_value;

                        //

                        self_closable_loan_value +=
                            extended_loan.data.value * extended_loan.loan_close_factor;

                        Ok((
                            total_weighted_discounted_collateral_value,
                            total_loan_value,
                            self_closable_loan_value
                        ))
                    },
                )
            },
        )?;

        // Calculate total discounted collateral value which is the sum of all discounted collateral value
        let total_discounted_collateral_value = if total_loan_value < ZERO_EPSILON {
            Decimal::ZERO
        } else {
            total_weighted_discounted_collateral_value / total_loan_value
        };

        let total_loan_to_value_ratio: Decimal;

        if total_discounted_collateral_value == Decimal::ZERO {
            // In case the total discounted collateral value is zero,
            // we set the LTV to zero if the total loan value is also zero and to max if the total loan value is not zero
            if total_loan_value < ZERO_EPSILON {
                total_loan_to_value_ratio = Decimal::ZERO;
            } else {
                // This happens when there is no collateral at all
                total_loan_to_value_ratio = Decimal::MAX;
            };
        } else {
            total_loan_to_value_ratio = total_loan_value / total_discounted_collateral_value;
        }

        self.self_closable_loan_value = self_closable_loan_value;

        self.total_loan_value = total_loan_value;
        self.total_loan_to_value_ratio = total_loan_to_value_ratio;

        Ok(())
    }
}
