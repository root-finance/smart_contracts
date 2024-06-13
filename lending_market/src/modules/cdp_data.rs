use scrypto::prelude::*;

use super::cdp_health_checker::ZERO_EPSILON;

#[derive(ScryptoSbor)]
pub enum CDPUpdatedEvenType {
    AddCollateral,
    RemoveCollateral,
    Borrow,
    Repay,
    Liquidate,
    Refinance,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct CDPUpdatedEvent {
    pub cdp_id: NonFungibleLocalId,
    pub event_type: CDPUpdatedEvenType,
}

#[derive(ScryptoSbor, Clone, Debug)]
pub struct CDPLiquidable {
    pub cdp_data: CollaterizedDebtPositionData,
    pub cdp_id: NonFungibleLocalId,
}

#[derive(ScryptoSbor, ScryptoEvent, Debug)]
pub struct CDPLiquidableEvent {
    pub cdps: Vec<CDPLiquidable>,
}

#[derive(ScryptoSbor, Clone, PartialEq, Debug)]
pub enum CDPType {
    Standard
}

#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct CollaterizedDebtPositionData {
    #[mutable]
    pub key_image_url: String,

    #[mutable]
    pub name: String,

    #[mutable]
    pub description: String,

    // #[immutable]
    pub minted_at: i64,

    #[mutable]
    pub updated_at: i64,

    #[mutable]
    pub cdp_type: CDPType,

    #[mutable]
    pub collaterals: IndexMap<ResourceAddress, PreciseDecimal>,

    #[mutable]
    pub loans: IndexMap<ResourceAddress, PreciseDecimal>,

    #[mutable]
    pub liquidated: IndexMap<ResourceAddress, PreciseDecimal>,

    #[mutable]
    pub liquidable: Option<Decimal>
}

#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct WrappedCDPData {
    pub cdp_data: CollaterizedDebtPositionData,
    pub cdp_id: NonFungibleLocalId,
    pub cdp_type_updated: bool,
    pub collateral_updated: bool,
    pub loan_updated: bool,
    pub liquidated_updated: bool,
}

impl WrappedCDPData {
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

    pub fn get_type(&self) -> CDPType {
        self.cdp_data.cdp_type.clone()
    }

    pub fn get_data(&self) -> CollaterizedDebtPositionData {
        self.cdp_data.clone()
    }

    fn get_units(map: &IndexMap<ResourceAddress, PreciseDecimal>, key: ResourceAddress) -> PreciseDecimal {
        map.get(&key).copied().unwrap_or(PreciseDecimal::ZERO)
    }

    pub fn get_collateral_units(&self, collateral: ResourceAddress) -> PreciseDecimal {
        Self::get_units(&self.cdp_data.collaterals, collateral)
    }

    pub fn get_loan_unit(&self, loan: ResourceAddress) -> PreciseDecimal {
        Self::get_units(&self.cdp_data.loans, loan)
    }

    //

    pub fn update_cdp_type(&mut self, cdp_type: CDPType) {
        self.cdp_data.cdp_type = cdp_type;
        self.cdp_type_updated = true;
    }

    pub fn update_collateral(
        &mut self,
        res_address: ResourceAddress,
        units: PreciseDecimal,
    ) -> Result<(), String> {
        let result = Self::update_map(&mut self.cdp_data.collaterals, res_address, units);
        self.collateral_updated = true;
        result
    }

    pub fn update_loan(
        &mut self,
        res_address: ResourceAddress,
        units: PreciseDecimal,
    ) -> Result<(), String> {
        let result = Self::update_map(&mut self.cdp_data.loans, res_address, units);
        self.loan_updated = true;
        result
    }

    pub fn on_liquidation(
        &mut self,
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
