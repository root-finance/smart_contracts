use crate::helpers::{init::TestHelper, methods::*};
use radix_engine_interface::prelude::*;
use transaction::builder::ManifestBuilder;

#[test]
#[ignore = "Tested code was definitively removed because unused"]
pub fn test_invalid_flash_loan() {
    let mut helper = TestHelper::new();

    let usd = helper.faucet.usdc_resource_address.clone();

    // SET UP A LP PROVIDER
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);

    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(10_000), usd);
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(10_000), usd);

    // Provide 15000 XRD
    market_contribute(&mut helper, lp_user_key, lp_user_account, XRD, dec!(15_000))
        .expect_commit_success();

    // Provide 15000 USDC
    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(600))
        .expect_commit_success();

    // FLASH LOAN
    // let (user_public_key, _, user_account_address) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(lp_user_account);

    let mut loan_amounts: IndexMap<ResourceAddress, Decimal> = IndexMap::new();

    loan_amounts.insert(XRD, dec!(1000));
    loan_amounts.insert(usd, dec!(100));

    let manifest_builder = ManifestBuilder::new();

    let manifest = manifest_builder
        .lock_fee_from_faucet()
        // TAKE FLASH LOAN
        .call_method(
            helper.market.market_component_address,
            "take_batch_flashloan",
            manifest_args!(loan_amounts),
        )
        .call_method(lp_user_account, "withdraw", manifest_args!(XRD, dec!(1100)))
        .call_method(lp_user_account, "withdraw", manifest_args!(usd, dec!(110)))
        .take_all_from_worktop(XRD, "xrd_buket")
        .take_all_from_worktop(usd, "usd_buket")
        .take_from_worktop(
            helper.market.batch_flashloan_resource_address,
            Decimal::from(1),
            "flash_loan_term_bucket",
        )
        .with_name_lookup(|builder, _lookup| {
            let flash_loan_term_bucket = _lookup.bucket("flash_loan_term_bucket");
            let xrd_bucket = _lookup.bucket("xrd_buket");

            builder
                .call_method(
                    helper.market.market_component_address,
                    "repay_batch_flashloan",
                    manifest_args!(vec![xrd_bucket], flash_loan_term_bucket),
                )
                .deposit_batch(lp_user_account)
        })
        .build();

    // market_take_batch_flashloan(&mut helper,user_public_key, user_account_address, loan_amounts,  &mut manifest_builder) ;
    // get_resource_flash_loan(&mut helper, user_public_key, user_account_address, loan_amount,&mut  manifest_builder);
    //market_repay_batch_flashloan(&mut helper, user_public_key, user_account_address, payments, &mut manifest_builder);

    helper
        .test_runner
        .execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&lp_user_key)],
        )
        .expect_commit_failure();
}

#[test]
#[ignore = "Tested code was definitively removed because unused"]
pub fn test_valid_flash_loan() {
    let mut helper = TestHelper::new();

    let usd = helper.faucet.usdc_resource_address.clone();

    // SET UP A LP PROVIDER
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);

    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(10_000), usd);
    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(10_000), usd);

    // Provide 15000 XRD
    market_contribute(&mut helper, lp_user_key, lp_user_account, XRD, dec!(15_000))
        .expect_commit_success();

    // Provide 15000 USDC
    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(600))
        .expect_commit_success();

    // FLASH LOAN
    // let (user_public_key, _, user_account_address) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(lp_user_account);

    let mut loan_amounts: IndexMap<ResourceAddress, Decimal> = IndexMap::new();

    loan_amounts.insert(XRD, dec!(1000));
    loan_amounts.insert(usd, dec!(100));

    let manifest_builder = ManifestBuilder::new();

    let manifest = manifest_builder
        .lock_fee_from_faucet()
        // TAKE FLASH LOAN
        .call_method(
            helper.market.market_component_address,
            "take_batch_flashloan",
            manifest_args!(loan_amounts),
        )
        .call_method(lp_user_account, "withdraw", manifest_args!(XRD, dec!(1100)))
        .call_method(lp_user_account, "withdraw", manifest_args!(usd, dec!(110)))
        .take_all_from_worktop(XRD, "xrd_buket")
        .take_all_from_worktop(usd, "usd_buket")
        .take_all_from_worktop(usd, "usd_buket_2")
        .take_from_worktop(
            helper.market.batch_flashloan_resource_address,
            Decimal::from(1),
            "flash_loan_term_bucket",
        )
        .with_name_lookup(|builder, lookup| {
            let flash_loan_term_bucket = lookup.bucket("flash_loan_term_bucket");
            let xrd_bucket = lookup.bucket("xrd_buket");
            let usd_bucket = lookup.bucket("usd_buket");
            let usd_bucket_2 = lookup.bucket("usd_buket_2");

            builder
                .call_method(
                    helper.market.market_component_address,
                    "repay_batch_flashloan",
                    manifest_args!(
                        vec![xrd_bucket, usd_bucket, usd_bucket_2],
                        flash_loan_term_bucket
                    ),
                )
        })
        .deposit_batch(lp_user_account)
        .build();

    helper
        .test_runner
        .execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&lp_user_key)],
        )
        .expect_commit_success();
}


#[test]
#[ignore = "Tested code was definitively removed because unused"]
pub fn test_exploit_flashloan_by_burning_transient() {
    let mut helper = TestHelper::new();

    let usd = helper.faucet.usdc_resource_address.clone();

    // SET UP A LP PROVIDER
    let (lp_user_key, _, lp_user_account) = helper.test_runner.new_allocated_account();

    helper.test_runner.load_account_from_faucet(lp_user_account);
    helper.test_runner.load_account_from_faucet(lp_user_account);

    get_resource(&mut helper, lp_user_key, lp_user_account, dec!(20_000), usd);

    // Provide 600 USDC
    market_contribute(&mut helper, lp_user_key, lp_user_account, usd, dec!(400))
        .expect_commit_success();

    // SET UP EXPLOITER
    let (exploiter_key, _, exploiter_account) = helper.test_runner.new_allocated_account();
    helper.test_runner.load_account_from_faucet(exploiter_account);

    let initial_balance = helper.test_runner.get_component_balance(exploiter_account, usd);

    // EXPLOIT FLASHLOAN
    let loan_amounts: IndexMap<ResourceAddress, Decimal> = indexmap! {
        usd => dec!(50)
    };

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        // TAKE FLASH LOAN
        .call_method(
            helper.market.market_component_address,
            "take_batch_flashloan",
            manifest_args!(loan_amounts),
        )
        // Instead of repaying, we burn the transient resource
        .take_all_from_worktop(helper.market.batch_flashloan_resource_address, "flash_loan_term_bucket")
        .with_name_lookup(|builder, lookup| {
            let flash_loan_term_bucket = lookup.bucket("flash_loan_term_bucket");
            builder.burn_resource(flash_loan_term_bucket)
        })
        // Transfer the borrowed funds to our account
        .deposit_batch(exploiter_account)
        .build();

    let receipt = helper.test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&exploiter_key)],
    );

    // The transaction should not succeed
    receipt.expect_commit_failure();

    // Check that the exploiter's balance has not increased
    let final_balance = helper.test_runner.get_component_balance(exploiter_account, usd);
    assert_eq!(final_balance, initial_balance);

}