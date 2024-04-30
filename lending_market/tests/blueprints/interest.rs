use crate::helpers::{init::TestHelper, methods::*};
use radix_engine_interface::prelude::*;

#[test]
fn test_interest() {
    let mut helper = TestHelper::new();

    const T2024: i64 = 1704067200;
    const T6_MONTHS: i64 = 15778476000;

    let usd = helper.faucet.usdc_resource_address;
    let btc = helper.faucet.btc_resource_address;
    let usd_pu = helper.market.pools.get(&usd).unwrap().clone().1;

    // 1) Alice deposits 1000 USD
    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(1), T2024);
    admin_update_price(&mut helper, 1u64, usd, dec!(15)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1300000)).expect_commit_success();

    let (alice_key, _, alice_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(alice_account);
    helper.test_runner.load_account_from_faucet(alice_account);

    get_resource(&mut helper, alice_key, alice_account, dec!(15_001), usd)
        .expect_commit_success();

    market_contribute(&mut helper, alice_key, alice_account, usd, dec!(1_000))
        .expect_commit_success();

    let alice_usd_balance = helper.test_runner.get_component_balance(alice_account, usd);

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(alice_account, usd_pu),
        dec!(1_000)
    );

    // 2) Bob borrows 500 USD
    let (bob_key, _, bob_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(bob_account);
    helper.test_runner.load_account_from_faucet(bob_account);

    market_create_cdp(
        &mut helper,
        bob_key,
        bob_account,
        vec![(XRD, dec!(30_000))],
    ) //
    .expect_commit_success();

    market_borrow(
        &mut helper,
        bob_key,
        bob_account,
        1u64,
        usd,
        dec!(500),
    )
    .expect_commit_success();

    // 3) Six months pass
    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(2), T2024 + T6_MONTHS);
    admin_update_price(&mut helper, 1u64, usd, dec!(15)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1300000)).expect_commit_success();

    // 4) Charles deposits 2000 USD
    let (charles_key, _, charles_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(charles_account);
    helper.test_runner.load_account_from_faucet(charles_account);
    helper.test_runner.load_account_from_faucet(charles_account);
    helper.test_runner.load_account_from_faucet(charles_account);
    helper.test_runner.load_account_from_faucet(charles_account);

    get_resource(&mut helper, charles_key, charles_account, dec!(30_001), usd)
        .expect_commit_success();

    market_contribute(&mut helper, charles_key, charles_account, usd, dec!(2_000))
        .expect_commit_success();

    let charles_usd_balance = helper.test_runner.get_component_balance(charles_account, usd);

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(charles_account, usd_pu),
        dec!(1987.552977875112810891)
    );
  
    // 5) Six months pass
    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(3), T2024 + T6_MONTHS * 2);
    admin_update_price(&mut helper, 1u64, usd, dec!(15)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1300000)).expect_commit_success();

    // 6) Charles redeems his 2000 USD
    market_redeem(
        &mut helper,
        charles_key,
        charles_account,
        usd_pu,
        dec!(1987.552977875112810891),
    ) //
        .expect_commit_success();


    assert_eq!(
        helper
            .test_runner
            .get_component_balance(charles_account, usd) - charles_usd_balance,
        dec!(2000.299580553239706061)
    );

    // 7) Six months pass
    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(4), T2024 + T6_MONTHS * 3);
    admin_update_price(&mut helper, 1u64, usd, dec!(15)).expect_commit_success();

    // 8) Bob repays his 500 USD initial debt. Investments returned him 50 usd, we buy them from faucet
    helper.test_runner.load_account_from_faucet(bob_account);
    get_resource(&mut helper, bob_key, bob_account, dec!(750), usd) //
        .expect_commit_success();

    market_repay(
        &mut helper,
        bob_key,
        bob_account,
        1u64,
        usd,
        dec!(514.832176152136555295),
    )
    .expect_commit_success();

    market_remove_collateral(
        &mut helper,
        bob_key,
        bob_account,
        1u64,
        XRD,
        dec!(30_000),
        false,
    )
    .expect_commit_success();

    // 9) Six months pass
    helper
        .test_runner
        .advance_to_round_at_timestamp(Round::of(5), T2024 + T6_MONTHS * 4);
    admin_update_price(&mut helper, 1u64, usd, dec!(15)).expect_commit_success();
    admin_update_price(&mut helper, 1u64, btc, dec!(1300000)).expect_commit_success();

    // 9) Alice redeems her 1000 USD
    market_redeem(
        &mut helper,
        alice_key,
        alice_account,
        usd_pu,
        dec!(1_000),
    ) //
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(alice_account, usd) - alice_usd_balance,
        dec!(1011.554902287405084721)
    );

    // 10) Collect market reserve
    let owner_usd_balance = helper
                .test_runner
                .get_component_balance(helper.owner_account_address, usd);
    market_collect_reserve(&mut helper)
        .expect_commit_success();

    assert_eq!(
        helper
            .test_runner
            .get_component_balance(helper.owner_account_address, usd) - owner_usd_balance,
        dec!(2.977693311491764513)
    );
}
