use radix_engine::vm::NoExtension;
use radix_engine_interface::prelude::*;
use scrypto_test::prelude::*;
use scrypto_unit::*;
use std::path::Path;

use crate::helpers::init::build_and_dump_to_fs;

use super::{faucet::FaucetTestHelper, price_feed::PriceFeedTestHelper};

pub struct MarketTestHelper {
    pub market_component_address: ComponentAddress,
    pub batch_flashloan_resource_address: ResourceAddress,
    pub cdp_resource_address: ResourceAddress,
    pub market_admin_badge: ResourceAddress,
    pub market_reserve_collector_badge: ResourceAddress,
    pub liquidation_term_resource_address: ResourceAddress,
    pub pools: IndexMap<ResourceAddress, (ComponentAddress, ResourceAddress)>,
}

impl MarketTestHelper {
    pub fn new(
        test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
        owner_account_address: ComponentAddress,
        owner_public_key: Secp256k1PublicKey,
        price_feed: &PriceFeedTestHelper,
        faucet: &FaucetTestHelper,
    ) -> MarketTestHelper {
        let _pool_package_address =
            test_runner.compile_and_publish(Path::new("../single_resource_pool"));

        // DONT REMOVE VERY IMPORTANT : ALLOW TO FIND THE PACKAGE ADDRESS OF single_resource_pool ON THE RTM file
        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                _pool_package_address,
                "SingleResourcePool",
                "instantiate_locally",
                manifest_args!(),
            )
            .deposit_batch(owner_account_address);

        dump_manifest_to_file_system(
            manifest.object_names(),
            &manifest.build(),
            "./rtm",
            Some("publish_single_resource_pool"),
            &NetworkDefinition::simulator(),
        )
        .err();

        let market_package_address = test_runner.compile_and_publish(Path::new("."));

        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                market_package_address,
                "LendingMarket",
                "instantiate",
                manifest_args!((10u8, dec!(0.4001))),
            )
            .deposit_batch(owner_account_address);

        let receipt = test_runner.execute_manifest(
            build_and_dump_to_fs(manifest, "Instantiate_market".into()),
            vec![NonFungibleGlobalId::from_public_key(&owner_public_key)],
        );

        let result = receipt.expect_commit(true);

        let component_addresses_created = result.new_component_addresses();
        let market_component_address = component_addresses_created[0];

        let resource_addresses_created = result.new_resource_addresses();

        let market_admin_badge = resource_addresses_created[0];
        let market_reserve_collector_badge = resource_addresses_created[1];
        let cdp_resource_address = resource_addresses_created[2];
        let batch_flashloan_resource_address = resource_addresses_created[3];
        let liquidation_term_resource_address = resource_addresses_created[3];

        // // Pools

        let mut pools = IndexMap::new();
        

        // // Initialize XRD lending pool

        let manifest2 = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .create_proof_from_account_of_non_fungibles(
                owner_account_address,
                market_admin_badge,
                vec![
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2),
                    NonFungibleLocalId::integer(3),
                    NonFungibleLocalId::integer(4),
                ],
            )
            .call_method(
                market_component_address,
                "create_lending_pool",
                manifest_args!(
                    price_feed.price_feed_component_address,
                    XRD,
                    (
                        dec!("0.2"),
                        dec!("0.15"),
                        dec!("0.08"),
                        dec!("0.001"),
                        0u8,
                        dec!("0"),
                        dec!("1"),
                        None::<Decimal>,
                        None::<Decimal>,
                        Some(dec!("0.99")),
                        5i64,
                        15i64,
                        240i64,
                        dec!("0.45"),
                    ),
                    (
                        dec!(0), dec!(0.04), dec!(3.00)
                    ),
                    (
                        None::<Decimal>,
                        Some(dec!("0.8")),
                        IndexMap::<ResourceAddress, Decimal>::new(),
                        IndexMap::<u8, Decimal>::new(),
                        dec!("0.7")
                    )
                ),
                
            )
            .deposit_batch(owner_account_address)
            .build();

        let receipt2 = test_runner.execute_manifest(
            manifest2,
            vec![NonFungibleGlobalId::from_public_key(&owner_public_key)],
        );
        let result2 = receipt2.expect_commit(true);

        pools.insert(
            XRD,
            (
                result2.new_component_addresses()[0],
                result2.new_resource_addresses()[0],
            ),
        );

        // Initialize USD lending pool

        let manifest3 = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .create_proof_from_account_of_non_fungibles(
                owner_account_address,
                market_admin_badge,
                vec![
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2),
                    NonFungibleLocalId::integer(3),
                    NonFungibleLocalId::integer(6),
                ],
            )
            .call_method(
                market_component_address,
                "create_lending_pool",
                manifest_args!(
                    price_feed.price_feed_component_address,
                    faucet.usdc_resource_address,
                    (
                        dec!("0.2"),
                        dec!("0.15"),
                        dec!("0.08"),
                        dec!("0.001"),
                        1u8,
                        dec!("0"),
                        dec!("1"),
                        None::<Decimal>,
                        None::<Decimal>,
                        Some(dec!("0.99")),
                        5i64,
                        15i64,
                        240i64,
                        dec!("0.8"),
                    ),
                    (
                        dec!(0), dec!(0.04), dec!(0.75)
                    ),
                    (
                        None::<Decimal>,
                        Some(dec!("0.8")),
                        IndexMap::<ResourceAddress, Decimal>::new(),
                        IndexMap::<u8, Decimal>::new(),
                        dec!("0.0")
                    )
                ),
            )
            .deposit_batch(owner_account_address)
            .build();

        let receipt3 = test_runner.execute_manifest(
            manifest3,
            vec![NonFungibleGlobalId::from_public_key(&owner_public_key)],
        );

        let result3 = receipt3.expect_commit(true);

        pools.insert(
            faucet.usdc_resource_address,
            (
                result3.new_component_addresses()[0],
                result3.new_resource_addresses()[0],
            ),
        );

        // Initialize BTC lending pool

        let manifest4 = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .create_proof_from_account_of_non_fungibles(
                owner_account_address,
                market_admin_badge,
                vec![
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2),
                    NonFungibleLocalId::integer(3),
                    NonFungibleLocalId::integer(6),
                ],
            )
            .call_method(
                market_component_address,
                "create_lending_pool",
                manifest_args!(
                    price_feed.price_feed_component_address,
                    faucet.btc_resource_address,
                    (
                        dec!("0.2"),
                        dec!("0.15"),
                        dec!("0.08"),
                        dec!("0.001"),
                        0u8,
                        dec!("0"),
                        dec!("1"),
                        None::<Decimal>,
                        None::<Decimal>,
                        Some(dec!("0.99")),
                        5i64,
                        15i64,
                        240i64,
                        dec!("0.45")
                    ),
                    (
                        dec!(0), dec!(0.04), dec!(3.00)
                    ),
                    (
                        None::<Decimal>,
                        Some(dec!("0.8")),
                        IndexMap::<ResourceAddress, Decimal>::new(),
                        IndexMap::<u8, Decimal>::new(),
                        dec!("0.7")
                    )
                ),
            )
            .deposit_batch(owner_account_address)
            .build();

        let receipt4 = test_runner.execute_manifest(
            manifest4,
            vec![NonFungibleGlobalId::from_public_key(&owner_public_key)],
        );

        let result4 = receipt4.expect_commit(true);

        pools.insert(
            faucet.btc_resource_address,
            (
                result4.new_component_addresses()[0],
                result4.new_resource_addresses()[0],
            ),
        );


        // Initialize ETH lending pool

        let manifest5 = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .create_proof_from_account_of_non_fungibles(
                owner_account_address,
                market_admin_badge,
                vec![
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2),
                    NonFungibleLocalId::integer(3),
                    NonFungibleLocalId::integer(6),
                ],
            )
            .call_method(
                market_component_address,
                "create_lending_pool",
                manifest_args!(
                    price_feed.price_feed_component_address,
                    faucet.eth_resource_address,
                    (
                        dec!("0.2"),
                        dec!("0.15"),
                        dec!("0.08"),
                        dec!("0.001"),
                        0u8,
                        dec!("0"),
                        dec!("1"),
                        None::<Decimal>,
                        None::<Decimal>,
                        Some(dec!("0.99")),
                        5i64,
                        15i64,
                        240i64,
                        dec!("0.45"),
                    ),
                    (
                        dec!(0), dec!(0.04), dec!(3.00)
                    ),
                    (
                        None::<Decimal>,
                        Some(dec!("0.8")),
                        IndexMap::<ResourceAddress, Decimal>::new(),
                        IndexMap::<u8, Decimal>::new(),
                        dec!("0.7")
                    )
                ),
            )
            .deposit_batch(owner_account_address)
            .build();

        let receipt5 = test_runner.execute_manifest(
            manifest5,
            vec![NonFungibleGlobalId::from_public_key(&owner_public_key)],
        );

        let result5: &CommitResult = receipt5.expect_commit(true);

        pools.insert(
            faucet.eth_resource_address,
            (
                result5.new_component_addresses()[0],
                result5.new_resource_addresses()[0],
            ),
        );

        Self {
            market_component_address,
            market_admin_badge,
            batch_flashloan_resource_address,
            cdp_resource_address,
            market_reserve_collector_badge,
            liquidation_term_resource_address,
            pools,
        }
    }
}
