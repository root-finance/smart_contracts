use crate::modules::utils::is_valid_rate;
use scrypto::prelude::*;

/// Input to update liquidation threshold
#[derive(ScryptoSbor)]
pub enum UpdateLiquidationThresholdInput {
    DefaultValue(Decimal),
    IdenticalResource(Option<Decimal>),
    IdenticalAssetType(Option<Decimal>),
    ResourceEntry(ResourceAddress, Option<Decimal>),
    AssetTypeEntry(u8, Option<Decimal>),
}

/// Liquidation threshold configuration
#[derive(ScryptoSbor, Clone, Debug)]
pub struct LiquidationThreshold {
    /// Threshold for position having same asset in loan and as collateral
    pub identical_resource: Option<Decimal>,
    /// Threshold for position having same asset type in loan and as collateral
    pub identical_asset_type: Option<Decimal>,
    /// Threshold for a specific resource
    pub resource: IndexMap<ResourceAddress, Decimal>,
    /// Threshold for a specific asset type
    pub asset_type: IndexMap<u8, Decimal>,
    /// Default threshold
    pub default_value: Decimal,
}
impl LiquidationThreshold {
    /// Perform a check on the liquidation threshold configuration
    /// 
    /// *Error*
    /// - If the configuration is invalid
    pub fn check(&self) -> Result<(), String> {
        if !is_valid_rate(self.default_value) {
            return Err("Invalid liquidation threshold default value".into());
        }

        if self.identical_resource.is_some() && !is_valid_rate(self.identical_resource.unwrap()) {
            return Err("Invalid liquidation threshold identical resource".into());
        };

        if self.identical_asset_type.is_some() && !is_valid_rate(self.identical_asset_type.unwrap())
        {
            return Err("Invalid liquidation threshold identical asset type".into());
        };

        for (_, threshold) in self.resource.iter() {
            if !is_valid_rate(*threshold) {
                return Err("Invalid liquidation threshold resource entry".into());
            }
        }

        for (_, threshold) in self.asset_type.iter() {
            if !is_valid_rate(*threshold) {
                return Err("Invalid liquidation threshold asset type entry".into());
            }
        }

        Ok(())
    }

    /// Getter the liquidation threshold according to params.
    /// 
    /// *Params*
    /// - `collateral_res_address`: The collateral resource address
    /// - `collateral_asset_type`: The collateral asset type
    /// - `loan_res_address`: The loan resource address
    /// - `loan_asset_type`: The loan asset type
    pub fn get_ratio(
        &self,
        collateral_res_address: ResourceAddress,
        collateral_asset_type: u8,
        loan_res_address: ResourceAddress,
        loan_asset_type: u8,
    ) -> Decimal {
        if loan_res_address == collateral_res_address {
            if self.identical_resource.is_some() {
                return self.identical_resource.unwrap();
            } else {
                return 0.into();
            }
        }

        if self.identical_asset_type.is_some() && loan_asset_type == collateral_asset_type {
            return self.identical_asset_type.unwrap();
        }

        self.resource
            .get(&loan_res_address)
            .copied()
            .unwrap_or_else(|| {
                self.asset_type
                    .get(&loan_asset_type)
                    .copied()
                    .unwrap_or(self.default_value)
            })
    }

    /// Update the liquidation threshold configuration
    /// 
    /// *Params*
    /// - `UpdateLiquidationThresholdInput`: The input structure for the update
    /// 
    /// *Errors*
    /// - If update of the internal state fails
    pub fn update_liquidation_threshold(
        &mut self,
        value: UpdateLiquidationThresholdInput,
    ) -> Result<(), String> {
        match value {
            UpdateLiquidationThresholdInput::DefaultValue(value) => {
                self.default_value = value;
            }

            UpdateLiquidationThresholdInput::IdenticalResource(value) => {
                self.identical_resource = value;
            }

            UpdateLiquidationThresholdInput::IdenticalAssetType(value) => {
                self.identical_asset_type = value;
            }

            UpdateLiquidationThresholdInput::AssetTypeEntry(asset_type, value) => {
                self.set_asset_type_entry(asset_type, value);
            }

            UpdateLiquidationThresholdInput::ResourceEntry(res_address, value) => {
                self.set_resource_entry(res_address, value);
            }
        }

        self.check()?;

        Ok(())
    }

    fn set_resource_entry(&mut self, resource: ResourceAddress, threshold: Option<Decimal>) {
        if let Some(threshold) = threshold {
            self.resource.insert(resource, threshold);
        } else {
            self.resource.remove(&resource);
        }
    }

    fn set_asset_type_entry(&mut self, asset_type: u8, threshold: Option<Decimal>) {
        if let Some(threshold) = threshold {
            self.asset_type.insert(asset_type, threshold);
        } else {
            self.asset_type.remove(&asset_type);
        }
    }
}
