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
mod meme_nft {
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
        }
    }

    struct MemeNft {
        // Define what resources and data will be managed by Meme components
        nft_resource_manager: ResourceManager,
        nft_info: NftInfo,
        bought_nfts: Vec<Decimal>,
    }

    impl MemeNft {
        pub fn instantiate_meme_nft() -> (Global<MemeNft>, Bucket) {
            // Create role badge
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(MemeNft::blueprint_id());
            let owner_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(init{
                    "name" => "WaterBear Owner Badge", updatable;
                }))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1)
                .into();
            let owner_role = OwnerRole::Fixed(rule!(require(owner_badge.resource_address())));

            // Creates a no supply of NFTs.
            let nft_resource_manager = ResourceBuilder::new_string_non_fungible::<WaterBear>(owner_role.clone())
            .metadata(metadata!(
                init {
                    "name" => "WaterBear", updatable;
                    "description" => "WaterBears Is Collection Of 3,333 Algorithmically Generated Tardigrades Living On The Radix Ledger. All Art Drawn By Hand In 24px Format. Explore The Collection's Ecosystem Through Staking, Gamified Mints And Custom Merchandise.", updatable;
                    "tags" => vec!["nft", "meme"], updatable;
                }
            ))
            .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                non_fungible_data_updater => rule!(require(global_caller(component_address)));
                non_fungible_data_updater_updater => rule!(require(owner_badge.resource_address()));
                ))
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(require(owner_badge.resource_address()));
                ))
            .burn_roles(burn_roles!(
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(require(owner_badge.resource_address()));
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

            (component, owner_badge)
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

        pub fn buy_nft(&mut self, mut payment: Bucket) -> (NonFungibleBucket, Bucket) {
            let nft_info = &mut self.nft_info;
            assert_eq!(
                nft_info.current <= nft_info.supply,
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

            let actual_payment = payment.take(buy_info.price);

            buy_info.vault.put(actual_payment);

            let nft = nft_info
                .vault
                .take_non_fungible(&NonFungibleLocalId::String(
                    StringNonFungibleLocalId::new(format!("WaterBear_{}", nft_info.current))
                        .unwrap(),
                ));

            self.bought_nfts.push(nft_info.current);

            nft_info.current += 1;

            return (nft, payment);
        }

        pub fn mint_nft(
            &mut self,
            id: Decimal,
            key_image_url: String,
            background: String,
            base: String,
            mouth: String,
            hats: String,
            neck: String,
            eyes: String,
            clothes: String,
        ) {
            let nft_info = &mut self.nft_info;
            assert!(id == nft_info.supply + 1, "Id not match!");
            let ticket = self.nft_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::String(
                    StringNonFungibleLocalId::new(format!("WaterBear_{}", id))
                        .unwrap(),
                ),
                WaterBear {
                    name: String::from(format!("WaterBear #{}", id)),
                    key_image_url: Url::of(key_image_url),
                    background,
                    base,
                    mouth,
                    hats,
                    neck,
                    eyes,
                    clothes,
                },
            );
            nft_info.vault.put(ticket.as_non_fungible());

            nft_info.supply += 1;
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
