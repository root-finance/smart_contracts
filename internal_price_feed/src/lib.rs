use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone, Debug)]
pub struct PriceData {
   pub pair_index: u32,
   pub price: u128,
   pub timestamp: u64,
   pub round: u64,
   pub decimal: u16
}

impl PriceData {
   pub fn to_decimal_price(&self) -> Decimal {
       Decimal::from(self.price) / 10.pow(self.decimal)
   }
}

// Define derive data structure to handle the data pairs that are derived using price updates recieved.
#[derive(ScryptoSbor, Debug)]
pub struct DerivedData {
   pub round_difference : i64,
   pub derived_price : u128,
   pub decimal : u16
}

#[derive(ScryptoSbor, Clone)]
pub struct PriceInfo {
   pub timestamp: i64,
   pub price: Decimal,
}


/// Supra requires price info to be fetched first an then additional entries can be expressed as quotient or multiplication of predefined pairs, 
/// available here: https://supra.com/docs/data-feeds/data-feeds-index
#[derive(ScryptoSbor, Clone, Debug)]
pub enum PriceFeedStrategy {
    /// Read the price from data pairs (price info must be available): pair_id_1, pair_id_2, operation (0 = multiplication,  1 = division)
    DataPairs(Vec<(u32, u32, u32)>),
    /// Read price info directly: index, operation (0 = normal, 1 = inverse)
    PriceInfo(u32, u32)
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct AuthBadgeData {}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct UpdaterBadgeData {
    pub active: bool,
}

#[blueprint]
mod price_feed {

    enable_method_auth! {
        roles {
            admin => updatable_by: [];
            updater => updatable_by: [admin];
        },
        methods {
            mint_updater_badge => restrict_to: [admin];
            update_updater_badge => restrict_to: [admin];
            admin_update_price => restrict_to: [admin];
            admin_remove_price => restrict_to: [admin];
            admin_update_feed => restrict_to: [admin];

            update_price => restrict_to: [updater];

            get_price => PUBLIC;
        }
    }

    extern_blueprint!(
        "package_sim1ph6xspj0xlmspjju2asxg7xnucy7tk387fufs4jrfwsvt85wvqf70a", // resim sdk
        // "package_sim1phhyaadjcggz9vs26vp5rl52pvsa0mppqkfkt9ld7rqdndxpzcl9j8", // testing
        // "package_tdx_2_1p5d0u603fjmut66kwf29wrmjt5l0ug4aaxvugqs7pmlwaxpuemuj04", // stokenet
        
        contract_pull as ContractPull {
             fn verify_proofs_and_get_data(&self, data: Vec<u8>) -> Vec<PriceData>;
             fn get_derived_svalue(&self, pair_id_1: u32, pair_id_2: u32, operation: u32) -> DerivedData ;
        }
    );
    const CP: Global<ContractPull> = global_component!(
        ContractPull,
        "component_sim1cpyeaya6pehau0fn7vgavuggeev64gahsh05dauae2uu25njcsk6j7" // resim sdk
        // "component_sim1czs7227zwrn4h0wrqfcxsw0pvlkxslt8k5w5aadkl0ctz4aagus7n6" // testing
        // "component_tdx_2_1czvw86xcxdafjkl9nmk6ejq8yp6rhcj5eydn39pxydaf0ny040k2md" // stokenet
    );

    pub struct PriceFeed {
        prices: IndexMap<ResourceAddress, PriceInfo>,
        feed: IndexMap<ResourceAddress, (Vec<u8>, PriceFeedStrategy)>,
        updater_badge_manager: ResourceManager,
        updater_counter: u64,
    }

    impl PriceFeed {
        pub fn instantiate() -> NonFungibleBucket {
            let (component_address_reservation, component_address) =
                Runtime::allocate_component_address(PriceFeed::blueprint_id());

            let component_rule = rule!(require(global_caller(component_address)));

            let (admin_badge_address_reservation, admin_badge_address) =
                Runtime::allocate_non_fungible_address();

            let admin_rule = rule!(require(admin_badge_address));

            let admin_badge = ResourceBuilder::new_integer_non_fungible::<AuthBadgeData>(
                OwnerRole::Fixed(admin_rule),
            )
            .with_address(admin_badge_address_reservation)
            .mint_initial_supply([(IntegerNonFungibleLocalId::from(1), AuthBadgeData {})]);

            let admin_rule = rule!(require(admin_badge.resource_address()));

            let updater_badge_manager =
                ResourceBuilder::new_integer_non_fungible::<UpdaterBadgeData>(OwnerRole::Fixed(
                    admin_rule.clone(),
                ))
                .mint_roles(mint_roles! {
                    minter => component_rule.clone();
                    minter_updater =>  rule!(deny_all);
                })
                .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                  non_fungible_data_updater => component_rule;
                  non_fungible_data_updater_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            Self {
                prices: IndexMap::new(),
                feed: IndexMap::new(),
                updater_badge_manager,
                updater_counter: 0,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(admin_rule.clone()))
            .with_address(component_address_reservation)
            .roles(roles! {
                admin => admin_rule;
                updater => rule!(require(updater_badge_manager.address()));
            })
            .globalize();

            admin_badge
        }

        // * Admin Methods * //

        pub fn mint_updater_badge(&mut self, active: bool) -> Bucket {
            let badge_id = NonFungibleLocalId::Integer(self._get_new_id().into());

            self.updater_badge_manager
                .mint_non_fungible(&badge_id, UpdaterBadgeData { active })
        }

        pub fn update_updater_badge(&self, local_id: NonFungibleLocalId, active: bool) {
            self.updater_badge_manager
                .update_non_fungible_data(&local_id, "active", active);
        }

        pub fn admin_update_price(&mut self, resource: ResourceAddress, price: Decimal) {
            let now = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            self.prices.insert(
                resource,
                PriceInfo {
                    timestamp: now,
                    price,
                },
            );
        }

        pub fn admin_remove_price(&mut self, resource: ResourceAddress,) {
            self.prices.remove(&resource);
        }

        pub fn admin_update_feed(&mut self, resource: ResourceAddress, proofs: Vec<u8>, strategy: PriceFeedStrategy) {
           CP.verify_proofs_and_get_data(proofs.clone());
            self.feed.insert(
                resource,
                (proofs, strategy)
            );
        }

        // * Updater Methods * //

        pub fn update_price(
            &mut self,
            badge_proof: Proof,
            resource: ResourceAddress,
            price: Decimal,
        ) {
            let local_id = badge_proof
                .check(self.updater_badge_manager.address())
                .as_non_fungible()
                .non_fungible_local_id();

            let badge_data: UpdaterBadgeData =
                self.updater_badge_manager.get_non_fungible_data(&local_id);

            assert!(badge_data.active, "Updater badge is not active.");
            let now = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
            self.prices.insert(
                resource,
                PriceInfo {
                    timestamp: now,
                    price,
                },
            );
        }

        // * Public Methods * //

        pub fn get_price(&self, quote: ResourceAddress) -> Option<PriceInfo> {
            match self.prices.get(&quote) {
                Some(price_info) => Some(price_info.clone()),
                None => self.feed.get(&quote).and_then(|(proofs, strategy)| {
                    let now = Clock::current_time(TimePrecision::Minute).seconds_since_unix_epoch;
                    match strategy {
                        PriceFeedStrategy::DataPairs(data_pairs) => {
                            let price: Decimal = data_pairs.iter().map(|(pair_id_1, pair_id_2, operation)| {
                                let derived_data = CP.get_derived_svalue(pair_id_1.clone(), pair_id_2.clone(), operation.clone());
                                derived_data.derived_price
                            }).fold(1u128, |acc, price| acc * price).into();
                            Some(PriceInfo {
                                timestamp: now,
                                price
                            })
                        },
                        PriceFeedStrategy::PriceInfo(pair_index, operation) => {
                            match CP.verify_proofs_and_get_data(proofs.clone()).into_iter().find(|e| e.pair_index == *pair_index) {
                                Some(price_data) => {
                                    let price: Decimal = match operation {
                                        1 => 1/price_data.to_decimal_price(),
                                        _ => price_data.to_decimal_price()
                                    };
                                    Some(PriceInfo {
                                        timestamp: now,
                                        price
                                    })
                                }
                                None => None
                            } 
                        }
                    }
                })   
            }
        }

        // * Helpers * //

        fn _get_new_id(&mut self) -> u64 {
            self.updater_counter += 1;
            self.updater_counter
        }
    }
}
