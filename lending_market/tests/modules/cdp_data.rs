use lending_market::modules::cdp_data::*;
use scrypto_test::prelude::*;


#[test]
fn test_get_collateral_units() {
    let res_address = XRD;
    let mut collaterals = IndexMap::new();
    collaterals.insert(res_address.clone(), dec!(10));
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
        cdp_type_updated: false,
        collateral_updated: false,
        loan_updated: false,
    };
    assert_eq!(wrapped_cdp_data.get_collateral_units(res_address), dec!(10));
}

#[test]
fn test_get_loan_unit() {
    let res_address = XRD;

    let mut loans = IndexMap::new();
    loans.insert(res_address.clone(), dec!(10));
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
        cdp_type_updated: false,
        collateral_updated: false,
        loan_updated: false,
    };
    assert_eq!(wrapped_cdp_data.get_loan_unit(res_address), dec!(10));
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
        cdp_type_updated: false,
        collateral_updated: false,
        loan_updated: false,
    };
    wrapped_cdp_data
        .update_collateral(res_address.clone(), dec!(10))
        .unwrap();
    let mut collaterals = IndexMap::new();
    collaterals.insert(res_address.clone(), dec!(10));

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
        cdp_type_updated: false,
        collateral_updated: false,
        loan_updated: false,
    };
    wrapped_cdp_data
        .update_loan(res_address.clone(), dec!(10))
        .unwrap();
    let mut loans = IndexMap::new();
    loans.insert(res_address.clone(), dec!(10));
    assert_eq!(wrapped_cdp_data.cdp_data.loans, loans);
}
