use scrypto::prelude::*;

use super::cdp_health_checker::ZERO_EPSILON;

/// Type of the event launched in case of CDP update
#[derive(ScryptoSbor)]
pub enum CDPUpdatedEvenType {
    /// Signals collateral was added
    AddCollateral,
    /// Signals collateral was removed
    RemoveCollateral,
    /// Signals a borrow happened
    Borrow,
    /// Signals repay of a borrowed amount happened
    Repay,
    /// Signals CDP liquidation
    Liquidate,
    /// (UNUSED)
    Refinance,
}

/// Event launched in case of CDP update
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct CDPUpdatedEvent {
    /// id of the updated CDP
    pub cdp_id: NonFungibleLocalId,
    /// type of the event
    pub event_type: CDPUpdatedEvenType,
}

/// Model of a liquidable CDP
#[derive(ScryptoSbor, Clone, Debug)]
pub struct CDPLiquidable {
    /// Up-to-date data that shows the CDP as liquidable
    pub cdp_data: CollaterizedDebtPositionData,
    /// id of the liquidable CDP
    pub cdp_id: NonFungibleLocalId,
}

/// Event that signals the presence of CDPs to liquidate
#[derive(ScryptoSbor, ScryptoEvent, Debug)]
pub struct CDPLiquidableEvent {
    /// List of CDPs to liquidate
    pub cdps: Vec<CDPLiquidable>,
}

/// Type of CDP
#[derive(ScryptoSbor, Clone, PartialEq, Debug)]
pub enum CDPType {
    /// A CDP where an user is directly responsible of his borrows and collaterals, subject to liquidation
    Standard
}

/// Data describing the CDP
#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct CollaterizedDebtPositionData {
    /// Image to display when exploring Radix transactions
    #[mutable]
    pub key_image_url: String,

    /// Name of the CDP
    #[mutable]
    pub name: String,

    /// Textual description of the CDP
    #[mutable]
    pub description: String,

    /// Immutable timestamp of CDP minting
    pub minted_at: i64,

    /// Timestamp of CDP update
    #[mutable]
    pub updated_at: i64,

    /// Type of the CDP
    #[mutable]
    pub cdp_type: CDPType,

    /// Map of collateral values, having the asset as key and the unit amount as value.
    /// Here, `PreciseDecimal` helps in keeping precision in computations
    /// even if the actual amount will require to be expressed as `Decimal`
    #[mutable]
    pub collaterals: IndexMap<ResourceAddress, PreciseDecimal>,

    /// Map of loaned values, having the asset as key and the unit amount as value.
    /// Here, `PreciseDecimal` helps in keeping precision in computation
    /// even if the actual amount will require to be expressed as `Decimal`
    #[mutable]
    pub loans: IndexMap<ResourceAddress, PreciseDecimal>,

    /// Map of liquidated values, having the asset as key and the unit amount as value.
    /// Here, `PreciseDecimal` helps in keeping precision in computation
    /// even if the actual amount will require to be expressed as `Decimal`
    #[mutable]
    pub liquidated: IndexMap<ResourceAddress, PreciseDecimal>,

    /// The maximum amount of liquidable value for this collateralized debt position
    #[mutable]
    pub liquidable: Option<Decimal>
}

/// Wrapper of the `CollaterizedDebtPositionData` that keeps trace of the modifications,
/// so that only the required changes are written on chain, reducing transaction cost and fees.
#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct WrappedCDPData {
    /// The wrapped CDP data
    pub cdp_data: CollaterizedDebtPositionData,
    /// The wrapped CDP id
    pub cdp_id: NonFungibleLocalId,
    /// (UNUSED)
    pub cdp_type_updated: bool,
    /// Indicator of an update in the collateral values
    pub collateral_updated: bool,
    /// Indicator of an update in the loaned values
    pub loan_updated: bool,
    /// Indicator of an update in the liquidated values
    pub liquidated_updated: bool,
}

impl WrappedCDPData {
    /// Constructor
    /// 
    /// *Params*
    /// - `res_manager``: The CDP resource manager
    /// - `cdp_id`: The id to use for the newly created CDP
    /// 
    /// *Output*
    /// A new `WrappedCDPData` 
    pub fn new(res_manager: &ResourceManager, cdp_id: &NonFungibleLocalId) -> WrappedCDPData {
        let cdp_data = res_manager.get_non_fungible_data(cdp_id);
        WrappedCDPData {
            cdp_id: cdp_id.clone(),
            cdp_data,
            cdp_type_updated: false,
            collateral_updated: false,
            loan_updated: false,
            liquidated_updated: false
        }
    }

    /// Getter of `CDPType`
    pub fn get_type(&self) -> CDPType {
        self.cdp_data.cdp_type.clone()
    }

    /// Getter of `CollaterizedDebtPositionData`
    pub fn get_data(&self) -> CollaterizedDebtPositionData {
        self.cdp_data.clone()
    }

    /// Getter of the collateral units amount
    /// 
    /// *Params*
    ///  - `collateral`: The resource address of  the collateral units to get
    pub fn get_collateral_units(&self, collateral: ResourceAddress) -> PreciseDecimal {
        Self::get_units(&self.cdp_data.collaterals, collateral)
    }

    /// Getter of the loan units amount
    pub fn get_loan_units(&self, loan: ResourceAddress) -> PreciseDecimal {
        Self::get_units(&self.cdp_data.loans, loan)
    }

    /// (UNUSED)
    pub fn update_cdp_type(&mut self, cdp_type: CDPType) {
        self.cdp_data.cdp_type = cdp_type;
        self.cdp_type_updated = true;
    }

    /// Update the CDP collateral values
    /// 
    /// *Params*
    /// - `res_address``: The resource to update among the collateralized values
    /// - `units`: The amount of units to set
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn update_collateral(
        &mut self,
        res_address: ResourceAddress,
        units: PreciseDecimal,
    ) -> Result<(), String> {
        let result = Self::update_map(&mut self.cdp_data.collaterals, res_address, units);
        self.collateral_updated = true;
        result
    }

    /// Update the CDP loaned values
    /// 
    /// *Params*
    /// - `res_address``: The resource to update among the collateralized values
    /// - `units`: The amount of units to set
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn update_loan(
        &mut self,
        res_address: ResourceAddress,
        units: PreciseDecimal,
    ) -> Result<(), String> {
        let result = Self::update_map(&mut self.cdp_data.loans, res_address, units);
        self.loan_updated = true;
        result
    }

    /// Cleanup tasks to perform upon liquidation.
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn on_liquidation(
        &mut self
    ) -> Result<(), String> {
        if self.cdp_data.collaterals.len() == 0 && self.cdp_data.loans.len() > 0 {
            for (res_address, units) in &self.cdp_data.loans {
                Self::update_map(&mut self.cdp_data.liquidated, *res_address, *units).map_err(|err| format!("Error updating cdp loan to liquidated: {err}"))?;
            }
            self.cdp_data.loans.clear();
            self.liquidated_updated = true;
            self.loan_updated = true;
        }
        Ok(())
    }

    /// Save the CDP, updating only the required fields
    /// 
    /// *Params*
    /// - `res_manager``: The CDP resource manager
    /// - `max_cdp_position`: The configured amount of positions for the CDP
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn save_cdp(
        &self,
        res_manager: &ResourceManager,
        max_cdp_position: u8,
    ) -> Result<(), String> {
        let mut updated = false;

        if self.cdp_type_updated {
            res_manager.update_non_fungible_data(
                &self.cdp_id,
                "cdp_type",
                self.cdp_data.cdp_type.clone(),
            );
            updated = true;
        }

        if self.collateral_updated {
            res_manager.update_non_fungible_data(
                &self.cdp_id,
                "collaterals",
                self.cdp_data.collaterals.clone(),
            );
            updated = true;
        }

        if self.loan_updated {
            res_manager.update_non_fungible_data(
                &self.cdp_id,
                "loans",
                self.cdp_data.loans.clone(),
            );
            updated = true;
        }

        if self.liquidated_updated {
            res_manager.update_non_fungible_data(
                &self.cdp_id,
                "liquidated",
                self.cdp_data.liquidated.clone(),
            );
            updated = true;
        }

        if self.cdp_data.liquidable.is_some() {
            res_manager.update_non_fungible_data(
                &self.cdp_id,
                "liquidable",
                self.cdp_data.liquidable.clone(),
            );
            updated = true;
        }

        if updated {
            let position_count = self.cdp_data.collaterals.len()
                + self.cdp_data.loans.len();

            assert!(position_count as u8 <= max_cdp_position);

            res_manager.update_non_fungible_data(
                &self.cdp_id,
                "updated_at",
                Clock::current_time(TimePrecision::Second).seconds_since_unix_epoch,
            );
        }

        // Logger::debug(format!("Save CDP: {:#?}", self)); 

        Ok(())
    }

    // local methods

    fn get_units(map: &IndexMap<ResourceAddress, PreciseDecimal>, key: ResourceAddress) -> PreciseDecimal {
        map.get(&key).copied().unwrap_or(PreciseDecimal::ZERO)
    }

    fn update_map(
        map: &mut IndexMap<ResourceAddress, PreciseDecimal>,
        key: ResourceAddress,
        units: PreciseDecimal,
    ) -> Result<(), String> {
        if units == PreciseDecimal::ZERO {
            return Ok(());
        }

        if let Some(entry) = map.get_mut(&key) {
            *entry += units;

            if *entry < ZERO_EPSILON.into() {
                map.remove(&key);
            }
        } else {
            map.insert(key, units);
        }

        Ok(())
    }
}
