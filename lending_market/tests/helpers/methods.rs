use super::init::{build_and_dump_to_fs, TestHelper};
use radix_engine_interface::prelude::*;
use scrypto_test::prelude::*;

pub fn admin_send_liquidator_badge(
    helper: &mut TestHelper,
    liquidator_badge_id: u64,
    user_account_address: ComponentAddress,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungibles(
            helper.owner_account_address,
            helper.market.market_admin_badge,
            vec![
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
                NonFungibleLocalId::integer(3),
                NonFungibleLocalId::integer(4),
            ],
        )
        .call_method(
            helper.market.market_component_address,
            "mint_liquidator_badge",
            manifest_args!(),
        )
        .take_non_fungibles_from_worktop(helper.market.market_liquidator_badge, vec![
            NonFungibleLocalId::integer(liquidator_badge_id),
        ], "badge_bucket")
        .with_name_lookup(|builder, lookup| {
            let badge_bucket = lookup.bucket("badge_bucket");
            builder.deposit(user_account_address, badge_bucket)
        });
        

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest, "admin_send_liquidator_badge".into()),
        vec![NonFungibleGlobalId::from_public_key(&helper.owner_public_key)],
    )
}

pub fn admin_update_price(
    helper: &mut TestHelper,
    admin_non_fungible_id: u64,
    resource_address: ResourceAddress,
    price: Decimal,
) -> TransactionReceiptV1 {
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungible(
            helper.owner_account_address,
            NonFungibleGlobalId::new(
                helper.price_feed.price_feed_admin_badge,
                NonFungibleLocalId::integer(admin_non_fungible_id),
            ),
        )
        .call_method(
            helper.price_feed.price_feed_component_address,
            "admin_update_price",
            manifest_args!(resource_address, price),
        )
        .deposit_batch(helper.owner_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest, "admin_update_price".into()),
        vec![NonFungibleGlobalId::from_public_key(
            &helper.owner_public_key,
        )],
    )
}

pub fn get_price(
    helper: &mut TestHelper,
    resource_address: ResourceAddress,
) -> TransactionReceiptV1 {
    let manifest = ManifestBuilder::new().lock_fee_from_faucet().call_method(
        helper.price_feed.price_feed_component_address,
        "get_price",
        manifest_args!(resource_address),
    );

    let receipt = helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest, "get_price".into()),
        vec![NonFungibleGlobalId::from_public_key(
            &helper.owner_public_key,
        )],
    );

    receipt.expect_commit_success();

    println!("{:?}\n", receipt);

    receipt
}
pub fn get_resource_flash_loan(
    helper: &mut TestHelper,
    _user_public_key: Secp256k1PublicKey,
    _user_account_address: ComponentAddress,
    xrd_amount: Decimal,
    manifest_builder: ManifestBuilder,
) {
    manifest_builder
        .lock_fee_from_faucet()
        .take_from_worktop(XRD, xrd_amount, "xrd_bucket")
        .with_name_lookup(|builder, lookup| {
            let xrd_bucket = lookup.bucket("xrd_bucket");
            builder.call_method(
                helper.faucet.faucet_component_address,
                "get_resource",
                manifest_args!(helper.faucet.usdc_resource_address, xrd_bucket),
            )
        });
}

#[allow(dead_code)]
fn mint_resource_from_faucet(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    amount: Decimal,
    resource_address: ResourceAddress,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            helper.faucet.faucet_component_address,
            "mint_resource",
            manifest_args!(resource_address, amount),
        )
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest, "mint_resource_from_faucet".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn get_resource(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    xrd_amount: Decimal,
    resource_address: ResourceAddress,
) -> TransactionReceipt {
    let manifest_builder_0 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(user_account_address, XRD, xrd_amount)
        .take_all_from_worktop(XRD, "xrd_bucket")
        .with_name_lookup(|builder, lookup| {
            let xrd_bucket = lookup.bucket("xrd_bucket");
            builder.call_method(
                helper.faucet.faucet_component_address,
                "get_resource",
                manifest_args!(resource_address, xrd_bucket),
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder_0, "get_resource".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn swap(
    helper: &mut TestHelper,
    account_address: ComponentAddress,
    account_public_key: Secp256k1PublicKey,
    from_amount: Decimal,
    from_resource_address: ResourceAddress,
    to_resource_address: ResourceAddress,
) -> TransactionReceiptV1 {
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account_address, from_resource_address, from_amount)
        .take_all_from_worktop(from_resource_address, "from_tokens")
        .with_name_lookup(|builder, lookup: ManifestNameLookup| {
            let bucket = lookup.bucket("from_tokens");
            builder
                .call_method(
                    helper.faucet.faucet_component_address,
                    "swap",
                    manifest_args!(bucket, to_resource_address),
                )
                .deposit_batch(account_address)
        })
        .build();

    let receipt = helper.test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&account_public_key)],
    );

    println!("{:?}\n", receipt);

    receipt
}

pub fn market_update_pool_state(
    helper: &mut TestHelper,
    res_address: ResourceAddress,
) -> TransactionReceiptV1 {
    let manifest = ManifestBuilder::new().lock_fee_from_faucet().call_method(
        helper.market.market_component_address,
        "update_pool_state",
        manifest_args!(res_address, true, true),
    );

    let receipt = helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest, "update_pool_state".into()),
        vec![NonFungibleGlobalId::from_public_key(
            &helper.owner_public_key,
        )],
    );

    receipt.expect_commit_success();

    println!("{:?}\n", receipt);

    receipt
}

pub fn market_list_liquidable_cdps(
    helper: &mut TestHelper,
) -> TransactionReceiptV1 {
    let manifest = ManifestBuilder::new().lock_fee_from_faucet().call_method(
        helper.market.market_component_address,
        "list_liquidable_cdps",
        manifest_args!(0u64, 10u64)
    );

    let receipt = helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest, "list_liquidable_cdps".into()),
        vec![NonFungibleGlobalId::from_public_key(
            &helper.owner_public_key,
        )],
    );

    receipt.expect_commit_success();

    println!("{:?}\n", receipt);

    receipt
}

pub fn market_show_cdp(
    helper: &mut TestHelper,
    cdp_id: u64,
) -> TransactionReceiptV1 {
    let manifest = ManifestBuilder::new().lock_fee_from_faucet().call_method(
        helper.market.market_component_address,
        "show_cdp",
        manifest_args!(cdp_id),
    );

    let receipt = helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest, "show_cdp".into()),
        vec![NonFungibleGlobalId::from_public_key(
            &helper.owner_public_key,
        )],
    );

    receipt.expect_commit_success();

    println!("{:?}\n", receipt);

    receipt
}


pub fn market_contribute(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    res_address: ResourceAddress,
    amount: Decimal,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(user_account_address, res_address, amount)
        .take_all_from_worktop(res_address, "res_bucket")
        .with_name_lookup(|builder, lookup| {
            let bucket = lookup.bucket("res_bucket");

            builder.call_method(
                helper.market.market_component_address,
                "contribute",
                manifest_args!(bucket),
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "contribute".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_redeem(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    res_address: ResourceAddress,
    amount: Decimal,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(user_account_address, res_address, amount)
        .take_all_from_worktop(res_address, "res_bucket")
        .with_name_lookup(|builder, lookup| {
            let bucket = lookup.bucket("res_bucket");

            builder.call_method(
                helper.market.market_component_address,
                "redeem",
                manifest_args!(bucket),
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "redeem".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_create_cdp(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    deposits: Vec<(ResourceAddress, Decimal)>,
) -> TransactionReceipt {
    let mut buckets = Vec::<ManifestBucket>::new();

    let manifest_builder_0 = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .with_name_lookup(|builder, _lookup| {
            let (_, newbuilder) =
                deposits
                    .iter()
                    .fold((0, builder), |(i, builder), (res_address, amount)| {
                        let i = i + 1;
                        (
                            i,
                            builder
                                .withdraw_from_account(user_account_address, *res_address, *amount)
                                .take_all_from_worktop(*res_address, format!("res_bucket_{}", i))
                                .with_name_lookup(|builder, lookup| {
                                    buckets.push(lookup.bucket(format!("res_bucket_{}", i)));

                                    builder
                                }),
                        )
                    });

            newbuilder
        })
        .call_method(
            helper.market.market_component_address,
            "create_cdp",
            manifest_args!(None::<Decimal>, None::<Decimal>, None::<Decimal>, buckets),
        )
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder_0, "create_cdp".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_add_collateral(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    cdp_id: u64,
    res_address: ResourceAddress,
    amount: Decimal,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungible(
            user_account_address,
            NonFungibleGlobalId::new(
                helper.market.cdp_resource_address,
                NonFungibleLocalId::Integer(cdp_id.into()),
            ),
        )
        .pop_from_auth_zone("cdp_proof")
        .withdraw_from_account(user_account_address, res_address, amount)
        .take_all_from_worktop(res_address, "res_bucket")
        .with_name_lookup(|builder, lookup| {
            let proof = lookup.proof("cdp_proof");
            let bucket = lookup.bucket("res_bucket");

            builder.call_method(
                helper.market.market_component_address,
                "add_collateral",
                manifest_args!(proof, vec![bucket]),
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "add_collateral".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_remove_collateral(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    cdp_id: u64,
    res_address: ResourceAddress,
    amount: Decimal,
    keep_pool_units: bool,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungible(
            user_account_address,
            NonFungibleGlobalId::new(
                helper.market.cdp_resource_address,
                NonFungibleLocalId::Integer(cdp_id.into()),
            ),
        )
        .pop_from_auth_zone("cdp_proof")
        .with_name_lookup(|builder, lookup| {
            let proof = lookup.proof("cdp_proof");

            builder.call_method(
                helper.market.market_component_address,
                "remove_collateral",
                manifest_args!(proof, vec![(res_address, amount, keep_pool_units)]),
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "remove_collateral".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_borrow(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    cdp_id: u64,
    res_address: ResourceAddress,
    amount: Decimal,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungible(
            user_account_address,
            NonFungibleGlobalId::new(
                helper.market.cdp_resource_address,
                NonFungibleLocalId::Integer(cdp_id.into()),
            ),
        )
        .pop_from_auth_zone("cdp_proof")
        .with_name_lookup(|builder, lookup| {
            let proof = lookup.proof("cdp_proof");

            builder.call_method(
                helper.market.market_component_address,
                "borrow",
                manifest_args!(proof, vec![(res_address, amount)]),
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "borrow".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_repay(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    cdp_id: u64,
    res_address: ResourceAddress,
    amount: Decimal,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungible(
            user_account_address,
            NonFungibleGlobalId::new(
                helper.market.cdp_resource_address,
                NonFungibleLocalId::Integer(cdp_id.into()),
            ),
        )
        .pop_from_auth_zone("cdp_proof")
        .withdraw_from_account(user_account_address, res_address, amount)
        .take_all_from_worktop(res_address, "res_bucket")
        .with_name_lookup(|builder, lookup| {
            let proof = lookup.proof("cdp_proof");
            let bucket = lookup.bucket("res_bucket");

            builder.call_method(
                helper.market.market_component_address,
                "repay",
                manifest_args!(proof, None::<NonFungibleLocalId>, vec![bucket]),
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "repay".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn check_cdp_for_liquidation(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    cdp_id: u64,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
    .lock_fee_from_faucet()
    .call_method(
        helper.market.market_component_address,
        "check_cdp_for_liquidation",
        manifest_args!(NonFungibleLocalId::integer(cdp_id)),
    );
    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "check_cdp_for_liquidation".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_liquidation(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    liquidator_badge_id: u64,
    cdp_id: u64,
    payments: Vec<(ResourceAddress, Decimal)>,
    total_payment_value: Option<Decimal>,
    requested_collaterals: Vec<ResourceAddress>,
) -> TransactionReceipt {
    let mut manifest_builder = ManifestBuilder::new()
        .lock_fee(FAUCET, 20000)
        .create_proof_from_account_of_non_fungible(
            user_account_address,
            NonFungibleGlobalId::new(
                helper.market.market_liquidator_badge,
                NonFungibleLocalId::integer(liquidator_badge_id),
            ),
        )
        .call_method(
            helper.market.market_component_address,
            "start_liquidation",
            manifest_args!(
                NonFungibleLocalId::integer(cdp_id),
                requested_collaterals.clone(),
                total_payment_value
            ),
        );

    let mut payment_buckets = Vec::<ManifestBucket>::new();
    let encoder = AddressBech32Encoder::for_simulator();
    for (res_address, amount) in payments {
        manifest_builder = manifest_builder
            .withdraw_from_account(user_account_address, res_address, amount)
            .take_all_from_worktop(
                res_address,
                format!("payment_bucket_{}", encoder.encode(&res_address.to_vec()).unwrap()),
            )
            .with_name_lookup(|builder, lookup| {
                payment_buckets
                    .push(lookup.bucket(format!("payment_bucket_{}", encoder.encode(&res_address.to_vec()).unwrap())));
                builder
            });
    }

    manifest_builder = manifest_builder
        .take_all_from_worktop(
            helper.market.liquidation_term_resource_address,
            "liquidation_term_bucket",
        );

    manifest_builder = manifest_builder
        .with_name_lookup(|builder, lookup| {
            let liquidation_term_bucket = lookup.bucket("liquidation_term_bucket");
            builder.call_method(
                helper.market.market_component_address,
                "end_liquidation",
                manifest_args!(payment_buckets, liquidation_term_bucket)
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "market_liquidation".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_fast_liquidation(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    liquidator_badge_id: u64,
    cdp_id: u64,
    payments: Vec<(ResourceAddress, Decimal)>,
    requested_collaterals: Vec<ResourceAddress>,
) -> TransactionReceipt {
    let mut payment_buckets = Vec::<ManifestBucket>::new();
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungible(
            user_account_address,
            NonFungibleGlobalId::new(
                helper.market.market_liquidator_badge,
                NonFungibleLocalId::integer(liquidator_badge_id),
            ),
        )
        .with_name_lookup(|builder, _lookup| {
            let (_, newbuilder) =
                payments
                    .iter()
                    .fold((0, builder), |(i, builder), (res_address, amount)| {
                        (
                            i + 1,
                            builder
                                .withdraw_from_account(user_account_address, *res_address, *amount)
                                .take_all_from_worktop(
                                    *res_address,
                                    format!("payment_bucket_{}", i),
                                )
                                .with_name_lookup(|builder, lookup| {
                                    payment_buckets
                                        .push(lookup.bucket(format!("payment_bucket_{}", i)));
                                    builder
                                }),
                        )
                    });

            newbuilder.call_method(
                helper.market.market_component_address,
                "fast_liquidation",
                manifest_args!(
                    NonFungibleLocalId::integer(cdp_id),
                    payment_buckets,
                    requested_collaterals
                ),
            )
        })
        .deposit_batch(user_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "start_liquidation".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_take_batch_flashloan(
    helper: &mut TestHelper,
    _user_public_key: Secp256k1PublicKey,
    _user_account_address: ComponentAddress,
    loan_amounts: IndexMap<ResourceAddress, Decimal>,
    manifest_builder: ManifestBuilder,
) {
    manifest_builder.lock_fee_from_faucet().call_method(
        helper.market.market_component_address,
        "take_batch_flashloan",
        manifest_args!(loan_amounts),
    );
}

pub fn market_collect_reserve(
    helper: &mut TestHelper,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungibles(
            helper.owner_account_address,
            helper.market.market_reserve_collector_badge,
            vec![
                NonFungibleLocalId::integer(1),
            ],
        )
        .with_name_lookup(|builder, _| {
            builder.call_method(
                helper.market.market_component_address,
                "collect_reserve",
                manifest_args!(),
            )
        })
        .deposit_batch(helper.owner_account_address);

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "collect_reserve".into()),
        vec![NonFungibleGlobalId::from_public_key(&helper.owner_public_key)],
    )
}

pub fn market_update_config(
    helper: &mut TestHelper,
    user_public_key: Secp256k1PublicKey,
    _user_account_address: ComponentAddress,
    new_max_cdp_position: u8,
) -> TransactionReceipt {
    let manifest_builder = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            helper.market.market_component_address,
            "update_market_config",
            manifest_args!(new_max_cdp_position),
        );

    helper.test_runner.execute_manifest(
        build_and_dump_to_fs(manifest_builder, "update_market_config".into()),
        vec![NonFungibleGlobalId::from_public_key(&user_public_key)],
    )
}

pub fn market_repay_batch_flashloan(
    helper: &mut TestHelper,
    _user_public_key: Secp256k1PublicKey,
    user_account_address: ComponentAddress,
    payments: Vec<(ResourceAddress, Decimal)>,
    manifest_builder: ManifestBuilder,
) {
    let mut payment_buckets = Vec::<ManifestBucket>::new();
    manifest_builder
        .lock_fee_from_faucet()
        .take_from_worktop(
            helper.market.batch_flashloan_resource_address,
            Decimal::from(1),
            "flash_loan_term_bucket",
        )
        .with_name_lookup(|builder, _lookup| {
            let flash_loan_term_bucket = _lookup.bucket("flash_loan_term_bucket");
            let (_, newbuilder) =
                payments
                    .iter()
                    .fold((0, builder), |(i, builder), (res_address, amount)| {
                        (
                            i,
                            builder
                                .withdraw_from_account(user_account_address, *res_address, *amount)
                                .take_all_from_worktop(
                                    *res_address,
                                    format!("payment_bucket_{}", i),
                                )
                                .with_name_lookup(|builder, lookup| {
                                    payment_buckets
                                        .push(lookup.bucket(format!("payment_bucket_{}", i)));
                                    builder
                                }),
                        )
                    });

            newbuilder
                .call_method(
                    helper.market.market_component_address,
                    "repay_batch_flashloan",
                    manifest_args!(payment_buckets, flash_loan_term_bucket),
                )
                .deposit_batch(user_account_address)
        });
}



pub fn market_list_info_stats(helper: &mut TestHelper) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            helper.market.market_component_address,
            "list_info_stats",
            manifest_args!()
        )
        .build();

    helper.test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&helper.owner_public_key)],
    )
}

// fn generic_txm(manifest_builder: ManifestBuilder) -> ManifestBuilder {
//     manifest_builder
//         .lock_fee_from_faucet()
//         .create_proof_from_account_of_non_fungible(
//             user_account_address,
//             NonFungibleGlobalId::new(
//                 helper.market.cdp_resource_address,
//                 NonFungibleLocalId::Integer(cdp_id.into()),
//             ),
//         )
//         .pop_from_auth_zone("cdp_proof")
//         .withdraw_from_account(user_account_address, res_address, amount)
//         .take_all_from_worktop(res_address, "res_bucket");

//     manifest_builder
// }
// fn generic_cdp_txm() {}
