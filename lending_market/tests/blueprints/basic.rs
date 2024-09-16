use crate::helpers::{init::{find_event_in_result, TestHelper}, methods::*};
use lending_market::modules::{cdp_data::CDPLiquidableEvent, cdp_health_checker::ZERO_EPSILON};
use radix_engine_interface::prelude::*;
use scrypto_unit::*;
use std::path::Path;

#[test]
fn test_deposit_withdraw_borrow_repay() {
    let mut helper = TestHelper::new();

    const T2022: i64 = 1640998800;
    const T2023: i64 = 1672534800;
    let usd = helper.faucet.usdc_resource_address;
    let btc = helper.faucet.btc_resource_address;

    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(1), T2022);
    admin_update_price(&mut helper, 1u64, usd, dec!(25)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1300000)).expect_commit_success();

    // SETUP A LP PROVIDER

    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, XRD),
        dec!(100_000)
    );

    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(50_000), usd) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, usd),
        dec!(2_000)
    );

    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(2_000))
        .expect_commit_success();

    // SET UP A BORROWER

    let (borrower_key, _, borrower_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(borrower_account);
    helper.test_runner.load_account_from_faucet(borrower_account);

    get_resource(&mut helper, borrower_key, borrower_account, dec!(20_000), btc) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(borrower_account, btc),
        dec!(0.0153846153846)
    );

    market_create_cdp(
        &mut helper,
        borrower_key,
        borrower_account,
        vec![
            (XRD, dec!(10_000)),
            (btc, dec!(0.0001))
        ],
    ) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(borrower_account, helper.market.cdp_resource_address),
        dec!(1)
    );

    // BORROW

    // market_deposit(
    //     &mut helper,
    //     borrower_key,
    //     borrower_account,
    //     1u64,
    //     XRD,
    //     dec!(10_000),
    // )
    // .expect_commit_success();

    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        1u64,
        usd,
        dec!(1000),
    )
        .expect_commit_failure();

    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        1u64,
        usd,
        dec!(100),
    )
        .expect_commit_success();

    market_remove_collateral(
        &mut helper,
        borrower_key,
        borrower_account,
        1u64,
        XRD,
        dec!(10_000),
        false,
    )
        .expect_commit_failure();

    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(2), T2023);
    admin_update_price(&mut helper, 1u64, usd, dec!(25)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1300000)).expect_commit_success();

    // borrower investments returned him 2 usd, we buy them from faucet
    helper.test_runner.load_account_from_faucet(borrower_account);
    get_resource(&mut helper, borrower_key, borrower_account, dec!(50), usd) //
        .expect_commit_success();

    market_repay(
        &mut helper,
        borrower_key,
        borrower_account,
        1u64,
        usd,
        dec!(100.250312164987776789),
    )
        .expect_commit_success();

    market_remove_collateral(
        &mut helper,
        borrower_key,
        borrower_account,
        1u64,
        XRD,
        dec!(10_000),
        false,
    )
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(borrower_account, XRD),
        dec!(19_950)
    );
    assert_eq!(
        helper
            .test_runner
            .get_component_balance(borrower_account, usd),
        dec!(1.999747667645468422)
    );

    // REDEEM

    let usd_pu = helper.market.pools.get(&usd).unwrap().clone().1;

    market_redeem(
        &mut helper,
        lp_user_key,
        lp_user_account,
        usd_pu,
        dec!(2_100),
    ) //
        .expect_commit_failure();

    market_redeem(
        &mut helper,
        lp_user_key,
        lp_user_account,
        usd_pu,
        dec!(2_000),
    ) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, usd),
        dec!(2000.000199967145115916)
    );
}

#[test]
fn test_create_pool_package_address() {
    let mut test_runner = TestRunnerBuilder::new().build();
    let _pool_package_address =
        test_runner.compile_and_publish(Path::new("../single_resource_pool"));
    println!("{:?}\n", _pool_package_address);
}

#[test]
fn test_liquidation() {
    let mut helper = TestHelper::new();
    let usd = helper.faucet.usdc_resource_address;
    let btc = helper.faucet.btc_resource_address;
    let eth = helper.faucet.eth_resource_address;
    let lsu = helper.faucet.lsu_resource_address;
    let hug = helper.faucet.hug_resource_address;
    let usdt = helper.faucet.usdt_resource_address;

    const T2024: i64 = 1704067200;
    const T6_MONTHS: i64 = 15778476000;

    helper.test_runner.load_account_from_faucet(helper.owner_account_address);
    helper.test_runner.load_account_from_faucet(helper.owner_account_address);

    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(1), T2024);
    admin_update_price(&mut helper, 1u64, usd, dec!(25)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1300000)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, eth, dec!(72500)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, lsu, dec!(1)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, hug, dec!(0.001)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, usdt, dec!(25)).expect_commit_success();

    // SET UP A LP PROVIDER
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(25_000), usd) //
        .expect_commit_success();
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(10_000), btc) //
        .expect_commit_success();
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(20_000), eth) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, usd),
        dec!(1_000)
    );
    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, btc),
        dec!(0.0076923076923)
    );
    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, eth),
        dec!(0.2758620689655)
    );

    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(800))
        .expect_commit_success();
    market_contribute(&mut helper, lp_user_key, lp_user_account, btc, dec!(0.005))
        .expect_commit_success();
    market_contribute(&mut helper, lp_user_key, lp_user_account, eth, dec!(0.2))
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, usd),
        dec!(200)
    );
    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, btc),
        dec!(0.0026923076923)
    );
    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, eth),
        dec!(0.0758620689655)
    );


    // SET UP A BORROWER
    let (borrower_key, _, borrower_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(borrower_account);
    helper.test_runner.load_account_from_faucet(borrower_account);
    helper.test_runner.load_account_from_faucet(borrower_account);
    helper.test_runner.load_account_from_faucet(borrower_account);
    helper.test_runner.load_account_from_faucet(borrower_account);

    //Create CDP WITH XRD AS Collateral
    market_create_cdp(
        &mut helper,
        borrower_key,
        borrower_account,
        vec![(XRD, dec!(20_000))],
    ) //
        .expect_commit_success();

    let cdp_id: u64 = 1;
    // Borrow USDC, BTC and ETH
    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        cdp_id,
        usd,
        dec!(200),
    )
        .expect_commit_success();

    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        cdp_id,
        btc,
        dec!(0.004),
    )
        .expect_commit_success();

    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        cdp_id,
        eth,
        dec!(0.05),
    )
        .expect_commit_success();

    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");
    assert!(event.cdps.is_empty());


    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(1), T2024 + T6_MONTHS);
    admin_update_price(&mut helper, 1u64, usd, dec!(25)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1500000)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, eth, dec!(72500)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, lsu, dec!(1)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, hug, dec!(0.001)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, usdt, dec!(25)).expect_commit_success();

    market_update_pool_state(&mut helper, usd).expect_commit_success();
    market_update_pool_state(&mut helper, btc).expect_commit_success();

    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");

    assert!(!event.cdps.is_empty());
    assert_eq!(pdec!(20000), *event.cdps[0].cdp_data.collaterals.get(&XRD).unwrap());
    assert_eq!(pdec!(200), *event.cdps[0].cdp_data.loans.get(&usd).unwrap());
    assert_eq!(pdec!(0.004), *event.cdps[0].cdp_data.loans.get(&btc).unwrap());
    assert_eq!(pdec!(0.05), *event.cdps[0].cdp_data.loans.get(&eth).unwrap());


    // SET UP LIQUIDATOR
    let auth = rule!(require(NonFungibleGlobalId::from_public_key(&helper.owner_public_key)));
    let (liquidator_user_key, liquidator_user_account) = (helper.owner_public_key, helper.test_runner.new_account_advanced(OwnerRole::Fixed(auth)));
    admin_send_liquidator_badge(&mut helper, 1, liquidator_user_account)
        .expect_commit_success();
    helper
        .test_runner
        .load_account_from_faucet(liquidator_user_account);
    helper
        .test_runner
        .load_account_from_faucet(liquidator_user_account);
    helper
        .test_runner
        .load_account_from_faucet(liquidator_user_account);
    helper
        .test_runner
        .load_account_from_faucet(liquidator_user_account);
    helper
        .test_runner
        .load_account_from_faucet(liquidator_user_account);

    // SWAP TO Cover debt
    swap(
        &mut helper,
        liquidator_user_account,
        liquidator_user_key,
        dec!(35000),
        XRD,
        usd,
    ).expect_commit_success();
    swap(
        &mut helper,
        liquidator_user_account,
        liquidator_user_key,
        dec!(15000),
        XRD,
        btc,
    ).expect_commit_success();
    swap(
        &mut helper,
        liquidator_user_account,
        liquidator_user_key,
        dec!(10000),
        XRD,
        eth,
    ).expect_commit_success();

    let xrd_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, XRD);

    let usd_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, usd);

    let btc_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, btc);

    let eth_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, eth);

    let mut requested_collaterals: Vec<ResourceAddress> = Vec::new();
    requested_collaterals.push(XRD);

    // START LIQUIDATION
    check_cdp_for_liquidation(&mut helper, liquidator_user_key, cdp_id).expect_commit_success();

    let payments: Vec<(ResourceAddress, Decimal)> = vec![(usd, dec!(100)), (btc, dec!(0.1)), (eth, dec!(0.1))];
    market_liquidation(
        &mut helper,
        liquidator_user_key,
        liquidator_user_account,
        1,
        cdp_id,
        payments,
        None,
        requested_collaterals.clone(),
    ).expect_commit_failure();

    let payments: Vec<(ResourceAddress, Decimal)> = vec![(usd, dec!(202)), (btc, dec!(0.0099)), (eth, dec!(0.062))];
    market_liquidation(
        &mut helper,
        liquidator_user_key,
        liquidator_user_account,
        1,
        cdp_id,
        payments,
        None,
        requested_collaterals,
    ).expect_commit_success();

    assert_eq!(dec!(18400), helper
        .test_runner
        .get_component_balance(liquidator_user_account, XRD) - xrd_balance);

    assert_eq!(dec!(-201.25625), helper
        .test_runner
        .get_component_balance(liquidator_user_account, usd) - usd_balance);

    assert_eq!(dec!(-0.008912395833333333), helper
        .test_runner
        .get_component_balance(liquidator_user_account, btc) - btc_balance);

    assert_eq!(dec!(0), helper
        .test_runner
        .get_component_balance(liquidator_user_account, eth) - eth_balance);

    market_update_pool_state(&mut helper, btc).expect_commit_success();
    market_update_pool_state(&mut helper, usd).expect_commit_success();
    market_update_pool_state(&mut helper, eth).expect_commit_success();
    market_update_pool_state(&mut helper, XRD).expect_commit_success();


    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");

    assert!(event.cdps.is_empty());

    // ENTERS A NEW BORROWER
    let (borrower2_key, _, borrower2_account) = helper.test_runner.new_allocated_account();
    helper
        .test_runner
        .load_account_from_faucet(borrower_account);

    //Create CDP WITH 10_000 XRD AS Collateral <=> 100 USD
    market_create_cdp(
        &mut helper,
        borrower2_key,
        borrower2_account,
        vec![(XRD, dec!(10_000))],
    ) //
        .expect_commit_success();

    let cdp_id: u64 = 2;
    // // Borrow 300$  Of USD
    market_borrow(
        &mut helper,
        borrower2_key,
        borrower2_account,
        cdp_id,
        usd,
        dec!(100),
    )
        .expect_commit_success();

    assert!(event.cdps.is_empty());
}

#[test]
fn test_partial_liquidation() {
    let mut helper = TestHelper::new();
    let usd = helper.faucet.usdc_resource_address;

    const T2024: i64 = 1704067200;

    helper.test_runner.load_account_from_faucet(helper.owner_account_address);
    helper.test_runner.load_account_from_faucet(helper.owner_account_address);

    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(1), T2024);
    admin_update_price(&mut helper, 1u64, usd, dec!(25)).expect_commit_success();

    // SET UP A LP PROVIDER
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(lp_user_account);
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(10000), usd) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, usd),
        dec!(400)
    );

    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(150))
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, usd),
        dec!(250)
    );

    // SET UP A BORROWER
    let (borrower_key, _, borrower_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(borrower_account);

    //Create CDP WITH XRD AS Collateral
    market_create_cdp(
        &mut helper,
        borrower_key,
        borrower_account,
        vec![(XRD, dec!(3000))],
    ) //
        .expect_commit_success();

    let cdp_id: u64 = 1;
    // Borrow USDC, BTC and ETH
    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        cdp_id,
        usd,
        dec!(76),
    )
        .expect_commit_success();


    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");
    assert!(event.cdps.is_empty());

    // SET UP LIQUIDATOR
    let auth = rule!(require(NonFungibleGlobalId::from_public_key(&helper.owner_public_key)));
    let (liquidator_user_key, liquidator_user_account) = (helper.owner_public_key, helper.test_runner.new_account_advanced(OwnerRole::Fixed(auth)));
    admin_send_liquidator_badge(&mut helper, 1, liquidator_user_account)
        .expect_commit_success();
    helper
        .test_runner
        .load_account_from_faucet(liquidator_user_account);


    // SWAP TO Cover debt
    swap(
        &mut helper,
        liquidator_user_account,
        liquidator_user_key,
        dec!(2000),
        XRD,
        usd,
    ).expect_commit_success();

    // PREPARE LIQUIDATION
    admin_update_price(&mut helper, 1u64, usd, dec!(28)).expect_commit_success();
    market_update_pool_state(&mut helper, usd).expect_commit_success();

    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");

    assert!(!event.cdps.is_empty());
    assert_eq!(pdec!(3000), *event.cdps[0].cdp_data.collaterals.get(&XRD).unwrap());
    assert_eq!(pdec!(76), *event.cdps[0].cdp_data.loans.get(&usd).unwrap());

    // START LIQUIDATION
    check_cdp_for_liquidation(&mut helper, liquidator_user_key, cdp_id).expect_commit_success();

    let mut requested_collaterals: Vec<ResourceAddress> = Vec::new();
    requested_collaterals.push(XRD);

    let xrd_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, XRD);

    let usd_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, usd);

    let payments: Vec<(ResourceAddress, Decimal)> = vec![(usd, dec!(76))];
    market_liquidation(
        &mut helper,
        liquidator_user_key,
        liquidator_user_account,
        1,
        cdp_id,
        payments,
        Some(dec!(0.4) * dec!(3000)),
        requested_collaterals.clone(),
    ).expect_commit_success();

    assert_eq!(dec!(1104), helper
        .test_runner
        .get_component_balance(liquidator_user_account, XRD) - xrd_balance);

    assert_eq!(dec!(-39.428571428571428571), helper
        .test_runner
        .get_component_balance(liquidator_user_account, usd) - usd_balance);


    market_update_pool_state(&mut helper, usd).expect_commit_success();
    market_update_pool_state(&mut helper, XRD).expect_commit_success();


    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");

    assert!(event.cdps.is_empty());
}


#[test]
fn test_partial_liquidation_with_dex_swap_reduced_rate() {
    let mut helper = TestHelper::new();
    let usd = helper.faucet.usdc_resource_address;
    let btc = helper.faucet.btc_resource_address;

    const T2024: i64 = 1704067200;

    helper.test_runner.load_account_from_faucet(helper.owner_account_address);
    helper.test_runner.load_account_from_faucet(helper.owner_account_address);

    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(1), T2024);
    admin_update_price(&mut helper, 1u64, usd, dec!(48.68223442)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(2872008.244)).expect_commit_success();


    // SET UP A LP PROVIDER
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(lp_user_account);
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(15000), btc) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, btc),
        dec!(0.00522282623364)
    );

    market_contribute(&mut helper, lp_user_key, lp_user_account, btc, dec!(0.005))
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, btc),
        dec!(0.00022282623364)
    );

    // SET UP A BORROWER
    let (borrower_key, _, borrower_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(borrower_account);

    //Create CDP
    market_create_cdp(
        &mut helper,
        borrower_key,
        borrower_account,
        vec![(XRD, dec!(13550))],
    ) //
        .expect_commit_success();

    let cdp_id: u64 = 1;
    // Borrow BTC
    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        cdp_id,
        btc,
        dec!(0.0033),
    )
        .expect_commit_success();


    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");
    assert!(event.cdps.is_empty());

    // SET UP LIQUIDATOR
    let auth = rule!(require(NonFungibleGlobalId::from_public_key(&helper.owner_public_key)));
    let (liquidator_user_key, liquidator_user_account) = (helper.owner_public_key, helper.test_runner.new_account_advanced(OwnerRole::Fixed(auth)));
    admin_send_liquidator_badge(&mut helper, 1, liquidator_user_account)
        .expect_commit_success();
    helper
        .test_runner
        .load_account_from_faucet(liquidator_user_account);


    // SWAP TO Cover debt
    swap(
        &mut helper,
        liquidator_user_account,
        liquidator_user_key,
        dec!(10000),
        XRD,
        btc,
    ).expect_commit_success();

    // PREPARE LIQUIDATION
    admin_update_price(&mut helper, 1u64, btc, dec!(2880000)).expect_commit_success();
    market_update_pool_state(&mut helper, btc).expect_commit_success();

    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");

    assert!(!event.cdps.is_empty());
    assert_eq!(pdec!(13550), *event.cdps[0].cdp_data.collaterals.get(&XRD).unwrap());
    assert_eq!(pdec!(0.0033), *event.cdps[0].cdp_data.loans.get(&btc).unwrap());

    // START LIQUIDATION
    check_cdp_for_liquidation(&mut helper, liquidator_user_key, cdp_id).expect_commit_success();

    let mut requested_collaterals: Vec<ResourceAddress> = Vec::new();
    requested_collaterals.push(XRD);

    let xrd_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, XRD);

    let btc_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, btc);

    // Sending around 10% less to simulate a rejected DEX inefficient swap or a possible fraud
    let payments: Vec<(ResourceAddress, Decimal)> = vec![(btc, dec!(0.9) * dec!(0.4) * dec!(0.0033))];
    market_liquidation(
        &mut helper,
        liquidator_user_key,
        liquidator_user_account,
        1,
        cdp_id,
        payments,
        Some(dec!(0.4) * dec!(0.0033) * dec!(2880000)),
        requested_collaterals.clone(),
    ).expect_commit_failure();

    // Sending 3% less which is a tolerable DEX inefficient swap
    let payments: Vec<(ResourceAddress, Decimal)> = vec![(btc, dec!(0.97) * dec!(0.4) * dec!(0.0033))];
    market_liquidation(
        &mut helper,
        liquidator_user_key,
        liquidator_user_account,
        1,
        cdp_id,
        payments,
        Some(dec!(0.4) * dec!(0.0033) * dec!(2880000)),
        requested_collaterals.clone(),
    ).expect_commit_success();

    assert_eq!(dec!(3497.472), helper
        .test_runner
        .get_component_balance(liquidator_user_account, XRD) - xrd_balance);

    assert_eq!(dec!(-0.0012144), helper
        .test_runner
        .get_component_balance(liquidator_user_account, btc) - btc_balance);

    let btc_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, btc);

    swap(
        &mut helper,
        liquidator_user_account,
        liquidator_user_key,
        dec!(3497.472),
        XRD,
        btc,
    ).expect_commit_success();

    assert_eq!(dec!(0.001214399999999222), helper
        .test_runner
        .get_component_balance(liquidator_user_account, btc) - btc_balance);

    assert!(dec!(-0.0012144)+dec!(0.001214399999999222) < ZERO_EPSILON);


    market_update_pool_state(&mut helper, btc).expect_commit_success();
    market_update_pool_state(&mut helper, XRD).expect_commit_success();


    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");

    assert!(event.cdps.is_empty());

    let receipt = market_show_cdp(&mut helper, cdp_id);
    assert!(format!("{:?}", receipt).contains("9748.4"));
    assert!(format!("{:?}", receipt).contains("0.0020856"));
    receipt.expect_commit_success();
}


#[test]
fn test_partial_liquidation_multi_collateral_multi_loan() {
    let mut helper = TestHelper::new();
    let usd = helper.faucet.usdc_resource_address;
    let btc = helper.faucet.btc_resource_address;
    let eth = helper.faucet.eth_resource_address;
    let lsu = helper.faucet.lsu_resource_address;
    let hug = helper.faucet.hug_resource_address;
    let usdt = helper.faucet.usdt_resource_address;

    const T2024: i64 = 1704067200;

    helper.test_runner.load_account_from_faucet(helper.owner_account_address);
    helper.test_runner.load_account_from_faucet(helper.owner_account_address);

    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(1), T2024);
    admin_update_price(&mut helper, 1u64, usd, dec!(25)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1500000)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, eth, dec!(72500)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, lsu, dec!(1)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, hug, dec!(0.001)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, usdt, dec!(25)).expect_commit_success();

    // SET UP A LP PROVIDER
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(lp_user_account);
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(1250), usd) //
        .expect_commit_success();
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(1250), hug) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, usd),
        dec!(50)
    );
    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, hug),
        dec!(1250000)
    );

    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(50))
        .expect_commit_success();
    market_contribute(&mut helper, lp_user_key, lp_user_account, hug, dec!(1250000))
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, usd),
        dec!(0)
    );
    assert_eq!(
        helper
            .test_runner
            .get_component_balance(lp_user_account, hug),
        dec!(0)
    );

    // SET UP A BORROWER
    let (borrower_key, _, borrower_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(borrower_account);
    get_resource(&mut helper, borrower_key, borrower_account, dec!(1250), eth) //
        .expect_commit_success();
    get_resource(&mut helper, borrower_key, borrower_account, dec!(1250), btc) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(borrower_account, eth),
        dec!(0.01724137931034375)
    );
    assert_eq!(
        helper
            .test_runner
            .get_component_balance(borrower_account, btc),
        dec!(0.0008333333333325)
    );

    //Create CDP
    market_create_cdp(
        &mut helper,
        borrower_key,
        borrower_account,
        vec![
            (eth, dec!(0.01724137931034375)),
            (btc, dec!(0.0008333333333325))],
    ) //
        .expect_commit_success();

    let cdp_id: u64 = 1;
    // Borrow
    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        cdp_id,
        usd,
        dec!(35),
    )
        .expect_commit_success();
    market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        cdp_id,
        hug,
        dec!(850000),
    )
        .expect_commit_success();


    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");
    assert!(event.cdps.is_empty());

    // SET UP LIQUIDATOR
    let auth = rule!(require(NonFungibleGlobalId::from_public_key(&helper.owner_public_key)));
    let (liquidator_user_key, liquidator_user_account) = (helper.owner_public_key, helper.test_runner.new_account_advanced(OwnerRole::Fixed(auth)));
    admin_send_liquidator_badge(&mut helper, 1, liquidator_user_account)
        .expect_commit_success();
    helper
        .test_runner
        .load_account_from_faucet(liquidator_user_account);


    // SWAP TO Cover debt
    swap(
        &mut helper,
        liquidator_user_account,
        liquidator_user_key,
        dec!(2000),
        XRD,
        usd,
    ).expect_commit_success();
    swap(
        &mut helper,
        liquidator_user_account,
        liquidator_user_key,
        dec!(2000),
        XRD,
        hug,
    ).expect_commit_success();

    // PREPARE LIQUIDATION
    admin_update_price(&mut helper, 1u64, eth, dec!(58000)).expect_commit_success();
    market_update_pool_state(&mut helper, eth).expect_commit_success();

    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");

    assert!(!event.cdps.is_empty());
    assert_eq!(pdec!(0.01724137931034375), *event.cdps[0].cdp_data.collaterals.get(&eth).unwrap());
    assert_eq!(pdec!(0.0008333333333325), *event.cdps[0].cdp_data.collaterals.get(&btc).unwrap());
    assert_eq!(pdec!(35), *event.cdps[0].cdp_data.loans.get(&usd).unwrap());
    assert_eq!(pdec!(850000), *event.cdps[0].cdp_data.loans.get(&hug).unwrap());

    // START LIQUIDATION
    check_cdp_for_liquidation(&mut helper, liquidator_user_key, cdp_id).expect_commit_success();

    let mut requested_collaterals: Vec<ResourceAddress> = Vec::new();
    requested_collaterals.push(eth);
    requested_collaterals.push(btc);

    let usd_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, usd);

    let hug_balance = helper
        .test_runner
        .get_component_balance(liquidator_user_account, hug);

    let payments: Vec<(ResourceAddress, Decimal)> = vec![(usd, dec!(40)), (hug, dec!(900000))];
    market_liquidation(
        &mut helper,
        liquidator_user_key,
        liquidator_user_account,
        1,
        cdp_id,
        payments,
        Some(dec!(0.4) * (dec!(35) * dec!(25) + dec!(850000) * dec!(0.001))),
        requested_collaterals.clone(),
    ).expect_commit_success();

    assert_eq!(dec!(-25.392000000000001131), helper
        .test_runner
        .get_component_balance(liquidator_user_account, usd) - usd_balance);

    assert_eq!(dec!(0), helper
        .test_runner
        .get_component_balance(liquidator_user_account, hug) - hug_balance);


    market_update_pool_state(&mut helper, eth).expect_commit_success();
    market_update_pool_state(&mut helper, btc).expect_commit_success();
    market_update_pool_state(&mut helper, usd).expect_commit_success();
    market_update_pool_state(&mut helper, hug).expect_commit_success();

    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");

    assert!(event.cdps.is_empty());

    let receipt = market_show_cdp(&mut helper, cdp_id);
    assert!(format!("{:?}", receipt).contains("9.607999999999998869"));
    assert!(format!("{:?}", receipt).contains("850000"));
    receipt.expect_commit_success();
}

#[test]
fn test_contribute_and_borrow_limits() {
    let mut helper = TestHelper::new();
    let usd = helper.faucet.usdc_resource_address;

    // Set up initial pool state
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();
    for _ in 1..30 {
        helper.test_runner.load_account_from_faucet(lp_user_account);
    }
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(300000), usd)
        .expect_commit_success();

    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(10000))
        .expect_commit_success();

    // Set up borrower
    let (borrower_key, _, borrower_account) = helper.test_runner.new_allocated_account();
    for _ in 1..30 {
        helper.test_runner.load_account_from_faucet(borrower_account);
    }
    get_resource(&mut helper, borrower_key, borrower_account, dec!(300000), usd)
        .expect_commit_success();

    // Borrower contributes collateral
    let collateral_amount = dec!(5000);
    market_contribute(&mut helper, borrower_key, borrower_account, usd, collateral_amount)
        .expect_commit_success();

    // Create CDP
    market_create_cdp(
        &mut helper,
        borrower_key,
        borrower_account,
        vec![(usd, collateral_amount)],
    )
        .expect_commit_success();

    // Try to borrow within allowed LTV (assuming 75% LTV)
    let safe_borrow_amount = collateral_amount * dec!(0.7); // 70% of collateral
    let safe_borrow_receipt = market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        1u64,
        usd,
        safe_borrow_amount,
    );

    // This borrow should succeed
    safe_borrow_receipt.expect_commit_success();
    println!("Safe borrow of {} USD succeeded", safe_borrow_amount);

    // Now try to borrow more, exceeding the LTV limit
    let excess_borrow_amount = collateral_amount * dec!(0.3); // Additional 30%, total would be 100%
    let excess_borrow_receipt = market_borrow(
        &mut helper,
        borrower_key,
        borrower_account,
        1u64,
        usd,
        excess_borrow_amount,
    );

    // This borrow should fail
    assert!(excess_borrow_receipt.is_commit_failure(), "Excess borrow should have failed");
}

#[test]
fn test_flashloan_abuse_attempt() {
    let mut helper = TestHelper::new();
    let usd = helper.faucet.usdc_resource_address;
    admin_update_price(&mut helper, 1u64, usd, dec!(0.00001)).expect_commit_success();

    // Set up initial pool state
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(lp_user_account);

    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(100), usd)
        .expect_commit_success();

    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(4_000_000))
        .expect_commit_success();

    // Set up attacker
    let (attacker_key, _, attacker_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(attacker_account);

    get_resource(&mut helper, attacker_key, attacker_account, dec!(500), usd)
        .expect_commit_success();

    // Attacker contributes a large amount
    let attacker_collateral = dec!(9_000_000);
    market_contribute(&mut helper, attacker_key, attacker_account, usd, attacker_collateral)
        .expect_commit_success();

    // Create CDP
    market_create_cdp(
        &mut helper,
        attacker_key,
        attacker_account,
        vec![(usd, attacker_collateral)],
    )
        .expect_commit_success();

    // Try to borrow the maximum possible amount
    let max_borrow_amount = attacker_collateral * dec!(0.75); // Assuming 75% LTV
    let borrow_receipt = market_borrow(
        &mut helper,
        attacker_key,
        attacker_account,
        1u64,
        usd,
        max_borrow_amount,
    );

    // This borrow should succeed
    borrow_receipt.expect_commit_success();

    // Now, try to remove all collateral (which should fail)
    let remove_collateral_receipt = market_remove_collateral(
        &mut helper,
        attacker_key,
        attacker_account,
        1u64,
        usd,
        attacker_collateral,
        false,
    );

    // This removal should fail
    assert!(remove_collateral_receipt.is_commit_failure(), "Collateral removal should have failed");

    // Try to repay a small amount and then remove more collateral than should be allowed
    let small_repay_amount = dec!(1000);
    market_repay(&mut helper, attacker_key, attacker_account, 1u64, usd, small_repay_amount)
        .expect_commit_success();

    let excess_remove_amount = attacker_collateral - max_borrow_amount + small_repay_amount + dec!(1);
    let excess_remove_receipt = market_remove_collateral(
        &mut helper,
        attacker_key,
        attacker_account,
        1u64,
        usd,
        excess_remove_amount,
        false,
    );

    // This removal should also fail
    assert!(excess_remove_receipt.is_commit_failure(), "Excess collateral removal should have failed");
}
