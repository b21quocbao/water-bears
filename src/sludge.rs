use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct Sludge {
    pub name: String,

    #[mutable]
    pub key_image_url: Url,

    #[mutable]
    pub origin: String,
}

#[derive(ScryptoSbor)]
pub struct BuyInfo {
    pub address: ResourceAddress,
    pub price: Decimal,
    pub vault: Vault,
}

#[derive(ScryptoSbor)]
pub struct NftInfo {
    pub supply: Decimal,
    pub vault: NonFungibleVault,
    pub buy_infos: Vec<BuyInfo>,
    pub current: Decimal,
}

#[blueprint]
mod sludge {
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
            mint_nft => restrict_to: [OWNER, admin];
            withdraw_from_vault => restrict_to: [OWNER];
            buy_nft => PUBLIC;
            withdraw_nft => restrict_to: [OWNER, admin];
        }
    }

    struct SludgeNft {
        // Define what resources and data will be managed by Sludge components
        nft_resource_manager: ResourceManager,
        nft_info: NftInfo,
        bought_nfts: Vec<Decimal>,
    }

    impl SludgeNft {
        pub fn instantiate_sludge(owner_badge: ResourceAddress) -> Global<SludgeNft> {
            // Create role badge
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(SludgeNft::blueprint_id());
            let owner_role = OwnerRole::Fixed(rule!(require(owner_badge)));

            // Creates a no supply of NFTs.
            let nft_resource_manager = ResourceBuilder::new_string_non_fungible::<Sludge>(owner_role.clone())
            .metadata(metadata!(
                init {
                    "name" => "Sludge", updatable;
                    "description" => "", updatable;
                    "icon_url" => Url::of("https://arweave.net/PEKB1izj90JP2vD8on7Ejf1o0fEo8EPHPEPWQ2fdLK0"), updatable;
                    "tags" => vec!["nft", "sludge"], updatable;
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
                nft_resource_manager: nft_resource_manager,
                nft_info: NftInfo {
                    supply: dec!(0),
                    current: dec!(1),
                    vault: NonFungibleVault::new(nft_resource_manager.address()),
                    buy_infos: vec![],
                },
                bought_nfts: vec![],
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
                    vault: Vault::new(address),
                });
            } else {
                let payment_index = payment_index_wrap.unwrap();
                let buy_info = &mut nft_info.buy_infos[payment_index];
                buy_info.price = price;
            }
        }

        pub fn buy_nft(
            &mut self,
            mut payment: Bucket,
            cnt: Decimal,
        ) -> (Vec<NonFungibleBucket>, Bucket) {
            let nft_info = &mut self.nft_info;
            assert_eq!(
                nft_info.current + cnt - 1 <= nft_info.supply,
                true,
                "NFT supply exceeded"
            );

            let payment_index_wrap = nft_info
                .buy_infos
                .iter()
                .position(|r| r.address == payment.resource_address());
            assert_ne!(
                payment_index_wrap, None,
                "Only listed tokens are accepted as payments!"
            );

            let payment_index = payment_index_wrap.unwrap();
            let buy_info = &mut nft_info.buy_infos[payment_index];

            let actual_payment = payment.take(buy_info.price * cnt);

            buy_info.vault.put(actual_payment);

            let mut nfts = Vec::new();

            for _ in 0..i32::try_from(cnt).unwrap() {
                let nft = nft_info
                    .vault
                    .take_non_fungible(&NonFungibleLocalId::String(
                        StringNonFungibleLocalId::new(format!("Sludge_{}", nft_info.current))
                            .unwrap(),
                    ));
                self.bought_nfts.push(nft_info.current);
                nft_info.current += 1;

                nfts.push(nft);
            }

            return (nfts, payment);
        }

        pub fn mint_nft(&mut self, id: Vec<Decimal>) {
            let nft_info = &mut self.nft_info;
            for i in 0..id.len() {
                assert!(id[i] == nft_info.supply + 1, "Id not match!");
                let ticket = self.nft_resource_manager.mint_non_fungible(
                    &NonFungibleLocalId::String(
                        StringNonFungibleLocalId::new(format!("Sludge_{}", id[i])).unwrap(),
                    ),
                    Sludge {
                        name: String::from(format!("Sludge #{}", id[i])),
                        key_image_url: Url::of(
                            "https://arweave.net/PEKB1izj90JP2vD8on7Ejf1o0fEo8EPHPEPWQ2fdLK0",
                        ),
                        origin: "Toxic Sludge".to_string(),
                    },
                );
                nft_info.vault.put(ticket.as_non_fungible());

                nft_info.supply += 1;
            }
        }

        pub fn withdraw_nft(&mut self, cnt: Decimal) -> Vec<NonFungibleBucket> {
            let nft_info = &mut self.nft_info;
            assert_eq!(
                nft_info.current + cnt - 1 <= nft_info.supply,
                true,
                "NFT supply exceeded"
            );
            let mut nfts = Vec::new();

            for _ in 0..i32::try_from(cnt).unwrap() {
                let nft = nft_info
                    .vault
                    .take_non_fungible(&NonFungibleLocalId::String(
                        StringNonFungibleLocalId::new(format!("Sludge_{}", nft_info.current))
                            .unwrap(),
                    ));
                self.bought_nfts.push(nft_info.current);
                nft_info.current += 1;

                nfts.push(nft);
            }

            return nfts;
        }

        pub fn withdraw_from_vault(&mut self, address: ResourceAddress) -> Bucket {
            let nft_info = &mut self.nft_info;

            let payment_index_wrap = nft_info.buy_infos.iter().position(|r| r.address == address);
            assert_ne!(
                payment_index_wrap, None,
                "Only listed tokens are accepted as payments!"
            );
            let payment_index = payment_index_wrap.unwrap();
            let buy_info = &mut nft_info.buy_infos[payment_index];

            let actual_payment = buy_info.vault.take(buy_info.vault.amount());
            return actual_payment;
        }
    }
}
