use crate::helpers::{init::{find_event_in_result, TestHelper}, methods::*};
use lending_market::modules::cdp_data::CDPLiquidableEvent;
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
        dec!(1.999747475611713071)
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
        dec!(2000.000200119327113721)
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
        dec!(0.005),
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
    assert_eq!(pdec!(0.005), *event.cdps[0].cdp_data.loans.get(&btc).unwrap());
    assert_eq!(pdec!(0.05), *event.cdps[0].cdp_data.loans.get(&eth).unwrap());


    // SET UP LIQUIDATOR
    let (liquidator_user_key, _, liquidator_user_account) =
        helper.test_runner.new_allocated_account();
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
        cdp_id,
        payments,
        None,
        requested_collaterals.clone(),
    ).expect_commit_failure();

    let payments: Vec<(ResourceAddress, Decimal)> = vec![(usd, dec!(202)),(btc, dec!(0.0099)), (eth, dec!(0.062))];
    market_liquidation(
        &mut helper,
        liquidator_user_key,
        liquidator_user_account,
        cdp_id,
        payments,
        None,
        requested_collaterals,
    ).expect_commit_success();

    assert_eq!(dec!(18400), helper
        .test_runner
        .get_component_balance(liquidator_user_account, XRD) - xrd_balance);

    assert_eq!(dec!(-201.256247133753602058), helper
        .test_runner
        .get_component_balance(liquidator_user_account, usd) - usd_balance);

    assert_eq!(dec!(-0.008878145116578112), helper
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
    let (liquidator_user_key, _, liquidator_user_account) =
    helper.test_runner.new_allocated_account();
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
        cdp_id,
        payments,
        Some(dec!(0.4) * dec!(3000)),
        requested_collaterals.clone(),
    ).expect_commit_success();

    assert_eq!(dec!(1203.36), helper
        .test_runner
        .get_component_balance(liquidator_user_account, XRD) - xrd_balance);

    assert_eq!(dec!(-42.857142857142857142), helper
        .test_runner
        .get_component_balance(liquidator_user_account, usd) - usd_balance);

  
 
    market_update_pool_state(&mut helper, usd).expect_commit_success();
    market_update_pool_state(&mut helper, XRD).expect_commit_success();
 

    let receipt = market_list_liquidable_cdps(&mut helper);
    let event: CDPLiquidableEvent = find_event_in_result(receipt.expect_commit_success(), "CDPLiquidableEvent").expect("CDPLiquidableEvent not found");
    
    assert!(event.cdps.is_empty());

}
