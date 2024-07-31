use crate::modules::{
    cdp_data::*, cdp_health_checker::*, interest_strategy::*, liquidation_threshold::*,
    market_config::*, operation_status::*, pool_config::*, pool_state::*, utils::*,
};
use crate::resources::*;
use scrypto::prelude::*;



#[derive(ScryptoSbor)]
pub enum UpdateCDPInput {
    KeyImageURL(String),
    Name(String),
    Description(String),
}

#[blueprint]
#[types(ResourceAddress, CDPUpdatedEvenType, CDPLiquidable, CDPType, CollaterizedDebtPositionData, WrappedCDPData, PositionData, ExtendedCollateralPositionData, ExtendedLoanPositionData, CDPHealthChecker, InterestStrategyBreakPoints, InterestStrategy, UpdateLiquidationThresholdInput, LiquidationThreshold, UpdateMarketConfigInput, MarketConfig, OperatingStatus, PoolConfig, LendingPoolUpdatedEvent, MarketStatsPool, MarketStatsAllPools, LendingPoolState, WithdrawType, DepositType, PriceInfo)]
#[events(CDPUpdatedEvent, LendingPoolUpdatedEvent, CDPLiquidableEvent)]
mod lending_market {

    extern_blueprint!(
        //"package_sim1p4nk9h5kw2mcmwn5u2xcmlmwap8j6dzet7w7zztzz55p70rgqs4vag", // resim sdk
        "package_sim1pkc0e8f9yhlvpv38s2ymrplu7q366y3k8zc53zf2srlm7qm64fk043", // testing
        // "package_tdx_2_1p4p4wqvt58vz525uj444mgpfacx5cwzj20zqkmqt04f75qmx5mtc6r",  // stokenet
        SingleResourcePool {

            fn instantiate(
                pool_res_address: ResourceAddress,
                owner_role: OwnerRole,
                admin_rule: AccessRule,
                contribute_rule: AccessRule,
                redeem_rule: AccessRule,
            ) -> (Global<SingleResourcePool>, ResourceAddress);

            fn contribute(&self, assets: Bucket) -> Bucket;

            fn redeem(&self, pool_units: Bucket) -> Bucket;

            fn protected_deposit(&mut self, assets: Bucket, deposit_type: DepositType);

            fn protected_withdraw(
                &self,
                amount: Decimal,
                withdraw_type: WithdrawType,
                withdraw_strategy: WithdrawStrategy
            ) -> Bucket;

            fn increase_external_liquidity(&mut self, amount: Decimal);

            fn get_pool_unit_ratio(&self) -> PreciseDecimal;

            fn get_pooled_amount(&self) -> (Decimal, Decimal);

        }
    );

    enable_method_auth! {
        roles {
            admin => updatable_by: [];
            moderator => updatable_by: [];
            reserve_collector => updatable_by: [];
            liquidator => updatable_by: [];
        },

        methods {

            /* Admin methods */

            create_lending_pool => restrict_to: [admin];

            update_price_feed => restrict_to: [admin];
            update_market_config => restrict_to: [admin];
            update_pool_config => restrict_to: [admin];
            update_liquidation_threshold => restrict_to: [admin];
            update_interest_strategy => restrict_to: [admin];

            admin_update_operating_status => restrict_to: [admin];

            update_operating_status => restrict_to: [admin,moderator];

            update_pool_state => PUBLIC;

            /* Reserve Collector methods*/

            collect_reserve => restrict_to: [reserve_collector];

            /* User methods */

            // CDP Management methods

            create_cdp => PUBLIC;

            update_cdp => PUBLIC;

            show_cdp => PUBLIC;

            // Flashloan methods

            take_batch_flashloan => PUBLIC;
            repay_batch_flashloan => PUBLIC;

            // Lending and Borrowing methods

            contribute => PUBLIC;
            redeem => PUBLIC;

            add_collateral => PUBLIC;
            remove_collateral => PUBLIC;
            borrow => PUBLIC;
            repay => PUBLIC;

            // Liquidation methods
            mint_liquidator_badge => restrict_to: [admin];
            list_liquidable_cdps => PUBLIC;
            refinance => PUBLIC;
            check_cdp_for_liquidation => PUBLIC;
            start_liquidation => restrict_to: [admin,liquidator];
            end_liquidation => restrict_to: [admin,liquidator];
            fast_liquidation => restrict_to: [admin,liquidator];

            // Statistics queries
            list_info_stats => PUBLIC;
        }

    }

    macro_rules! save_cdp_macro {
        ($self:expr,$cdp:expr) => {
            $cdp.save_cdp(&$self.cdp_res_manager, $self.market_config.max_cdp_position)
                .expect("Error saving CDP");
        };
    }

    macro_rules! emit_cdp_event {
        ($cdp_id:expr,$event_type:expr) => {
            Runtime::emit_event(CDPUpdatedEvent {
                cdp_id: $cdp_id,
                event_type: $event_type,
            });
        };
    }

    struct LendingMarket {
        /// Save the admin rule for lending pool creation
        admin_rule: AccessRule,

        ///
        cdp_res_manager: ResourceManager,

        ///
        cdp_counter: u64,

        ///
        liquidator_counter: u64,

        /// Current lending market component address
        market_component_address: ComponentAddress,

        ///
        pool_unit_refs: IndexMap<ResourceAddress, ResourceAddress>,

        ///
        reverse_pool_unit_refs: IndexMap<ResourceAddress, ResourceAddress>,

        ///
        listed_assets: IndexSet<ResourceAddress>,

        ///
        pool_states: KeyValueStore<ResourceAddress, LendingPoolState>,

        ///
        transient_res_manager: ResourceManager,

        ///
        liquidator_badge_manager: ResourceManager,

        ///
        operating_status: OperatingStatus,

        ///
        market_config: MarketConfig,
    }

    impl LendingMarket {
        pub fn instantiate(market_config: MarketConfig) -> (NonFungibleBucket, NonFungibleBucket) {
            // Check inputs
            market_config.check().expect("Invalid market config");

            // Get address reservation for the lending market component
            let (market_component_address_reservation, market_component_address) =
                Runtime::allocate_component_address(LendingMarket::blueprint_id());
            let component_rule = rule!(require(global_caller(market_component_address)));

            // * Create admin badge * //

            // Get address reservation for the admin badge resource address
            let (admin_badge_address_reservation, admin_badge_address) =
                Runtime::allocate_non_fungible_address();

            // Admin will be able to create lending pools, update pool configurations and update operating status
            let admin_rule = rule!(require_amount(dec!(4), admin_badge_address));

            // Moderator will be able to update operating status if the last update was not done by an admin
            let modarator_rule = rule!(require_amount(dec!(2), admin_badge_address));

            let admin_badge =
                create_admin_badge(admin_rule.clone(), admin_badge_address_reservation);

            // * Create reserve collector badge * //

            let reserve_collector_badge = create_reserve_collector_badge(admin_rule.clone());
            let reserve_collector_rule = rule!(require(reserve_collector_badge.resource_address()));
            
            // * Create CDP resource manager * //
            let cdp_res_manager =
                create_cdp_res_manager(admin_rule.clone(), component_rule.clone());

            // * Create transient resource manager * //
            let transient_res_manager =
                create_transient_res_manager(admin_rule.clone(), component_rule.clone());

            // * Create liquidator badge manager * //
            let liquidator_badge_manager = create_liquidator_badge_manager(admin_rule.clone(), component_rule);

            // *  Instantiate our component with the previously created resources and addresses * //
            Self {
                market_component_address,
                cdp_res_manager,
                admin_rule: admin_rule.clone(),
                cdp_counter: 0,
                liquidator_counter: 0,
                transient_res_manager,
                liquidator_badge_manager,
                pool_unit_refs: IndexMap::new(),
                reverse_pool_unit_refs: IndexMap::new(),
                pool_states: KeyValueStore::<ResourceAddress, LendingPoolState>::new_with_registered_type(),
                listed_assets: IndexSet::new(),
                operating_status: OperatingStatus::new(),
                market_config,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(market_component_address_reservation)
            .roles(roles! {
                admin => admin_rule.clone();
                moderator => modarator_rule;
                reserve_collector => reserve_collector_rule;
                liquidator => rule!(require(liquidator_badge_manager.address()));
            })
            .metadata(metadata!(
                roles {
                    metadata_setter => admin_rule.clone();
                    metadata_setter_updater => rule!(deny_all);
                    metadata_locker => admin_rule;
                    metadata_locker_updater => rule!(deny_all);
                }
            ))
            .globalize();

            (admin_badge, reserve_collector_badge)
        }

        /*
        POOL MANAGEMENT METHODS
        */

        pub fn create_lending_pool(
            &mut self,
            price_feed_component: Global<AnyComponent>,
            pool_res_address: ResourceAddress,
            pool_config: PoolConfig,
            interest_strategy_break_points: InterestStrategyBreakPoints,
            liquidation_threshold: LiquidationThreshold,
        ) {
            assert!(
                self.listed_assets.get(&pool_res_address).is_none(),
                "The lending pool is already registered"
            );

            let res_manager: ResourceManager = pool_res_address.into();

            let recaller_role = res_manager
                .get_role("recaller")
                .expect("Error getting recaller role");

            let recaller_updater_role = res_manager
                .get_role("recaller_updater")
                .expect("Error getting recaller_updater role");

            assert!(
                recaller_role == AccessRule::DenyAll
                    && recaller_updater_role == AccessRule::DenyAll,
                "Recallable assets are not supported"
            );

            liquidation_threshold
                .check()
                .expect("Invalid liquidation threshold");

            pool_config.check().expect("Invalid pool config");

            let component_rule = rule!(require(global_caller(self.market_component_address)));

            let (pool, pool_unit_res_address) = Blueprint::<SingleResourcePool>::instantiate(
                pool_res_address,
                OwnerRole::Fixed(component_rule.clone()),
                component_rule.clone(),
                component_rule.clone(),
                component_rule,
            );

            let pool_unit_res_manager = ResourceManager::from_address(pool_unit_res_address);

            let pool_res_symbol: String = ResourceManager::from_address(pool_res_address)
            .get_metadata("symbol").expect("Pool resource symbol not provided").unwrap_or_default();

            pool_unit_res_manager.set_metadata("name", format!("{pool_res_symbol} rtToken"));
            pool_unit_res_manager.lock_metadata("name");
            pool_unit_res_manager.set_metadata("symbol", format!("rt{pool_res_symbol}"));
            pool_unit_res_manager.lock_metadata("symbol");

            let mut interest_strategy = InterestStrategy::new();

            // set_breakpoints will check the validity of the breakpoints
            interest_strategy
                .set_breakpoints(interest_strategy_break_points)
                .expect("Invalid interest strategy breakpoints");

            let last_price_info =
                get_price(price_feed_component, pool_res_address).expect("Price not found");

            let pool_state = LendingPoolState {
                pool,
                collaterals: Vault::new(pool_unit_res_address),
                reserve: Vault::new(pool_res_address),
                pool_res_address,

                price: last_price_info.price,

                price_updated_at: Clock::current_time(TimePrecision::Second)
                    .seconds_since_unix_epoch,

                total_loan: 0.into(),
                total_deposit: 0.into(),
                total_loan_unit: 0.into(),
                total_deposit_unit: 0.into(),
                total_reserved_amount: 0.into(),
                interest_rate: 0.into(),
                interest_updated_at: Clock::current_time(TimePrecision::Second)
                    .seconds_since_unix_epoch,

                price_feed_comp: price_feed_component,
                interest_strategy,
                liquidation_threshold,
                pool_config,
                operating_status: OperatingStatus::new(),
                pool_utilization: 0.into()
            };

            //
            self.pool_states.insert(pool_res_address, pool_state);

            //
            self.reverse_pool_unit_refs
                .insert(pool_unit_res_address, pool_res_address);

            self.pool_unit_refs
                .insert(pool_res_address, pool_unit_res_address);

            self.listed_assets.insert(pool_res_address);
        }

        // Collect reserve retention from all pools
        pub fn collect_reserve(&mut self) -> Vec<(Decimal, Bucket)> {
            let listed_assets = self.listed_assets.clone();

            listed_assets
                .iter()
                .map(|pool_res_address| {
                    let mut pool_state = self._get_pool_state(pool_res_address, None, None);

                    let price = pool_state.price;

                    let fee = pool_state.reserve.take_all();

                    (price, fee)
                })
                .collect()
        }

        pub fn update_price_feed(
            &mut self,
            pool_res_address: ResourceAddress,
            price_feed: Global<AnyComponent>,
        ) {
            let mut pool_state = self._get_pool_state(&pool_res_address, None, None);

            get_price(price_feed, pool_res_address).expect("Price not found");

            pool_state.price_feed_comp = price_feed;
        }

        pub fn update_liquidation_threshold(
            &mut self,
            pool_res_address: ResourceAddress,
            value: UpdateLiquidationThresholdInput,
        ) {
            let mut pool_state = self._get_pool_state(&pool_res_address, None, None);

            pool_state
                .liquidation_threshold
                .update_liquidation_threshold(value)
                .expect("Invalid liquidation threshold");
        }

        pub fn update_interest_strategy(
            &mut self,
            pool_res_address: ResourceAddress,
            interest_strategy_break_points: InterestStrategyBreakPoints,
        ) {
            let mut pool_state = self._get_pool_state(&pool_res_address, None, None);

            pool_state
                .interest_strategy
                .set_breakpoints(interest_strategy_break_points)
                .expect("Invalid interest strategy breakpoints");
        }

        pub fn update_market_config(&mut self, value: UpdateMarketConfigInput) {
            self.market_config
                .update(value)
                .expect("Invalid market config");
        }

        pub fn update_pool_config(
            &mut self,
            pool_res_address: ResourceAddress,
            value: UpdatePoolConfigInput,
        ) {
            let mut pool_state = self._get_pool_state(&pool_res_address, None, None);

            pool_state
                .pool_config
                .update(value)
                .expect("Invalid pool config");
        }

        pub fn update_pool_state(
            &mut self,
            pool_res_address: ResourceAddress,
            bypass_price_debounce: bool,
            bypass_interest_debounce: bool,
        ) {
            self._get_pool_state(
                &pool_res_address,
                None,
                Some((bypass_price_debounce, bypass_interest_debounce)),
            );
        }

        ///
        fn _update_operating_status(
            &mut self,
            value: OperatingService,
            enabled: bool,
            set_by_admin: bool,
            pool_res_address: Option<ResourceAddress>,
        ) -> Result<(), String> {
            match pool_res_address {
                Some(pool_res_address) => {
                    let mut pool_state = self._get_pool_state(&pool_res_address, None, None);

                    pool_state
                        .operating_status
                        .update(value, enabled, set_by_admin)
                }
                None => self.operating_status.update(value, enabled, set_by_admin),
            }
        }

        // Update the operating status of the lending market or a specific pool
        // Update made by a moderator can be reverted by an admin
        pub fn update_operating_status(
            &mut self,
            value: OperatingService,
            enabled: bool,
            pool_res_address: Option<ResourceAddress>,
        ) {
            self._update_operating_status(value, enabled, false, pool_res_address)
                .expect("Error updating operating status by a moderator")
        }

        // Update the operating status of the lending market or a specific pool with admin flag
        // Update made by an admin will not be reverted by a moderator
        pub fn admin_update_operating_status(
            &mut self,
            value: OperatingService,
            enabled: bool,
            pool_res_address: Option<ResourceAddress>,
        ) {
            self._update_operating_status(value, enabled, true, pool_res_address)
                .expect("Error updating operating status by an admin")
        }

        ///*  CDP CREATION AND MANAGEMENT METHODS * ///

        pub fn list_liquidable_cdps(&self, skip: u64, limit: u64) -> Vec<CDPLiquidable> {
            let mut results = vec![];
            // Logger::debug(format!("self.cdp_counter  {}", self.cdp_counter ));
            for cdp_id in skip..limit.min(self.cdp_counter + 1) {
                let cdp_id = &NonFungibleLocalId::Integer(cdp_id.into());
                // Logger::debug(format!("Search cdp {} exists= {}", cdp_id, self.cdp_res_manager.non_fungible_exists(cdp_id)));
                if self.cdp_res_manager.non_fungible_exists(cdp_id) {
                    let cdp_data = WrappedCDPData::new(&self.cdp_res_manager, cdp_id);
                    if !cdp_data.cdp_data.collaterals.is_empty() && !cdp_data.cdp_data.loans.is_empty() {
                        let mut cdp_health_checker = CDPHealthChecker::new_without_update(
                            &cdp_data,
                            &self.pool_states,
                        );

                        cdp_health_checker.update_health_check_data().expect(&format!("Error updating health check data for cdp {}", &cdp_data.cdp_id));

                        if cdp_health_checker.can_liquidate().is_ok() {
                            results.push(CDPLiquidable {
                                cdp_id: cdp_data.cdp_id,
                                cdp_data: cdp_data.cdp_data,
                            });
                        }
                    }
                }
            }
            Runtime::emit_event(CDPLiquidableEvent { cdps: results.clone() });
            results
        }

        pub fn create_cdp(
            &mut self,
            name: Option<String>,
            description: Option<String>,
            key_image_url: Option<String>,
            deposits: Vec<Bucket>,
        ) -> Bucket {
            let cdp_id = NonFungibleLocalId::Integer(self._get_new_cdp_id().into());

            let now = Clock::current_time(TimePrecision::Second).seconds_since_unix_epoch;

            let data = CollaterizedDebtPositionData {
                name: name.unwrap_or("".into()),
                description: description.unwrap_or("".into()),
                key_image_url: key_image_url.unwrap_or("".into()),
                cdp_type: CDPType::Standard,
                collaterals: IndexMap::new(),
                loans: IndexMap::new(),
                liquidated: IndexMap::new(),
                minted_at: now,
                updated_at: now,
                liquidable: None,
            };

            let cdp = self.cdp_res_manager.mint_non_fungible(&cdp_id, data);

            if !deposits.is_empty() {
                self._add_collateral_internal(cdp_id, deposits);
            }

            cdp
        }

        pub fn update_cdp(&mut self, cdp_proof: Proof, value: UpdateCDPInput) {
            let cdp_id = self._validate_cdp_proof(cdp_proof);

            match value {
                UpdateCDPInput::KeyImageURL(key_image_url) => {
                    self.cdp_res_manager.update_non_fungible_data(
                        &cdp_id,
                        "key_image_url",
                        key_image_url,
                    );
                }
                UpdateCDPInput::Name(name) => {
                    self.cdp_res_manager
                        .update_non_fungible_data(&cdp_id, "name", name);
                }
                UpdateCDPInput::Description(description) => {
                    self.cdp_res_manager.update_non_fungible_data(
                        &cdp_id,
                        "description",
                        description,
                    );
                }
            }

            self.cdp_res_manager.update_non_fungible_data(
                &cdp_id,
                "updated_at",
                Clock::current_time(TimePrecision::Second).seconds_since_unix_epoch,
            );
        }

        pub fn show_cdp(&self, cdp_id: u64) -> Option<WrappedCDPData> {
            let cdp_id = &NonFungibleLocalId::Integer(cdp_id.into());
            if self.cdp_res_manager.non_fungible_exists(cdp_id) {
                let cdp_data = WrappedCDPData::new(&self.cdp_res_manager, cdp_id);
                Some(cdp_data)
            } else {
                None
            }
        }

        // / * Flashloan methods * ///

        pub fn take_batch_flashloan(
            &mut self,
            loan_amounts: IndexMap<ResourceAddress, Decimal>,
        ) -> (Vec<Bucket>, Bucket) {
            self._check_operating_status(OperatingService::Flashloan);

            let mut loans: Vec<Bucket> = Vec::new();
            let mut terms: IndexMap<ResourceAddress, BatchFlashloanItem> = IndexMap::new();

            for (pool_res_address, amount) in loan_amounts.iter() {
                assert!(
                    amount >= &Decimal::ZERO,
                    "INVALID_INPUT: borrowed amount must be greater than zero"
                );

                let pool_state = self
                    .pool_states
                    .get(pool_res_address)
                    .expect("Pool state not found for provided resource");

                pool_state
                    .check_operating_status(OperatingService::Flashloan)
                    .expect("Flashloan is not enabled for the pool");

                let fee_amount = (*amount) * pool_state.pool_config.flashloan_fee_rate;

                let loan_term = BatchFlashloanItem {
                    fee_amount,
                    loan_amount: *amount,
                    paid_back: false,
                };

                let loan = pool_state.pool.protected_withdraw(
                    *amount,
                    WithdrawType::TemporaryUse,
                    WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
                );

                loans.push(loan);
                terms.insert(*pool_res_address, loan_term);
            }

            (
                loans,
                self.transient_res_manager
                    .mint_ruid_non_fungible(TransientResData {
                        data: TransientResDataType::BatchFlashloanItem(terms),
                    }),
            )
        }

        pub fn repay_batch_flashloan(
            &mut self,
            payments: Vec<Bucket>,
            batch_loan_term: Bucket,
        ) -> Vec<Bucket> {
            let mut remainders: Vec<Bucket> = Vec::new();

            let transient_res_data: TransientResData =
                batch_loan_term.as_non_fungible().non_fungible().data();

            let mut batch_loan_term_data = match transient_res_data.data {
                TransientResDataType::BatchFlashloanItem(batch_loan_term_data) => {
                    batch_loan_term_data
                }
                _ => panic!("Invalid transient resource data"),
            };

            for mut payment in payments {
                let pool_res_address = payment.resource_address();

                let loan_term = batch_loan_term_data
                    .get_mut(&pool_res_address)
                    .expect("flash loan term not found for provided resource");

                if loan_term.paid_back {
                    remainders.push(payment);
                    continue;
                }

                let due_amount = loan_term.fee_amount + loan_term.loan_amount;

                assert!(
                    payment.amount() >= (due_amount),
                    "Insufficient repayment given for your loan!"
                );

                let mut pool_state = self
                    .pool_states
                    .get_mut(&pool_res_address)
                    .expect("Pool state not found for provided resource");

                pool_state.pool.protected_deposit(
                    payment.take_advanced(
                        loan_term.loan_amount,
                        WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
                    ),
                    DepositType::FromTemporaryUse,
                );

                let protocol_fee_amount =
                    loan_term.fee_amount * pool_state.pool_config.protocol_flashloan_fee_rate;

                let lp_fee_amount = loan_term.fee_amount - protocol_fee_amount;

                pool_state.pool.protected_deposit(
                    payment.take_advanced(
                        lp_fee_amount,
                        WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
                    ),
                    DepositType::LiquiditySupply,
                );

                pool_state.reserve.put(payment.take_advanced(
                    protocol_fee_amount,
                    WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
                ));

                loan_term.paid_back = true;

                remainders.push(payment);
            }

            let all_paid_back = batch_loan_term_data
                .iter()
                .all(|(_, loan_term)| loan_term.paid_back);

            assert!(all_paid_back, "Not all loans are paid back");

            self.transient_res_manager.burn(batch_loan_term);

            remainders
        }

        //* Lending and Borrowing methods * //

        pub fn contribute(&mut self, assets: Bucket) -> Bucket {
            self._check_operating_status(OperatingService::Contribute);

            let mut pool_state = self._get_pool_state(
                &assets.resource_address(),
                Some(OperatingService::Contribute),
                None,
            );

            pool_state
                .contribute_proxy(assets)
                .expect("Error contributing to pool")
        }

        pub fn redeem(&mut self, pool_units: Bucket) -> Bucket {
            self._check_operating_status(OperatingService::Redeem);

            let pool_res_address = *self
                .reverse_pool_unit_refs
                .get(&pool_units.resource_address())
                .expect("Pool unit not found");

            self._get_pool_state(&pool_res_address, Some(OperatingService::Redeem), None)
                .redeem_proxy(pool_units, false)
        }

        pub fn add_collateral(&mut self, cdp_proof: Proof, deposits: Vec<Bucket>) {
            let cdp_id = self._validate_cdp_proof(cdp_proof);

            self._add_collateral_internal(cdp_id.clone(), deposits);

            emit_cdp_event!(cdp_id, CDPUpdatedEvenType::AddCollateral);
        }

        pub fn remove_collateral(
            &mut self,
            cdp_proof: Proof,
            withdraw_details: Vec<(ResourceAddress, Decimal, bool)>,
        ) -> Vec<Bucket> {
            self._check_operating_status(OperatingService::RemoveCollateral);

            let cdp_id = self._validate_cdp_proof(cdp_proof);

            let mut cdp_data = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            let withdrawals = withdraw_details.into_iter().fold(
                Vec::new(),
                |mut withdrawals, (pool_res_address, unit_amount, keep_deposit_unit)| {
                    let mut pool_state = self._get_pool_state(
                        &pool_res_address,
                        Some(OperatingService::RemoveCollateral),
                        None,
                    );

                    let current_deposit_units = cdp_data.get_collateral_units(pool_res_address);

                    let withdraw_collateral_units = current_deposit_units.min(unit_amount.into());

                    cdp_data
                        .update_collateral(pool_res_address, -withdraw_collateral_units)
                        .expect("Error updating collateral for CDP");

                    let deposit_units = pool_state
                        .remove_pool_units_from_collateral(withdraw_collateral_units)
                        .expect("Error redeeming pool units from collateral");

                    let returned_assets = if !keep_deposit_unit {
                        pool_state.redeem_proxy(deposit_units, false)
                    } else {
                        deposit_units
                    };

                    withdrawals.push(returned_assets);

                    withdrawals
                },
            );

            CDPHealthChecker::new(
                &cdp_data,
                &mut self.pool_states,
            )
            .check_cdp()
            .expect("Error checking CDP");

            save_cdp_macro!(self, cdp_data);

            emit_cdp_event!(cdp_id, CDPUpdatedEvenType::RemoveCollateral);

            withdrawals
        }

        pub fn borrow(
            &mut self,
            cdp_proof: Proof,
            borrows: Vec<(ResourceAddress, Decimal)>,
        ) -> Vec<Bucket> {
            self._check_operating_status(OperatingService::Borrow);

            let cdp_id = self._validate_cdp_proof(cdp_proof);

            let mut cdp_data = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            let loans =
                borrows
                    .into_iter()
                    .fold(Vec::new(), |mut loans, (pool_res_address, amount)| {
                        let mut pool_state = self._get_pool_state(
                            &pool_res_address,
                            Some(OperatingService::Borrow),
                            None,
                        );

                        let (borrowed_assets, delta_loan_units) = pool_state
                            .withdraw_for_borrow(amount)
                            .expect("Error in withdraw_for_borrow");

                        cdp_data
                            .update_loan(pool_res_address, delta_loan_units.into())
                            .expect("Error updating loan");

                        loans.push(borrowed_assets);

                        loans
                    });

            CDPHealthChecker::new(
                &cdp_data,
                &mut self.pool_states,
            )
            .check_cdp()
            .expect("Error checking CDP");

            save_cdp_macro!(self, cdp_data);

            emit_cdp_event!(cdp_id, CDPUpdatedEvenType::Borrow);

            loans
        }

        pub fn repay(
            &mut self,
            cdp_proof: Proof,
            _delegatee_cdp_id: Option<NonFungibleLocalId>,
            payments: Vec<Bucket>,
        ) -> (Vec<Bucket>, Decimal) {
            self._check_operating_status(OperatingService::Repay);

            let cdp_id = self._validate_cdp_proof(cdp_proof);

            let mut cdp_data = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            if cdp_data.cdp_data.collaterals.is_empty() {
                panic!("Position was liquidated");
            }

            let (remainders, payment_value) = self._repay_internal(
                &mut cdp_data,
                payments,
                None,
                false,
            );

            emit_cdp_event!(cdp_id, CDPUpdatedEvenType::Repay);

            (remainders, payment_value)
        }

        pub fn refinance(
            &mut self,
            cdp_id: NonFungibleLocalId,
            payments: Vec<Bucket>,
        ) -> (Vec<Bucket>, Decimal) {
            let mut cdp_data = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            CDPHealthChecker::new(
                &cdp_data,
                &mut self.pool_states,
            )
            .can_refinance()
            .expect("Error checking CDP");

            let (remainders, payment_value) = self._repay_internal(
                &mut cdp_data,
                payments,
                None,
                false,
            );

            emit_cdp_event!(cdp_id, CDPUpdatedEvenType::Refinance);

            (remainders, payment_value)
        }

        pub fn start_liquidation(
            &mut self,
            cdp_id: NonFungibleLocalId,
            requested_collaterals: Vec<ResourceAddress>,
            total_payment_value: Option<Decimal>,
        ) -> (Vec<Bucket>, Bucket) {
            self._check_operating_status(OperatingService::Liquidation);

            if let Some(total_payment_value) = total_payment_value {
                assert!(
                    total_payment_value >= 0.into(),
                    "INVALID_INPUT: Total payment value must be non-negative"
                );
            }

            let mut cdp_data = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            let is_within_minute = Clock::current_time_is_strictly_before(Instant::new(cdp_data.cdp_data.updated_at).add_minutes(1).unwrap(), TimePrecision::Second);
            if !is_within_minute {
                panic!("cdp info is too old.");
            }
            let self_closable_loan_value = match cdp_data.cdp_data.liquidable {
                Some(self_closable_loan_value) => self_closable_loan_value,
                None => panic!("The cdp is not liquidable.")
            };

            let temp_total_payment_value = total_payment_value
                .unwrap_or(self_closable_loan_value)
                .min(self_closable_loan_value);

            let (returned_collaterals, total_payement_value) = self
                ._remove_collateral_for_liquidation(
                    &mut cdp_data,
                    requested_collaterals,
                    temp_total_payment_value,
                    false,
                );

            let liquidation_term =
                self.transient_res_manager
                    .mint_ruid_non_fungible(TransientResData {
                        data: TransientResDataType::LiquidationTerm(LiquidationTerm {
                            cdp_id,
                            payement_value: total_payement_value,
                        }),
                    });

            (returned_collaterals, liquidation_term)
        }

        pub fn end_liquidation(
            &mut self,
            payments: Vec<Bucket>,
            liquidation_term: Bucket,
        ) -> (Vec<Bucket>, Decimal) {
            let transient_data: TransientResData =
                liquidation_term.as_non_fungible().non_fungible().data();

            let liquidation_term_data = match transient_data.data {
                TransientResDataType::LiquidationTerm(liquidation_term_data) => {
                    liquidation_term_data
                }
                _ => panic!("Invalid transient resource data"),
            };

            let cdp_id = liquidation_term_data.cdp_id;

            let mut cdp_data = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            let (remainders, total_payment_value) = self._repay_internal(
                &mut cdp_data,
                payments,
                Some(liquidation_term_data.payement_value),
                true,
            );

            assert!(
                (total_payment_value - liquidation_term_data.payement_value).checked_abs().unwrap() < ZERO_EPSILON,
                "Total payment value {} does not match with the liquidation term {}",
                total_payment_value, liquidation_term_data.payement_value
            );

            assert!(
                total_payment_value > liquidation_term_data.payement_value * self.market_config.max_liquidable_value,
                "Total payment value {} exceeds the max liquidable value {}",
                total_payment_value, liquidation_term_data.payement_value * self.market_config.max_liquidable_value
            );

            self.transient_res_manager.burn(liquidation_term);

            cdp_data.on_liquidation().expect("perform cdp liquidation tasks");

            save_cdp_macro!(self, cdp_data);

            emit_cdp_event!(cdp_id, CDPUpdatedEvenType::Liquidate);

            (remainders, total_payment_value)
        }

        pub fn mint_liquidator_badge(&mut self) -> Bucket {
            let badge_id = NonFungibleLocalId::Integer(self._get_new_liquidator_id().into());

            self.liquidator_badge_manager
                .mint_non_fungible(&badge_id, LiquidatorBadgeData {})
        }

        pub fn check_cdp_for_liquidation(&mut self, cdp_id: NonFungibleLocalId) -> bool {
            let mut cdp_data: WrappedCDPData = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            let mut cdp_health_checker = CDPHealthChecker::new(
                &cdp_data,
                &mut self.pool_states,
            );

            let can_liquidate = cdp_health_checker.can_liquidate().is_ok();
            match cdp_data.cdp_data.liquidable {
                Some(_) => {
                    if can_liquidate {
                        cdp_data.cdp_data.liquidable = Some(cdp_health_checker.self_closable_loan_value);
                    } else {
                        cdp_data.cdp_data.liquidable = None;
                    }
                }, 
                None => {
                    if can_liquidate {
                        cdp_data.cdp_data.liquidable = Some(cdp_health_checker.self_closable_loan_value);
                    }
                }
            }
            save_cdp_macro!(self, cdp_data);    
            can_liquidate
        }

        pub fn fast_liquidation(
            &mut self,
            cdp_id: NonFungibleLocalId,
            payments: Vec<Bucket>,
            requested_collaterals: Vec<ResourceAddress>,
        ) -> (Vec<Bucket>, Vec<Bucket>, Decimal) {
            self._check_operating_status(OperatingService::Liquidation);

            let mut cdp_data = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            let is_within_minute = Clock::current_time_is_strictly_before(Instant::new(cdp_data.cdp_data.updated_at).add_minutes(1).unwrap(), TimePrecision::Second);
            if cdp_data.cdp_data.liquidable.is_some() && !is_within_minute {
                panic!("cdp info is too old");
            } else if cdp_data.cdp_data.liquidable.is_none() && !is_within_minute {
                panic!("The cdp is not liquidable and cdp info is too old.")
            } else if cdp_data.cdp_data.liquidable.is_none() {
                panic!("The cdp is not liquidable.")
            }

            let (remainders, total_payment_value) =
                self._repay_internal(&mut cdp_data, payments, None, true);

            let (returned_collaterals, _total_payement_value) = self
                ._remove_collateral_for_liquidation(
                    &mut cdp_data,
                    requested_collaterals,
                    total_payment_value,
                    true
                );

            cdp_data.on_liquidation().expect("perform cdp liquidation tasks");

            save_cdp_macro!(self, cdp_data);

            emit_cdp_event!(cdp_id, CDPUpdatedEvenType::Liquidate);

            (remainders, returned_collaterals, total_payment_value)
        }

        //*  PUBLIC QUERIES   *//

        pub fn list_info_stats(&self) -> MarketStatsAllPools {
            let second_per_year = 31536000;
            let mut total_supply_all_pools = Decimal::zero();
            let mut total_borrow_all_pools = Decimal::zero();

            let market_stats = self.listed_assets
                .clone()
                .into_iter()
                .map(|asset| {
                    let pool_state_ref = self.pool_states.get(&asset).unwrap();
                    let pool_state = pool_state_ref;
                    let utilization_rate = pool_state.get_pool_utilization();

                    let deposit_rate =
                        utilization_rate *
                        pool_state.interest_rate *
                        (1 - pool_state.pool_config.protocol_interest_fee_rate);

                    let supply_apy_term: PreciseDecimal = PreciseDecimal::ONE + deposit_rate / second_per_year;
                    let supply_apy =
                        supply_apy_term.checked_powi(second_per_year).unwrap() - dec!(1);

                    let borrow_apy_term: PreciseDecimal =
                        pdec!(1) + pool_state.interest_rate / second_per_year;
                    let borrow_apy =
                        borrow_apy_term.checked_powi(second_per_year).unwrap() - dec!(1);

                    let pooled_amount = pool_state.pool.get_pooled_amount();
                    let available_liquidity = pooled_amount.0;
                    let total_liquidity = pooled_amount.0 + pooled_amount.1;

                    let total_borrow = pool_state.total_loan
                        .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                        .unwrap();

                    let total_supply = pool_state.total_deposit
                        .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                        .unwrap();

                    total_supply_all_pools += total_supply;
                    total_borrow_all_pools += total_borrow;

                    MarketStatsPool {
                        asset_address: pool_state.pool_res_address,
                        available_liquidity,
                        total_liquidity,
                        total_supply,
                        total_borrow,
                        supply_apy,
                        borrow_apy,
                        deposit_limit: pool_state.pool_config.deposit_limit,
                        borrow_limit: pool_state.pool_config.borrow_limit,
                        utilization_limit: pool_state.pool_config.utilization_limit,
                        optimal_usage: pool_state.pool_config.optimal_usage,
                        ltv_limit: pool_state.liquidation_threshold.default_value,
                    }
                })
                .collect::<Vec<MarketStatsPool>>();

            let market_total_stats = MarketStatsAllPools {
                total_supply_all_pools,
                total_borrow_all_pools,
                market_stats_pools: market_stats,
            };
            market_total_stats
        }

        //*  PRIVATE UTILITY METHODS   *//

        fn _add_collateral_internal(&mut self, cdp_id: NonFungibleLocalId, deposits: Vec<Bucket>) {
            self._check_operating_status(OperatingService::AddCollateral);

            let mut cdp_data = WrappedCDPData::new(&self.cdp_res_manager, &cdp_id);

            deposits.into_iter().fold((), |_, assets| {
                let res_address = assets.resource_address();

                let value = self.pool_unit_refs.get(&res_address);

                let (pool_res_address, pool_unit_res_address) = if let Some(value) = value {
                    (res_address, *value)
                } else {
                    (
                        *self.reverse_pool_unit_refs.get(&res_address).unwrap(),
                        res_address,
                    )
                };

                let mut pool_state = self._get_pool_state(
                    &pool_res_address,
                    Some(OperatingService::AddCollateral),
                    None,
                );

                let deposit_units = if res_address == pool_unit_res_address {
                    assets
                } else {
                    pool_state
                        .contribute_proxy(assets)
                        .expect("Error contributing to pool")
                };

                cdp_data
                    .update_collateral(pool_res_address, deposit_units.amount().into())
                    .expect("Error updating collateral for CDP");

                pool_state
                    .add_pool_units_as_collateral(deposit_units)
                    .expect("Error adding pool units as collateral");
            });

            save_cdp_macro!(self, cdp_data);
        }

        fn _remove_collateral_for_liquidation(
            &mut self,
            cdp_data: &mut WrappedCDPData,
            requested_collaterals: Vec<ResourceAddress>,
            requested_collaterals_value: Decimal,
            check_requested_collaterals: bool
        ) -> (Vec<Bucket>, Decimal) {
            let mut returned_collaterals: Vec<Bucket> = Vec::new();
            let mut returned_collaterals_value = dec!(0);

            let mut temp_requested_value = requested_collaterals_value;

            for pool_res_address in requested_collaterals {
                // Make sure that that each requested collateral will have a bucket in the worktop
                if temp_requested_value == dec!(0) {
                    // returned_collaterals.push(Bucket::new(pool_res_address));
                    break;
                }

                let mut pool_state =  self._get_pool_state_without_update(
                    &pool_res_address,
                    Some(OperatingService::Liquidation)
                );

                let bonus_rate = dec!(1) + pool_state.pool_config.liquidation_bonus_rate;

                let unit_ratio = pool_state.pool.get_pool_unit_ratio();

                let max_collateral_units = cdp_data.get_collateral_units(pool_res_address);

                let max_collateral_amount = (max_collateral_units / unit_ratio)
                    .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                    .unwrap();

                let mut max_collateral_value = max_collateral_amount * pool_state.price;

                max_collateral_value = max_collateral_value.min(bonus_rate * temp_requested_value);

                if max_collateral_value >= (bonus_rate * temp_requested_value) {
                    temp_requested_value = dec!(0);
                } else {
                    temp_requested_value -= max_collateral_value / bonus_rate;
                }

                returned_collaterals_value += max_collateral_value * (1 - pool_state.pool_config.protocol_liquidation_fee_rate + pool_state.pool_config.liquidation_bonus_rate);

                let collateral_units = (max_collateral_value / pool_state.price) * unit_ratio;

                cdp_data
                    .update_collateral(pool_res_address, -collateral_units)
                    .expect("Error updating collateral for CDP");

                let pool_unit = pool_state
                    .remove_pool_units_from_collateral(collateral_units)
                    .expect("Error redeeming pool units from collateral");

                let mut collaterals = pool_state.redeem_proxy(pool_unit, true);
                let protocol_fee_amount = collaterals.amount()
                    * pool_state.pool_config.protocol_liquidation_fee_rate;

                pool_state.reserve.put(collaterals.take_advanced(
                    protocol_fee_amount,
                    WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
                ));

                returned_collaterals.push(collaterals);
            }

            if check_requested_collaterals {
                assert!(
                    temp_requested_value == dec!(0),
                    "Insufficient collateral value, {} remaining",
                    temp_requested_value
                );
            }
            save_cdp_macro!(self, cdp_data);

            (returned_collaterals, returned_collaterals_value)
        }

        fn _repay_internal(
            &mut self,
            cdp_data: &mut WrappedCDPData,
            payments: Vec<Bucket>,
            payment_value: Option<Decimal>,
            for_liquidation: bool
        ) -> (Vec<Bucket>, Decimal) {
            let mut expected_payment_value = payment_value.unwrap_or(dec!(0));

            let (mut remainders, mut total_payment_value) = (Vec::new(), PreciseDecimal::zero());
            for mut payment in payments {
                let pool_res_address = payment.resource_address();

                let mut pool_state = if for_liquidation {
                    self._get_pool_state_without_update(&pool_res_address, None)
                } else {
                    self._get_pool_state(&pool_res_address, None, None)
                };

                // ! Liquidation
                if for_liquidation {
                    pool_state
                        .check_operating_status(OperatingService::Liquidation)
                        .expect("Liquidation is not enabled for the pool");

                // ! Repay
                } else {
                    pool_state
                        .check_operating_status(OperatingService::Repay)
                        .expect("Borrow is not enabled for the pool");
                }

                let loan_unit_ratio = pool_state
                    .get_loan_unit_ratio()
                    .expect("Error getting loan unit ratio for provided resource");

                let (_, pool_borrowed_amount) = pool_state.pool.get_pooled_amount();

                let position_loan_units = cdp_data.get_loan_unit(pool_res_address);

                let mut max_loan_amount = position_loan_units / loan_unit_ratio;

                // ! Liquidation
                if for_liquidation {
                    max_loan_amount *= pool_state.pool_config.loan_close_factor;
                }

                max_loan_amount = max_loan_amount.min(payment.amount().into());

                let mut max_loan_value = (max_loan_amount * pool_state.price)
                    .min((pool_borrowed_amount * pool_state.price).into())
                    .checked_truncate(RoundingMode::ToNearestMidpointToEven)
                    .unwrap();

                // ! Liquidation
                if payment_value.is_some() {
                    max_loan_value = max_loan_value.min(expected_payment_value.into());
                    expected_payment_value -= max_loan_value;

                    assert!(
                        expected_payment_value >= dec!(0),
                        "expected_payment_value should not be negative: max_loan_value={}, expected_payment_value={}",
                        max_loan_value,
                        expected_payment_value
                    );
                };

                let repay_amount = max_loan_value / pool_state.price;

                let delta_loan_unit = pool_state
                    .deposit_for_repay(payment.take_advanced(
                        repay_amount,
                        WithdrawStrategy::Rounded(RoundingMode::ToNearestMidpointToEven),
                    ), !for_liquidation)
                    .expect("Error in deposit_from_repay");

                cdp_data
                    .update_loan(pool_res_address, -delta_loan_unit)
                    .expect("Error updating loan");

                remainders.push(payment);

                total_payment_value += max_loan_value;
            };

            if let Some(value) = payment_value {
                assert!(
                    expected_payment_value < ZERO_EPSILON,
                    "Insufficient payment value, {} required, {} total, {} remaining to pay",
                    value,
                    total_payment_value,
                    expected_payment_value
                );
            }

            if !for_liquidation {
                save_cdp_macro!(self, cdp_data);
            }

            (remainders, total_payment_value.checked_truncate(RoundingMode::ToNearestMidpointToEven)
            .unwrap())
        }

        fn _get_pool_state_without_update(
            &mut self,
            pool_res_address: &ResourceAddress,
            operating_service: Option<OperatingService>
        ) -> KeyValueEntryRefMut<'_, LendingPoolState> {
            let pool_state = self.pool_states.get_mut(pool_res_address).unwrap();

            if let Some(operating_status) = operating_service {
                pool_state
                    .check_operating_status(operating_status)
                    .expect("Invalid operating status");
            }

            pool_state
        }

        fn _get_pool_state(
            &mut self,
            pool_res_address: &ResourceAddress,
            operating_service: Option<OperatingService>,
            bypass_debounce: Option<(bool, bool)>,
        ) -> KeyValueEntryRefMut<'_, LendingPoolState> {
            let mut pool_state = self.pool_states.get_mut(pool_res_address).unwrap();

            if let Some(operating_status) = operating_service {
                pool_state
                    .check_operating_status(operating_status)
                    .expect("Invalid operating status");
            }

            pool_state
                .update_interest_and_price(bypass_debounce)
                .expect("Error updating pool state");

            pool_state
        }

        fn _get_new_cdp_id(&mut self) -> u64 {
            self.cdp_counter += 1;
            self.cdp_counter
        }

        fn _get_new_liquidator_id(&mut self) -> u64 {
            self.liquidator_counter += 1;
            self.liquidator_counter
        }

        fn _validate_cdp_proof(&self, cdp: Proof) -> NonFungibleLocalId {
            let validated_cdp = cdp.check(self.cdp_res_manager.address());
            validated_cdp.as_non_fungible().non_fungible_local_id()
        }

        fn _check_operating_status(&self, value: OperatingService) {
            assert!(
                self.operating_status.check(value.clone()),
                "{:?} is not allowed for this pool",
                value
            );
        }
    }
}
