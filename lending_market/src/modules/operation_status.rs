use scrypto::prelude::*;

/// A semaphore to the pool operating status
#[derive(ScryptoSbor, Debug, Clone, Default)]
pub struct OperatingStatusValue {
    enabled: bool,
    set_by_admin: bool,
}

/// Enumeration of the possible operations on the pool
#[derive(ScryptoSbor, Debug, Clone)]
pub enum OperatingService {
    Contribute,
    Redeem,
    AddCollateral,
    RemoveCollateral,
    Borrow,
    Repay,
    Liquidation,
}

/// The operating status of the pool
#[derive(ScryptoSbor, Default)]
pub struct OperatingStatus {
    pub is_contribute_enabled: OperatingStatusValue,
    pub is_redeem_enabled: OperatingStatusValue,
    pub is_deposit_enabled: OperatingStatusValue,
    pub is_withdraw_enabled: OperatingStatusValue,
    pub is_borrow_enabled: OperatingStatusValue,
    pub is_repay_enabled: OperatingStatusValue,
    pub is_liquidate_enabled: OperatingStatusValue,
}
//
impl OperatingStatus {
    /// Constructor
    /// 
    /// *Output*
    /// An `OperatingStatus`
    pub fn new() -> Self {
        Self {
            is_contribute_enabled: OperatingStatusValue {
                enabled: true,
                set_by_admin: false,
            },
            is_redeem_enabled: OperatingStatusValue {
                enabled: true,
                set_by_admin: false,
            },
            is_deposit_enabled: OperatingStatusValue {
                enabled: true,
                set_by_admin: false,
            },
            is_withdraw_enabled: OperatingStatusValue {
                enabled: true,
                set_by_admin: false,
            },
            is_borrow_enabled: OperatingStatusValue {
                enabled: true,
                set_by_admin: false,
            },
            is_repay_enabled: OperatingStatusValue {
                enabled: true,
                set_by_admin: false,
            },
            is_liquidate_enabled: OperatingStatusValue {
                enabled: true,
                set_by_admin: false,
            },
        }
    }

    /// Update the operating status
    /// 
    /// *Params*
    /// - `operating_status`: The current pool operation
    /// - `enabled`: Whether to enable or disable the operation
    /// - `set_by_admin`: Whether the operating status change is performed by an admin
    /// 
    /// *Errors*
    /// - If update of the internal state fails
    pub fn update(
        &mut self,
        operating_status: OperatingService,
        enabled: bool,
        set_by_admin: bool,
    ) -> Result<(), String> {
        let field = match operating_status {
            OperatingService::Contribute => &mut self.is_contribute_enabled,
            OperatingService::Redeem => &mut self.is_redeem_enabled,
            OperatingService::AddCollateral => &mut self.is_deposit_enabled,
            OperatingService::RemoveCollateral => &mut self.is_withdraw_enabled,
            OperatingService::Borrow => &mut self.is_borrow_enabled,
            OperatingService::Repay => &mut self.is_repay_enabled,
            OperatingService::Liquidation => &mut self.is_liquidate_enabled,
        };

        if field.set_by_admin && !set_by_admin {
            return Err(format!(
                "The operating status for {:?} is set by admin",
                operating_status
            ));
        }

        field.enabled = enabled;
        field.set_by_admin = set_by_admin;

        Ok(())
    }

    /// Getter of the currentyl enabled operation
    /// 
    /// *Params*
    /// - `value`: The operation to check
    pub fn check(&self, value: OperatingService) -> bool {
        match value {
            OperatingService::Contribute => self.is_contribute_enabled.enabled,
            OperatingService::Redeem => self.is_redeem_enabled.enabled,
            OperatingService::AddCollateral => self.is_deposit_enabled.enabled,
            OperatingService::RemoveCollateral => self.is_withdraw_enabled.enabled,
            OperatingService::Borrow => self.is_borrow_enabled.enabled,
            OperatingService::Repay => self.is_repay_enabled.enabled,
            OperatingService::Liquidation => self.is_liquidate_enabled.enabled,
        }
    }
}
