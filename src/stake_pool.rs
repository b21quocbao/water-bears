use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct WaterBear {
    pub name: String,

    #[mutable]
    pub key_image_url: Url,

    #[mutable]
    pub background: String,
    #[mutable]
    pub base: String,
    #[mutable]
    pub mouth: String,
    #[mutable]
    pub hats: String,
    #[mutable]
    pub neck: String,
    #[mutable]
    pub eyes: String,
    #[mutable]
    pub clothes: String,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct Id {
    #[mutable]
    pub updated_at: i64,
    #[mutable]
    pub nfts: BTreeSet<NonFungibleLocalId>,
    #[mutable]
    pub last_claim_time: Decimal,
    #[mutable]
    pub total_claim_amount: Decimal,
    #[mutable]
    pub reward_amount: Decimal,
    #[mutable]
    pub next_period: i64,
}

#[blueprint]
mod stake_pool {
    // Defines component method authority

    // --Roles--
    // * `OWNER` -  The owner of the component which has authority to change resource and component
    //    metadata.
    // * `SELF` - The component itself which is designated authority to mint candies.
    // * `member` - The member which is specified by those who hold the badge resource determined by
    //    the `member_card_address`.
    // * `PUBLIC` - The role that defines everybody else.
    enable_method_auth! {
        roles {
            admin => updatable_by: [SELF,OWNER];
            server => updatable_by: [SELF,OWNER];
        },
        methods {
            stake => PUBLIC;
            withdraw => PUBLIC;
            claim_rewards => PUBLIC;
            deposit_reward_pool => PUBLIC;
            create_id => PUBLIC;
            withdraw_reward_pool => restrict_to: [OWNER];
            withdraw_all_reward_pool => restrict_to: [OWNER];
            update_reward_factor => restrict_to: [OWNER];
            update_reward_address => restrict_to: [OWNER];
        }
    }

    struct StakePool {
        // Define what resources and data will be managed by StakePool components
        nft_vault: NonFungibleVault,
        cnt: u64,
        reward_vault: Vec<Vault>,
        reward_factor: Decimal,
        id_manager: ResourceManager,
        id_counter: u64,
        current_period: i64,
    }

    impl StakePool {
        pub fn instantiate_stake_pool(
            nft_address: ResourceAddress,
            badge_address: ResourceAddress,
            reward_address: ResourceAddress,
        ) -> Global<StakePool> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(StakePool::blueprint_id());
            let id_manager = ResourceBuilder::new_integer_non_fungible::<Id>(OwnerRole::Fixed(
                    rule!(require(badge_address)),
                ))
                .metadata(metadata!(
                    init {
                        "name" => format!("WaterBear Staking ID"), updatable;
                        "description" => format!("An ID recording your stake in the WaterBear ecosystem."), updatable;
                    }
                ))
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address))
                    || require(badge_address));
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles!(
                    burner => rule!(deny_all);
                    burner_updater => rule!(deny_all);
                ))
                .withdraw_roles(withdraw_roles!(
                    withdrawer => rule!(deny_all);
                    withdrawer_updater => rule!(deny_all);
                ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                    non_fungible_data_updater => rule!(require(global_caller(component_address))
                    || require(badge_address));
                    non_fungible_data_updater_updater => rule!(deny_all);
                ))
                .create_with_no_initial_supply();
            let owner_role = OwnerRole::Fixed(rule!(require(badge_address)));

            // Instantiate our component
            let component = Self {
                nft_vault: NonFungibleVault::new(nft_address),
                reward_vault: vec![Vault::new(reward_address)],
                reward_factor: dec!(5.0),
                id_manager,
                id_counter: 0,
                cnt: 0,
                current_period: 0,
            }
            .instantiate()
            .prepare_to_globalize(owner_role)
            .with_address(address_reservation)
            .globalize();

            component
        }

        pub fn deposit_reward_pool(&mut self, payment: Bucket) {
            self.reward_vault[0].put(payment);
        }

        pub fn create_id(&mut self) -> Bucket {
            self.id_counter += 1;

            let id_data = Id {
                updated_at: 0,
                nfts: BTreeSet::new(),
                last_claim_time: dec!(0),
                total_claim_amount: dec!(0),
                reward_amount: dec!(0),
                next_period: self.current_period + 1,
            };

            let id: Bucket = self
                .id_manager
                .mint_non_fungible(&NonFungibleLocalId::integer(self.id_counter), id_data);

            id
        }

        pub fn stake(&mut self, payment: NonFungibleBucket, id_proof: NonFungibleProof) {
            let id: NonFungibleLocalId;

            let id_proof =
                id_proof.check_with_message(self.id_manager.address(), "Invalid Id supplied!");
            id = id_proof
                .as_non_fungible()
                .non_fungible::<Id>()
                .local_id()
                .clone();

            self.recal_reward(&id);

            let id_data: Id = self.id_manager.get_non_fungible_data(&id);
            let mut nfts = id_data.nfts;

            nfts.insert(payment.non_fungible::<WaterBear>().local_id().clone());
            self.nft_vault.put(payment);
            self.cnt += 1;

            self.id_manager.update_non_fungible_data(&id, "nfts", nfts);
        }

        pub fn withdraw(
            &mut self,
            nft_id: NonFungibleLocalId,
            id_proof: NonFungibleProof,
        ) -> NonFungibleBucket {
            let id_proof =
                id_proof.check_with_message(self.id_manager.address(), "Invalid Id supplied!");

            let id: NonFungibleLocalId = id_proof.non_fungible::<Id>().local_id().clone();
            self.recal_reward(&id);

            let id_data: Id = self.id_manager.get_non_fungible_data(&id);
            let mut nfts = id_data.nfts;

            nfts.remove(&nft_id);
            let nft_payment = self.nft_vault.take_non_fungible(&nft_id);
            self.cnt -= 1;

            self.id_manager.update_non_fungible_data(&id, "nfts", nfts);

            return nft_payment;
        }

        pub fn claim_rewards(&mut self, id_proof: NonFungibleProof) -> Bucket {
            let id_proof =
                id_proof.check_with_message(self.id_manager.address(), "Invalid Id supplied!");

            let id = id_proof.non_fungible::<Id>().local_id().clone();
            self.recal_reward(&id);

            let id_data: Id = self.id_manager.get_non_fungible_data(&id);

            let reward_payment = self.reward_vault[0].take(id_data.reward_amount);

            self.id_manager
                .update_non_fungible_data(&id, "reward_amount", dec!(0));

            return reward_payment;
        }

        pub fn withdraw_reward_pool(&mut self, amount: Decimal) -> Bucket {
            assert!(
                amount < self.reward_vault[0].amount(),
                "Withdrawal amount exceeds vault supply!, supply: {:?} but withdrawal amount: {amount}",
                self.reward_vault[0].amount()
            );
            let reward_payment = self.reward_vault[0].take(amount);
            return reward_payment;
        }

        pub fn withdraw_all_reward_pool(&mut self) -> Bucket {
            let reward_payment = self.reward_vault[0].take_all();
            return reward_payment;
        }

        pub fn update_reward_factor(&mut self, reward_factor: Decimal) {
            self.reward_factor = reward_factor;
        }

        pub fn update_reward_address(&mut self, reward_address: ResourceAddress) {
            assert!(self.reward_vault[0].is_empty(), "Vault is not empty",);
            let address = self.reward_vault[0].resource_address();
            self.reward_vault.push(Vault::new(reward_address));
            self.reward_vault.rotate_right(1);
            let address_after = self.reward_vault[0].resource_address();
            assert!(address != address_after, "Something wrong",);
        }

        fn recal_reward(&mut self, id: &NonFungibleLocalId) {
            let id_data: Id = self.id_manager.get_non_fungible_data(id);

            let now = Clock::current_time_rounded_to_minutes().seconds_since_unix_epoch;

            self.id_manager.update_non_fungible_data(
                &id,
                "reward_amount",
                id_data.reward_amount
                    + self.reward_factor * (now - id_data.updated_at) * id_data.nfts.len() / 86400,
            );
            self.id_manager
                .update_non_fungible_data(&id, "updated_at", now);
        }
    }
}
