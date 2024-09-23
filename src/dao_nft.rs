use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct Dao {
    pub name: String,

    #[mutable]
    pub key_image_url: Url,

    #[mutable]
    pub description: String,
}
  
#[derive(ScryptoSbor)]
pub struct NftInfo {
    pub supply: Decimal,
    pub current: Decimal,
}

#[blueprint]
mod dao {
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
            mint_nft => restrict_to: [OWNER, admin];
        }
    }

    struct DaoNft {
        // Define what resources and data will be managed by Dao components
        nft_resource_manager: ResourceManager,
        nft_info: NftInfo,
    }

    impl DaoNft {
        pub fn instantiate_dao(owner_badge: ResourceAddress) -> Global<DaoNft> {
            // Create role badge
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(DaoNft::blueprint_id());
            let owner_role = OwnerRole::Fixed(rule!(require(owner_badge)));

            // Creates a no supply of NFTs.
            let nft_resource_manager = ResourceBuilder::new_string_non_fungible::<Dao>(owner_role.clone())
            .metadata(metadata!(
                init {
                    "name" => "WaterBears DAO", updatable;
                    "description" => "", updatable;
                    "icon_url" => Url::of("https://arweave.net/RcELr8fbTER3aJ-HvHeBLaicCikU_xZiWfqZVnke2NI"), updatable;
                    "tags" => vec!["nft", "dao"], updatable;
                }
            ))
            .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                non_fungible_data_updater => rule!(require(owner_badge));
                non_fungible_data_updater_updater => rule!(require(owner_badge));
            ))
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(require(owner_badge));
            ))
            .burn_roles(burn_roles!(
                burner => rule!(require(owner_badge));
                burner_updater => rule!(require(owner_badge));
            ))
            .create_with_no_initial_supply();

            // Instantiate our component
            let component = Self {
                nft_resource_manager: nft_resource_manager,
                nft_info: NftInfo {
                    supply: dec!(0),
                    current: dec!(1),
                },
            }
            .instantiate()
            .prepare_to_globalize(owner_role)
            .with_address(address_reservation)
            .globalize();

            component
        }

        pub fn mint_nft(&mut self, id: Vec<Decimal>, key_image_url: Vec<String>) -> Vec<NonFungibleBucket> {
            let nft_info = &mut self.nft_info;
            let mut nfts = Vec::new();

            for i in 0..id.len() {
                assert!(id[i] == nft_info.supply + 1, "Id not match!");
                let ticket = self.nft_resource_manager.mint_non_fungible(
                    &NonFungibleLocalId::String(
                        StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", id[i])).unwrap(),
                    ),
                    Dao {
                        name: String::from(format!("WaterBears DAO #{}", id[i])),
                        key_image_url: Url::of(key_image_url[i].clone()),
                        description: String::from(format!("WaterBears DAO #{} is a member of the WaterBears DAO", id[i])),
                    },
                );

                nft_info.supply += 1;
                nfts.push(ticket.as_non_fungible());
            }

            return nfts;
        }
    }
}
