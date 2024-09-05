use indexmap::IndexMap;
use lending_market::modules::{cdp_data::*, cdp_health_checker::{CDPHealthChecker, ExtendedCollateralPositionData, ExtendedLoanPositionData, PositionData}, liquidation_threshold::LiquidationThreshold};
use scrypto_test::prelude::*;

use crate::helpers::init::TestHelper;


#[test]
fn test_get_collateral_units() {
    let res_address = XRD;
    let mut collaterals = IndexMap::new();
    collaterals.insert(res_address.clone(), pdec!(10));
    let cdp_data = CollaterizedDebtPositionData {
        key_image_url: "url".to_string(),
        name: "name".to_string(),
        description: "description".to_string(),
        cdp_type: CDPType::Standard,
        collaterals,
        loans: IndexMap::new(),
        minted_at: 0,
        updated_at: 0,
        liquidable: None,
    };
    let wrapped_cdp_data = WrappedCDPData {
        cdp_data,
        cdp_id: 1u64.into(),
        collateral_updated: false,
        loan_updated: false,
    };
    assert_eq!(wrapped_cdp_data.get_collateral_units(res_address), pdec!(10));
}

#[test]
fn test_get_loan_unit() {
    let res_address = XRD;

    let mut loans = IndexMap::new();
    loans.insert(res_address.clone(), pdec!(10));
    let cdp_data = CollaterizedDebtPositionData {
        key_image_url: "url".to_string(),
        name: "name".to_string(),
        description: "description".to_string(),
        cdp_type: CDPType::Standard,
        collaterals: IndexMap::new(),
        loans,
        minted_at: 0,
        updated_at: 0,
        liquidable: None,
    };
    let wrapped_cdp_data = WrappedCDPData {
        cdp_data,
        cdp_id: 1u64.into(),
        collateral_updated: false,
        loan_updated: false,
    };
    assert_eq!(wrapped_cdp_data.get_loan_units(res_address), pdec!(10));
}

#[test]
fn test_update_collateral() {
    let res_address = XRD;
    let mut wrapped_cdp_data = WrappedCDPData {
        cdp_data: CollaterizedDebtPositionData {
            key_image_url: "url".to_string(),
            name: "name".to_string(),
            description: "description".to_string(),
            cdp_type: CDPType::Standard,
            collaterals: IndexMap::new(),
            loans: IndexMap::new(),
            minted_at: 0,
            updated_at: 0,
            liquidable: None,
        },
        cdp_id: 1u64.into(),
        collateral_updated: false,
        loan_updated: false,
    };
    wrapped_cdp_data
        .update_collateral(res_address.clone(), pdec!(10))
        .unwrap();
    let mut collaterals = IndexMap::new();
    collaterals.insert(res_address.clone(), pdec!(10));

    assert_eq!(wrapped_cdp_data.cdp_data.collaterals, collaterals);
}

#[test]
fn test_update_loan() {
    let res_address = XRD;
    let mut wrapped_cdp_data = WrappedCDPData {
        cdp_data: CollaterizedDebtPositionData {
            key_image_url: "url".to_string(),
            name: "name".to_string(),
            description: "description".to_string(),
            cdp_type: CDPType::Standard,
            collaterals: IndexMap::new(),
            loans: IndexMap::new(),
            minted_at: 0,
            updated_at: 0,
            liquidable: None,
        },
        cdp_id: 1u64.into(),
        collateral_updated: false,
        loan_updated: false,
    };
    wrapped_cdp_data
        .update_loan(res_address.clone(), pdec!(10))
        .unwrap();
    let mut loans = IndexMap::new();
    loans.insert(res_address.clone(), pdec!(10));
    assert_eq!(wrapped_cdp_data.cdp_data.loans, loans);
}

#[test]
fn test_ltv() {
    let helper = TestHelper::new();

    let mut collateral_positions = IndexMap::new();
    collateral_positions.insert(helper.faucet.btc_resource_address, ExtendedCollateralPositionData {
        asset_type: 0,
        data: PositionData {
            amount: dec!(0.274917797779976621),
            units: pdec!(0.284664061926028008),
            unit_ratio: pdec!(0.93125564662493945763045950808534333),
            value: dec!(0)
        },
        liquidation_bonus_rate: dec!(0),
        liquidation_threshold: LiquidationThreshold {
            identical_resource: Some(dec!(0.7)),
            identical_asset_type: Some(dec!(0.7)),
            default_value: dec!(0.7),
            ..LiquidationThreshold::default()
        },
        pool_res_address: helper.faucet.btc_resource_address,
        price: dec!(2538907.8490715)
    });
    collateral_positions.insert(XRD, ExtendedCollateralPositionData {
        asset_type: 0,
        data: PositionData {
            amount: dec!(463.974351813320909609),
            units: pdec!(463.971851662752889651),
            unit_ratio: pdec!(0.999994611446606372136152486067018011),
            value: dec!(0)
        },
        liquidation_bonus_rate: dec!(0),
        liquidation_threshold: LiquidationThreshold {
            identical_resource: Some(dec!(0.7)),
            identical_asset_type: Some(dec!(0.7)),
            default_value: dec!(0.7),
            ..LiquidationThreshold::default()
        },
        pool_res_address: XRD,
        price: dec!(1)
    });
    let mut  loan_positions = IndexMap::new();
    loan_positions.insert(helper.faucet.eth_resource_address, ExtendedLoanPositionData {
        asset_type: 0,
        data: PositionData {
            amount: dec!(0),
            units: pdec!(4.445769177458039212),
            unit_ratio: pdec!(0.97488430868568543145060850964372638),
            value: dec!(0)
        },
        discounted_collateral_value: dec!(0),
        pool_res_address: helper.faucet.eth_resource_address,
        loan_close_factor: dec!(0),
        price: dec!(107211.80214007)
    });

    let mut health_check = CDPHealthChecker {
        cdp_type: CDPType::Standard,
        collateral_positions,
        loan_positions,
        self_closable_loan_value: dec!(0),
        total_loan_to_value_ratio: dec!(0),
        total_loan_value: dec!(0)
    };

    health_check.check_cdp().unwrap();

    assert_eq!(dec!(0.899431648740609894), health_check.total_loan_to_value_ratio)
}
