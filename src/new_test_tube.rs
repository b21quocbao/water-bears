use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct TestTube {
    pub name: String,
}

#[derive(ScryptoSbor)]
pub struct BuyInfo {
    pub address: ResourceAddress,
    pub price: Decimal,
}

#[derive(ScryptoSbor)]
pub struct NftInfo {
    pub supply: Decimal,
    pub buy_infos: Vec<BuyInfo>,
}

#[blueprint]
mod breed_nft {
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
            update_payment => restrict_to: [OWNER, admin];
            buy_nft => PUBLIC;
            update_genesis => restrict_to: [OWNER, admin];
            update_dna_amount => restrict_to: [OWNER, admin];
            update_supply => restrict_to: [OWNER, admin];
        }
    }

    struct NewBreedNft {
        // Define what resources and data will be managed by Breed components
        nft_resource_manager: ResourceManager,
        nft_info: NftInfo,
        genesis_address: ResourceAddress,
        genesis_amount: Decimal,
        dna_amount: Decimal,
    }

    impl NewBreedNft {
        pub fn instantiate_breed_nft(
            owner_badge: ResourceAddress,
            genesis_address: ResourceAddress,
            genesis_amount: Decimal,
            test_tube_address: ResourceAddress
        ) -> Global<NewBreedNft> {
            // Create role badge
            let (address_reservation, _) =
                Runtime::allocate_component_address(NewBreedNft::blueprint_id());
            let owner_role = OwnerRole::Fixed(rule!(require(owner_badge)));

            // Instantiate our component
            let component = Self {
                nft_resource_manager: ResourceManager::from_address(test_tube_address),
                nft_info: NftInfo {
                    supply: dec!(0),
                    buy_infos: vec![],
                },
                genesis_address,
                genesis_amount,
                dna_amount: dec!(300)
            }
            .instantiate()
            .prepare_to_globalize(owner_role)
            .with_address(address_reservation)
            .globalize();

            component
        }

        pub fn update_payment(&mut self, address: ResourceAddress, price: Decimal) {
            let nft_info = &mut self.nft_info;

            let payment_index_wrap = nft_info.buy_infos.iter().position(|r| r.address == address);
            if payment_index_wrap == None {
                nft_info.buy_infos.push(BuyInfo {
                    address: address,
                    price: price,
                });
            } else {
                let payment_index = payment_index_wrap.unwrap();
                let buy_info = &mut nft_info.buy_infos[payment_index];
                buy_info.price = price;
            }
        }

        pub fn buy_nft(
            &mut self,
            payment: Bucket,
            cnt: Decimal,
            nft_proof: NonFungibleProof,
        ) -> Vec<NonFungibleBucket> {
            let nft_info = &mut self.nft_info;
            let payment_index_wrap = nft_info
                .buy_infos
                .iter()
                .position(|r| r.address == payment.resource_address());
            assert_ne!(
                payment_index_wrap, None,
                "Only listed tokens are accepted as payments!"
            );

            let nft_proof =
                nft_proof.check_with_message(self.genesis_address, "Invalid proof address!");
            assert!(
                nft_proof.contains_amount(self.genesis_amount),
                "Invalid proof amount!"
            );

            assert!(payment.amount() == cnt * self.dna_amount, "Invalid dna amount!");

            payment.burn();

            let mut nfts = Vec::<NonFungibleBucket>::new();

            for _ in 0..i32::try_from(cnt).unwrap() {
                nft_info.supply += 1;
                let nft = self
                    .nft_resource_manager
                    .mint_non_fungible(
                        &NonFungibleLocalId::String(
                            StringNonFungibleLocalId::new(format!("TestTube_{}", nft_info.supply))
                                .unwrap(),
                        ),
                        TestTube {
                            name: String::from(format!("TestTube #{}", nft_info.supply)),
                        },
                    )
                    .as_non_fungible();

                nfts.push(nft);
            }

            return nfts;
        }

        pub fn update_genesis(&mut self, genesis_address: ResourceAddress, gensis_amount: Decimal) {
            self.genesis_address = genesis_address;
            self.genesis_amount = gensis_amount;
        }

        pub fn update_dna_amount(&mut self, dna_amount: Decimal) {
            self.dna_amount = dna_amount;
        }
        
        pub fn update_supply(&mut self, supply: Decimal) {
            self.nft_info.supply = supply;
        }
    }
}
