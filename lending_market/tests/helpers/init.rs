use radix_engine::vm::NoExtension;
use scrypto_test::prelude::*;
use scrypto_unit::*;

use super::{faucet::FaucetTestHelper, market::MarketTestHelper, price_feed::PriceFeedTestHelper};

pub struct TestHelper {
    pub test_runner: TestRunner<NoExtension, InMemorySubstateDatabase>,
    pub owner_account_address: ComponentAddress,
    pub owner_private_key: Secp256k1PrivateKey,
    pub owner_public_key: Secp256k1PublicKey,

    pub price_feed: PriceFeedTestHelper,
    pub faucet: FaucetTestHelper,
    pub market: MarketTestHelper,
}

impl TestHelper {
    pub fn new() -> TestHelper {
        let mut test_runner = TestRunnerBuilder::new()
        .with_custom_genesis(CustomGenesis::default(Epoch::zero(), CustomGenesis::default_consensus_manager_config()))
        .build();

        let (owner_public_key, owner_private_key, owner_account_address) =
            test_runner.new_allocated_account();

        let price_feed =
            PriceFeedTestHelper::new(&mut test_runner, owner_account_address, owner_public_key);

        let faucet = FaucetTestHelper::new(
            &mut test_runner,
            owner_account_address,
            owner_public_key,
            &price_feed,
        );

        let market = MarketTestHelper::new(
            &mut test_runner,
            owner_account_address,
            owner_public_key,
            &price_feed,
            &faucet,
        );

        let helper = Self {
            test_runner,
            owner_account_address,
            owner_private_key,
            owner_public_key,

            price_feed,
            faucet,
            market,
        };

        helper
    }
}

pub fn build_and_dump_to_fs(
    manifest_builder: ManifestBuilder,
    name: String,
) -> TransactionManifestV1 {
    let naming = manifest_builder.object_names();
    let manifest = manifest_builder.build();

    dump_manifest_to_file_system(
        naming,
        &manifest,
        "./rtm",
        Some(&name),
        &NetworkDefinition::simulator(),
    )
    .err();

    manifest
}


pub fn find_event_in_result<T: ScryptoDecode>(result: &CommitResult, event_type: &str) -> Option<T> {
    result.application_events.iter().find_map(|(EventTypeIdentifier(_, id), event)|{
        if id == &event_type {
            Some(scrypto_decode::<T>(&event).unwrap())
        } else {
            None
        }
    })
}