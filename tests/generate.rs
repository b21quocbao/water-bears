use scrypto_test::prelude::*;
use scrypto_test::utils::dump_manifest_to_file_system;
use scrypto::prelude::*;

#[derive(Clone, Debug, NonFungibleData, ScryptoSbor, ManifestSbor)]
pub struct WaterBear {
    pub name: String,

    pub key_image_url: Url,

    pub background: String,
    pub base: String,
    pub mouth: String,
    pub hats: String,
    pub neck: String,
    pub eyes: String,
    pub clothes: String,
}

#[test]
fn test_hello() {
    let network = &NetworkDefinition::mainnet();
    let net_decoder = AddressBech32Decoder::new(network);
    let account_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "account_rdx129qmwg7xtcqsegd0zpulw3wpdpp7c4xmrwcssy2jt095827lls6rtq",
    )
    .unwrap();
    let buy_account_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "account_rdx12930ndfh8spce8fxw0rm5wrfvkgqkl5hv27un0vqnmc76xu6k44kx4",
    )
    .unwrap();
    let treasury_account_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "account_rdx12xxj3j5lx4ls2303pwfs7rtfqnyw77uwc9fluz7d3er6x4vp3n2utz",
    )
    .unwrap();

    let package_address = PackageAddress::try_from_bech32(
        &net_decoder,
        "package_rdx1pk8cnsvttsc2dc6v77tma5ztwf4760l0a06utwzzkjpe8twc2k234w",
    )
    .unwrap();
    let auction_package_address = PackageAddress::try_from_bech32(
        &net_decoder,
        "package_rdx1p49fqf3xjhan7f0e8r3mf57rmdznfcxnfw2remqleyq9725ds3a7uw",
    )
    .unwrap();

    let water_bear_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_rdx1czkg9u5ap0dpdklz9llqq6arer2ned5jj06y7dweu6yj9m0u9ejys2",
    )
    .unwrap();
    let owner_badge: ResourceAddress = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_rdx1t506yl9nqyw90h9gxx323t67r2p99thssu3atwsnwum4l68t3uf9jv",
    )
    .unwrap();
    let water_bear_resource = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_rdx1nguyesjve2e0wql8d9cepx7u63jtevdg05a7n5fc5m767mn4vkxpeq",
    )
    .unwrap();

    let stake_pool_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_rdx1cpg2g2nx5jm20ykyqheymuzs2wvulhn5qlpvhd6q4958vfj6gaa2yh",
    )
    .unwrap();
    let id_resource = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_rdx1nf0wazqeccmpltf5q4a7msxcz4eyaejh4s2kpc75g7flwy4s00dd0l",
    )
    .unwrap();
    let dna_token = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_rdx1th9txt2hxdlc740ypf9uj8ghgwwmhf7wdnxafhxa3uxmmus56nusxs",
    )
    .unwrap();

    let test_tube_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_rdx1cztegh0j733zgv452uhe4avthvxu5ld00fer77awt05a87h98zkkn8",
    )
    .unwrap();
    let new_test_tube_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_rdx1cp0yra6p000xhwf0eyd3aa8pdyyqgqjcrvq4rkkh76h22su6z2j9p9",
    )
    .unwrap();
    let test_tube_nft = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_rdx1n2e6jk57z6dde2e4f6zleju7rgawdya2jzvtgkr5kv6cz5j64uu780",
    )
    .unwrap();

    let sludge_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_rdx1czqx3kux3t2fgsaafufudc3626gdwn7e3fj62s6fzl2ukzu8xzu6kf",
    )
    .unwrap();
    
    let water_bear_dao_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_rdx1cp38s8l7g7ntvvszt7azj485nqzxflfqm4jupdf9m8axr304vsqz4y",
    )
    .unwrap();

    let english_auction_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_rdx1czqx3kux3t2fgsaafufudc3626gdwn7e3fj62s6fzl2ukzu8xzu6kf",
    )
    .unwrap();
    let accepted_payment_token = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
    )
    .unwrap();
    let bidder_badge = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_rdx1n2e6jk57z6dde2e4f6zleju7rgawdya2jzvtgkr5kv6cz5j64uu780",
    )
    .unwrap();

    
    // // instantiate_meme_nft
    // let manifest = ManifestBuilder::new()
    //     .call_function(
    //         package_address,
    //         "MemeNft",
    //         "instantiate_meme_nft",
    //         manifest_args!(),
    //     )
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("1. instantiate_meme_nft"),
    //     network,
    // );

    // // update_payment
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(water_bear_component, "update_payment", (XRD, dec!(45)));
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("2. update_payment"),
    //     network,
    // );

    // // // mint_nft
    // // let manifest = ManifestBuilder::new()
    // //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    // //     .call_method(
    // //         water_bear_component,
    // //         "mint_nft",
    // //         (
    // //             [dec!(1), dec!(2), dec!(3), dec!(4), dec!(5)],
    // //             [
    // //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    // //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    // //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    // //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    // //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    // //             ],
    // //             ["Green", "Green", "Green", "Green", "Green"],
    // //             ["Red", "Red", "Red", "Red", "Red"],
    // //             ["", "", "", "", ""],
    // //             ["", "", "", "", ""],
    // //             ["", "", "", "", ""],
    // //             ["", "", "", "", ""],
    // //             ["", "", "", "", ""],
    // //         ),
    // //     );
    // // let _ = dump_manifest_to_file_system(
    // //     manifest.object_names(),
    // //     &manifest.build(),
    // //     "./transaction_manifest",
    // //     Some("3. mint_nft"),
    // //     network,
    // // );

    // // buy_nft
    // let manifest = ManifestBuilder::new()
    //     .withdraw_from_account(buy_account_component, XRD, dec!(750))
    //     .take_all_from_worktop(XRD, "nft_bucket_1")
    //     .call_method_with_name_lookup(water_bear_component, "buy_nft", |lookup| {
    //         (lookup.bucket("nft_bucket_1"), dec!(3))
    //     })
    //     .deposit_batch(buy_account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("4. buy_nft"),
    //     network,
    // );

    // // withdraw_nft
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(water_bear_component, "withdraw_nft", (dec!(2),))
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("5. withdraw_nft"),
    //     network,
    // );

    // // withdraw_from_vault
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(water_bear_component, "withdraw_from_vault", (XRD,))
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("6. withdraw_from_vault"),
    //     network,
    // );

    // // instantiate_stake_pool
    // let manifest = ManifestBuilder::new()
    //     .call_function(
    //         package_address,
    //         "StakePool",
    //         "instantiate_stake_pool",
    //         manifest_args!(water_bear_resource, owner_badge, dna_token),
    //     )
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("7. instantiate_stake_pool"),
    //     network,
    // );

    // // deposit_reward_pool
    // let manifest = ManifestBuilder::new()
    //     .withdraw_from_account(account_component, dna_token, dec!(250))
    //     .take_all_from_worktop(dna_token, "bucket")
    //     .call_method_with_name_lookup(stake_pool_component, "deposit_reward_pool", |lookup| {
    //         (lookup.bucket("bucket"),)
    //     });
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("8. deposit_reward_pool"),
    //     network,
    // );

    // // create_id
    // let manifest = ManifestBuilder::new()
    //     .call_method(stake_pool_component, "create_id", ())
    //     .deposit_batch(buy_account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("9. create_id"),
    //     network,
    // );

    // // stake nft
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_non_fungibles(
    //         account_component,
    //         id_resource,
    //         indexset!(NonFungibleLocalId::integer(1)),
    //     )
    //     .pop_from_auth_zone("proof_1")
    //     .call_method_with_name_lookup(stake_pool_component, "withdraw", |lookup| {
    //         (
    //             indexset!(
    //                 NonFungibleLocalId::String(
    //                     StringNonFungibleLocalId::new("WaterBear_1").unwrap()
    //                 ),
    //                 NonFungibleLocalId::String(
    //                     StringNonFungibleLocalId::new("WaterBear_2").unwrap()
    //                 )
    //             ),
    //             lookup.proof("proof_1"),
    //         )
    //     })
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("10. stake nft"),
    //     network,
    // );

    // // update_reward_factor
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(stake_pool_component, "update_reward_factor", (dec!(4.9),));
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("11. update_reward_factor"),
    //     network,
    // );

    // // claim_rewards
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_non_fungibles(
    //         account_component,
    //         id_resource,
    //         indexset!(NonFungibleLocalId::integer(1)),
    //     )
    //     .pop_from_auth_zone("proof")
    //     .call_method_with_name_lookup(stake_pool_component, "claim_rewards", |lookup| {
    //         (lookup.proof("proof"),)
    //     })
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("12. claim_rewards"),
    //     network,
    // );

    // // withdraw
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_non_fungibles(
    //         account_component,
    //         id_resource,
    //         indexset!(NonFungibleLocalId::integer(1)),
    //     )
    //     .pop_from_auth_zone("proof_1")
    //     .call_method_with_name_lookup(stake_pool_component, "withdraw", |lookup| {
    //         (
    //             indexset!(
    //                 NonFungibleLocalId::String(
    //                     StringNonFungibleLocalId::new("WaterBear_1").unwrap()
    //                 ),
    //                 NonFungibleLocalId::String(
    //                     StringNonFungibleLocalId::new("WaterBear_2").unwrap()
    //                 )
    //             ),
    //             lookup.proof("proof_1"),
    //         )
    //     })
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("13. withdraw"),
    //     network,
    // );

    // // claim_rewards
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_non_fungibles(
    //         buy_account_component,
    //         id_resource,
    //         indexset!(NonFungibleLocalId::integer(1)),
    //     )
    //     .pop_from_auth_zone("proof")
    //     .call_method_with_name_lookup(stake_pool_component, "claim_rewards", |lookup| {
    //         (lookup.proof("proof"),)
    //     })
    //     .deposit_batch(buy_account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("14. claim_rewards"),
    //     network,
    // );

    // // withdraw_reward_pool
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(stake_pool_component, "withdraw_reward_pool", (dec!(210),))
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("15. withdraw_reward_pool"),
    //     network,
    // );

    // // withdraw_all_reward_pool
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(stake_pool_component, "withdraw_all_reward_pool", ())
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("16. withdraw_all_reward_pool"),
    //     network,
    // );

    // // update_reward_address
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(stake_pool_component, "update_reward_address", (XRD,));
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("17. update_reward_address"),
    //     network,
    // );

    // // instantiate_breed_nft
    // let manifest = ManifestBuilder::new()
    //     .call_function(
    //         package_address,
    //         "BreedNft",
    //         "instantiate_breed_nft",
    //         manifest_args!(owner_badge, water_bear_resource, dec!(2)),
    //     )
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("18. instantiate_breed_nft"),
    //     network,
    // );

    // // update burner
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .set_role(
    //         dna_token,
    //         ObjectModuleId::Main,
    //         RoleKey::from("burner"),
    //         rule!(require(global_caller(test_tube_component))),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("19. burner"),
    //     network,
    // );

    // // update_payment
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(
    //         test_tube_component,
    //         "update_payment",
    //         (dna_token, dec!(300)),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("20. update_payment"),
    //     network,
    // );

    // // buy_nft
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_non_fungibles(
    //         buy_account_component,
    //         water_bear_resource,
    //         indexset!(
    //             NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
    //             NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_2").unwrap()),
    //         ),
    //     )
    //     .pop_from_auth_zone("proof_1")
    //     .withdraw_from_account(buy_account_component, dna_token, dec!(900))
    //     .take_all_from_worktop(dna_token, "bucket")
    //     .call_method_with_name_lookup(test_tube_component, "buy_nft", |lookup| {
    //         (lookup.bucket("bucket"), dec!(3), lookup.proof("proof_1"))
    //     })
    //     .deposit_batch(buy_account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("21. buy_nft"),
    //     network,
    // );

    // // instantiate_baby_nft
    // let manifest = ManifestBuilder::new()
    //     .call_function(
    //         package_address,
    //         "BabyNft",
    //         "instantiate_baby_nft",
    //         manifest_args!(owner_badge, test_tube_nft),
    //     )
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("22. instantiate_baby_nft"),
    //     network,
    // );

    // // update burner
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .set_role(
    //         test_tube_nft,
    //         ObjectModuleId::Main,
    //         RoleKey::from("burner"),
    //         rule!(require(global_caller(water_bear_component))),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("23. burner"),
    //     network,
    // );

    // // custom
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .set_role(
    //         water_bear_resource,
    //         ObjectModuleId::Main,
    //         RoleKey::from("non_fungible_data_updater"),
    //         rule!(require(owner_badge)),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("25. custom_1"),
    //     network,
    // );

    // // custom
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .update_non_fungible_data(
    //         water_bear_resource,
    //         NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
    //         "key_image_url",
    //         "https://arweave.net/A1UgVPZYbv7d0ymltAHAP8KLTvNgb34CnT8DFQYyH80",
    //     )
    //     .update_non_fungible_data(
    //         water_bear_resource,
    //         NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_2").unwrap()),
    //         "key_image_url",
    //         "https://arweave.net/fKLXKQeYYNRXAtRwelJ4hydWAayDA9Krv3D4DklhlM0",
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("26. custom_2"),
    //     network,
    // );

    // // instantiate_breed_nft
    // let manifest = ManifestBuilder::new()
    //     .call_function(
    //         package_address,
    //         "NewBreedNft",
    //         "instantiate_breed_nft",
    //         manifest_args!(owner_badge, water_bear_resource, dec!(2), test_tube_nft),
    //     )
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("27. instantiate_breed_nft"),
    //     network,
    // );

    // // update burner
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .set_role(
    //         dna_token,
    //         ObjectModuleId::Main,
    //         RoleKey::from("burner"),
    //         rule!(require(global_caller(new_test_tube_component))),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("28. update burner"),
    //     network,
    // );

    // // update minter
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .set_role(
    //         test_tube_nft,
    //         ObjectModuleId::Main,
    //         RoleKey::from("minter"),
    //         rule!(require(global_caller(new_test_tube_component))),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("29. update minter"),
    //     network,
    // );

    // // update_payment
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(
    //         new_test_tube_component,
    //         "update_payment",
    //         (dna_token, dec!(300)),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("30. update_payment"),
    //     network,
    // );

    // // update_supply
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(new_test_tube_component, "update_supply", (dec!(1000),));
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("31. update_supply"),
    //     network,
    // );

    // // buy_nft
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_non_fungibles(
    //         account_component,
    //         water_bear_resource,
    //         indexset!(
    //             NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
    //             NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_2").unwrap()),
    //         ),
    //     )
    //     .pop_from_auth_zone("proof_1")
    //     .withdraw_from_account(account_component, dna_token, dec!(900))
    //     .take_all_from_worktop(dna_token, "bucket")
    //     .call_method_with_name_lookup(new_test_tube_component, "buy_nft", |lookup| {
    //         (lookup.bucket("bucket"), dec!(3), lookup.proof("proof_1"))
    //     })
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("33. buy_nft"),
    //     network,
    // );

    // // instantiate_sludge
    // let manifest = ManifestBuilder::new()
    //     .call_function(
    //         package_address,
    //         "SludgeNft",
    //         "instantiate_sludge",
    //         manifest_args!(owner_badge),
    //     )
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("34. instantiate_sludge"),
    //     network,
    // );

    // // update_payment
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(sludge_component, "update_payment", (XRD, dec!(250)));
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("35. update_payment"),
    //     network,
    // );

    // // mint_nft
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(
    //         sludge_component,
    //         "mint_nft",
    //         ([dec!(1), dec!(2), dec!(3), dec!(4), dec!(5)],),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("36. mint_nft"),
    //     network,
    // );

    // // buy_nft
    // let manifest = ManifestBuilder::new()
    //     .withdraw_from_account(account_component, XRD, dec!(750))
    //     .take_all_from_worktop(XRD, "nft_bucket_1")
    //     .call_method_with_name_lookup(sludge_component, "buy_nft", |lookup| {
    //         (lookup.bucket("nft_bucket_1"), dec!(3))
    //     })
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("37. buy_nft"),
    //     network,
    // );

    // // withdraw_nft
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(sludge_component, "withdraw_nft", (dec!(2),))
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("38. withdraw_nft"),
    //     network,
    // );

    // // withdraw_from_vault
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(sludge_component, "withdraw_from_vault", (XRD,))
    //     .deposit_batch(account_component);
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("39. withdraw_from_vault"),
    //     network,
    // );

    // // update burner
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .set_role(
    //         water_bear_resource,
    //         ObjectModuleId::Main,
    //         RoleKey::from("burner"),
    //         rule!(require(global_caller(water_bear_component))),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("40. burner"),
    //     network,
    // );

    // instantiate_english_auction
    let relative_ending_epoch: u64 = 288;
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(
            treasury_account_component,
            water_bear_resource,
            indexset!(NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBearsDAO_3").unwrap())),
        )
        .take_all_from_worktop(water_bear_resource, "bucket")
        .call_function_with_name_lookup(
            auction_package_address,
            "EnglishAuction",
            "instantiate_english_auction",
            |lookup| {
                (
                    vec![lookup.bucket("bucket")],
                    accepted_payment_token,
                    owner_badge,
                    water_bear_resource,
                    relative_ending_epoch,
                )
            },
        )
        .deposit_batch(treasury_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("41. instantiate_english_auction"),
        network,
    );

    // bid
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            buy_account_component,
            water_bear_resource,
            indexset!(
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
            ),
        )
        .withdraw_from_account(buy_account_component, XRD, dec!(100))
        .take_all_from_worktop(XRD, "bucket")
        .call_method_with_name_lookup(english_auction_component, "bid", |lookup| {
            (lookup.bucket("bucket"),)
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("42. bid"),
        network,
    );

    // increase_bid_more
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(buy_account_component, XRD, dec!(21))
        .take_all_from_worktop(XRD, "bucket")
        .create_proof_from_account_of_non_fungibles(
            buy_account_component,
            bidder_badge,
            btreeset!(NonFungibleLocalId::String(
                StringNonFungibleLocalId::new("replace_here").unwrap()
            )),
        )
        .pop_from_auth_zone("proof")
        .call_method_with_name_lookup(english_auction_component, "increase_bid", |lookup| {
            (lookup.bucket("bucket"), lookup.proof("proof"), )
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("44. increase_bid_more"),
        network,
    );

    // admin_auction_settlement
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(treasury_account_component, owner_badge, dec!(1))
        .call_method(english_auction_component, "admin_auction_settlement", ())
        .deposit_batch(treasury_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("45. admin_auction_settlement"),
        network,
    );

    // claim_nfts
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(
            buy_account_component,
            bidder_badge,
            btreeset!(NonFungibleLocalId::String(
                        StringNonFungibleLocalId::new("replace_here").unwrap()
                    )),
        )
        .take_all_from_worktop(bidder_badge, "bucket")
        .call_method_with_name_lookup(english_auction_component, "claim_nfts", |lookup| {
            (lookup.bucket("bucket"),)
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("46. claim_nfts"),
        network,
    );

    // cancel_bid
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(
            buy_account_component,
            bidder_badge,
            btreeset!(NonFungibleLocalId::String(
                        StringNonFungibleLocalId::new("replace_here").unwrap()
                    )),
        )
        .take_all_from_worktop(bidder_badge, "bucket")
        .call_method_with_name_lookup(english_auction_component, "cancel_bid", |lookup| {
            (lookup.bucket("bucket"),)
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("47. cancel_bid"),
        network,
    );
    
    // withdraw_payment
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(english_auction_component, "withdraw_payment", ())
        .deposit_batch(treasury_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("48. withdraw_payment"),
        network,
    );

    
    // update minter
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .set_role(
            water_bear_resource,
            ObjectModuleId::Main,
            RoleKey::from("minter"),
            rule!(require(global_caller(water_bear_component)) || require(owner_badge)),
        );
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("50. update minter"),
        network,
    );

    let key_image_urls = [
        "",
        "https://arweave.net/RcELr8fbTER3aJ-HvHeBLaicCikU_xZiWfqZVnke2NI",
        "https://arweave.net/mIwohfcO38JblFxJdrajFiVJVyhmgVYXNGsKSdf350w",
        "https://arweave.net/Bdd5tNpZS6fbqYe3yFnUTLrgAnrlPfRRaN_BrWBPEdw",
        "https://arweave.net/KVH6sbjLjultdcHbQHZSFliIL2i6Kpc8LX4JfEbau4c",
        "https://arweave.net/AJWfAWQrmUFN3RJfR04FQj0qbka2no5vPwI9dMBUeG0",
        "https://arweave.net/8pr2GbVdguq4DJ91f34MWLMl_xfXJzmyMZjW8pwe1fM",
        "https://arweave.net/Nw3YS6GL6v3s9EuwLpowC7Q9HMrDcLupbiZds_vmilg",
        "https://arweave.net/rCgqUIqBe2IU6H01132VTbpP1DRq-EYfkaZ2nZm7hg4",
        "https://arweave.net/fNacoielqaAJ6ZhQPSol14q3MQ-Nsv3-HXtdB_l-Br8",
        "https://arweave.net/TBGTMYdMeYtM4AP-oXQa16IAeFHMdx8UKbYaVRc9QiM",
        "https://arweave.net/h5xdkNKLkeHwi9hr3zLrzvjkaDaJxSKFS-IAU7Jsn-Q",
        "https://arweave.net/5mBodIfGsRmQQhfvD4poENvfe9P1Q2qYguvRfGRX258",
        "https://arweave.net/7eb0hTcPkTDhvgiNNPulU8o3m4JIdvAG6W7y0eYnAfc",
        "https://arweave.net/9yw98EJZb91naZTNjm9UXQv0QE-GH8KHuT92qNjXgL4",
        "https://arweave.net/Jn4RFwC3eAqdske_V6BRC1OPUrOzOvjGZfaEvaI_TcU",
        "https://arweave.net/P1ik_cR518oxY0Am2nhlxvkCMkZTmE9LzgKrzt7k380",
        "https://arweave.net/HUOEqmkgB0rM57v2IIsfFrgNvDkfU69SNRmU-22GRt4",
        "https://arweave.net/tpoVVlg0CQ23EyiihSpIwrodY7-WQhX1qFHLwYRwCsk",
        "https://arweave.net/PdEUIk_ZvGduiCPuMpy5qS0ocJJZm5_y1AqdvC0kqU0",
        "https://arweave.net/Euc5cUW7qIUV7EmRvwxmFkrqjbvOghSWFupBtTe3Kw8",
    ];

    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .mint_non_fungible(
            water_bear_resource,
            [
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 1)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 1)), key_image_url: Url::of(key_image_urls[1]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 2)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 2)), key_image_url: Url::of(key_image_urls[2]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 3)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 3)), key_image_url: Url::of(key_image_urls[3]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 4)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 4)), key_image_url: Url::of(key_image_urls[4]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 5)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 5)), key_image_url: Url::of(key_image_urls[5]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 6)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 6)), key_image_url: Url::of(key_image_urls[6]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 7)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 7)), key_image_url: Url::of(key_image_urls[7]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 8)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 8)), key_image_url: Url::of(key_image_urls[8]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 9)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 9)), key_image_url: Url::of(key_image_urls[9]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 10)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 10)), key_image_url: Url::of(key_image_urls[10]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 11)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 11)), key_image_url: Url::of(key_image_urls[11]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 12)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 12)), key_image_url: Url::of(key_image_urls[12]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 13)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 13)), key_image_url: Url::of(key_image_urls[13]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 14)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 14)), key_image_url: Url::of(key_image_urls[14]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 15)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 15)), key_image_url: Url::of(key_image_urls[15]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 16)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 16)), key_image_url: Url::of(key_image_urls[16]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 17)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 17)), key_image_url: Url::of(key_image_urls[17]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 18)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 18)), key_image_url: Url::of(key_image_urls[18]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 19)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 19)), key_image_url: Url::of(key_image_urls[19]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
                (NonFungibleLocalId::String(StringNonFungibleLocalId::new(format!("WaterBearsDAO_{}", 20)).unwrap(),), WaterBear { name: String::from(format!("WaterBears DAO #{}", 20)), key_image_url: Url::of(key_image_urls[20]), base: "DAO".into(), background: "".into(), mouth: "".into(), hats: "".into(), neck: "".into(), eyes: "".into(), clothes: "".into() }),
            ]
        )
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("50. update minter"),
        network,
    );
}
