use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct BabyWaterBear {
    pub name: String,
}

#[blueprint]
mod baby_nft {
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
            breed => PUBLIC;
            update_test_tube => restrict_to: [OWNER, admin];
        }
    }

    struct BabyNft {
        // Define what resources and data will be managed by Baby components
        nft_resource_manager: ResourceManager,
        supply: Decimal,
        test_tube_address: ResourceAddress,
    }

    impl BabyNft {
        pub fn instantiate_baby_nft(
            owner_badge: ResourceAddress,
            test_tube_address: ResourceAddress,
        ) -> Global<BabyNft> {
            // Create role badge
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(BabyNft::blueprint_id());
            let owner_role = OwnerRole::Fixed(rule!(require(owner_badge)));

            // Creates a no supply of NFTs.
            let nft_resource_manager = ResourceBuilder::new_string_non_fungible::<BabyWaterBear>(
                owner_role.clone(),
            )
            .metadata(metadata!(
                init {
                    "name" => "Baby WaterBear", updatable;
                    "description" => "Baby WaterBears Has A Supply Of 6,666 Supply.", updatable;
                    "icon_url" => Url::of("https://arweave.net/WspdWgJgZk2PYyszZaMUSOph1SINV5-tSNpyhFPHiAc"), updatable;
                    "tags" => vec!["nft", "breed"], updatable;
                }
            ))
            .non_fungible_data_update_roles(non_fungible_data_update_roles!(
            non_fungible_data_updater => rule!(require(global_caller(component_address)));
            non_fungible_data_updater_updater => rule!(require(owner_badge));
            ))
            .mint_roles(mint_roles!(
            minter => rule!(require(global_caller(component_address)));
            minter_updater => rule!(require(owner_badge));
            ))
            .burn_roles(burn_roles!(
            burner => rule!(require(global_caller(component_address)));
            burner_updater => rule!(require(owner_badge));
            ))
            .create_with_no_initial_supply();

            // Instantiate our component
            let component = Self {
                nft_resource_manager,
                supply: dec!(0),
                test_tube_address,
            }
            .instantiate()
            .prepare_to_globalize(owner_role)
            .with_address(address_reservation)
            .globalize();

            component
        }

        pub fn breed(&mut self, payment: NonFungibleBucket) -> Vec<NonFungibleBucket> {
            assert_eq!(
                payment.resource_address(),
                self.test_tube_address,
                "Only test tube are accepted as payments!"
            );

            let mut nfts = Vec::<NonFungibleBucket>::new();

            for _ in 0..i32::try_from(payment.amount()).unwrap() {
                self.supply += 1;
                let nft = self
                    .nft_resource_manager
                    .mint_non_fungible(
                        &NonFungibleLocalId::String(
                            StringNonFungibleLocalId::new(format!("BabyWaterBear_{}", self.supply))
                                .unwrap(),
                        ),
                        BabyWaterBear {
                            name: String::from(format!("BabyWaterBear #{}", self.supply)),
                        },
                    )
                    .as_non_fungible();
                nfts.push(nft);
            }
            payment.burn();

            return nfts;
        }

        pub fn update_test_tube(&mut self, test_tube_address: ResourceAddress) {
            self.test_tube_address = test_tube_address;
        }
    }
}
