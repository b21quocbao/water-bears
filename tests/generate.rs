use scrypto_test::prelude::*;
use scrypto_test::utils::dump_manifest_to_file_system;

#[test]
fn test_hello() {
    let network = &NetworkDefinition::stokenet();
    let net_decoder = AddressBech32Decoder::new(network);
    let account_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "account_tdx_2_12y02z98matcdkewlqd2jhckvwfkl9r4lqhmfcmq07cfdcx4qx5rex9",
    )
    .unwrap();
    let buy_account_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "account_tdx_2_128h3tgatzl2enh3k5ytnxxrea835f2jjt8dwk329fjwxzf02fn96dt",
    )
    .unwrap();

    let package_address = PackageAddress::try_from_bech32(
        &net_decoder,
        "package_tdx_2_1p5tnjevtetwpupam8jzdgkt0xe7x38c7c2dukrqpwmnhuxc9ncm95c",
    )
    .unwrap();

    let water_bear_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_tdx_2_1crlhtxmur02mz25ly9g682z00daxw0nrlkr6vj0jccdntpkrg4r5ap",
    )
    .unwrap();
    let owner_badge: ResourceAddress = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1t5x73kmmck9srd4u7f48wp849929dakyydusz6qk4tths2zznj9gw4",
    )
    .unwrap();
    let water_bear_resource = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1n2k6qtnlvl65dhhlf45sjn5uwzxmf7ny9qnezv7v8c47pyhvtua7y9",
    )
    .unwrap();

    let stake_pool_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_tdx_2_1crqntmap3fztduqaxug0ne3xzwd2mpf2yn9uvp9y65jaslz8jvjz0v",
    )
    .unwrap();
    let id_resource = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1ntr9s8ea8m6k5dqe6hlw8wh9cl9vpjf37me7upsc33paws5s9j6hug",
    )
    .unwrap();
    let dna_token = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1th9lefh8vkxc428kthlneedvrytvnxcm20hgjrhjts0sklc650j0z5",
    )
    .unwrap();

    let test_tube_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_tdx_2_1cqcdpckfc5h9ym9wtwp3wr6tkys0cckw6eap3qawur5fv5wn8hak65",
    )
    .unwrap();
    let test_tube_nft = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1ngpva0vcqfkylq4qhuxqwrqqjl7elfk03yv6e5hwng08lm6xaglefh",
    )
    .unwrap();
    let baby_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_tdx_2_1cr8y0cctj5fg7vnr4u7qmpns37dpxvxk0n22vadz98x4ur5wf44gvs",
    )
    .unwrap();

    // let network = &NetworkDefinition::mainnet();
    // let net_decoder = AddressBech32Decoder::new(network);
    // let account_component = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "account_tdx_2_12y02z98matcdkewlqd2jhckvwfkl9r4lqhmfcmq07cfdcx4qx5rex9",
    // )
    // .unwrap();
    // let buy_account_component = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "account_tdx_2_128h3tgatzl2enh3k5ytnxxrea835f2jjt8dwk329fjwxzf02fn96dt",
    // )
    // .unwrap();

    // let package_address = PackageAddress::try_from_bech32(
    //     &net_decoder,
    //     "package_tdx_2_1p5tnjevtetwpupam8jzdgkt0xe7x38c7c2dukrqpwmnhuxc9ncm95c",
    // )
    // .unwrap();

    // let water_bear_component = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "component_tdx_2_1crlhtxmur02mz25ly9g682z00daxw0nrlkr6vj0jccdntpkrg4r5ap",
    // )
    // .unwrap();
    // let owner_badge: ResourceAddress = ResourceAddress::try_from_bech32(
    //     &net_decoder,
    //     "resource_tdx_2_1t5x73kmmck9srd4u7f48wp849929dakyydusz6qk4tths2zznj9gw4",
    // )
    // .unwrap();
    // let water_bear_resource = ResourceAddress::try_from_bech32(
    //     &net_decoder,
    //     "resource_tdx_2_1n2k6qtnlvl65dhhlf45sjn5uwzxmf7ny9qnezv7v8c47pyhvtua7y9",
    // )
    // .unwrap();

    // let stake_pool_component = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "component_tdx_2_1crqntmap3fztduqaxug0ne3xzwd2mpf2yn9uvp9y65jaslz8jvjz0v",
    // )
    // .unwrap();
    // let id_resource = ResourceAddress::try_from_bech32(
    //     &net_decoder,
    //     "resource_tdx_2_1ntr9s8ea8m6k5dqe6hlw8wh9cl9vpjf37me7upsc33paws5s9j6hug",
    // )
    // .unwrap();
    // let dna_token = ResourceAddress::try_from_bech32(
    //     &net_decoder,
    //     "resource_tdx_2_1th9lefh8vkxc428kthlneedvrytvnxcm20hgjrhjts0sklc650j0z5",
    // )
    // .unwrap();

    // let test_tube_component = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "component_tdx_2_1cqcdpckfc5h9ym9wtwp3wr6tkys0cckw6eap3qawur5fv5wn8hak65",
    // )
    // .unwrap();
    // let test_tube_nft = ResourceAddress::try_from_bech32(
    //     &net_decoder,
    //     "resource_tdx_2_1ngpva0vcqfkylq4qhuxqwrqqjl7elfk03yv6e5hwng08lm6xaglefh",
    // )
    // .unwrap();
    // let baby_component = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "component_tdx_2_1cr8y0cctj5fg7vnr4u7qmpns37dpxvxk0n22vadz98x4ur5wf44gvs",
    // )
    // .unwrap();

    // instantiate_meme_nft
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "MemeNft",
            "instantiate_meme_nft",
            manifest_args!(),
        )
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("1. instantiate_meme_nft"),
        network,
    );

    // update_payment
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(water_bear_component, "update_payment", (XRD, dec!(45)));
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("2. update_payment"),
        network,
    );

    // // mint_nft
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
    //     .call_method(
    //         water_bear_component,
    //         "mint_nft",
    //         (
    //             [dec!(1), dec!(2), dec!(3), dec!(4), dec!(5)],
    //             [
    //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    //                 "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
    //             ],
    //             ["Green", "Green", "Green", "Green", "Green"],
    //             ["Red", "Red", "Red", "Red", "Red"],
    //             ["", "", "", "", ""],
    //             ["", "", "", "", ""],
    //             ["", "", "", "", ""],
    //             ["", "", "", "", ""],
    //             ["", "", "", "", ""],
    //         ),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("3. mint_nft"),
    //     network,
    // );

    // buy_nft
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(buy_account_component, XRD, dec!(750))
        .take_all_from_worktop(XRD, "nft_bucket_1")
        .call_method_with_name_lookup(water_bear_component, "buy_nft", |lookup| {
            (lookup.bucket("nft_bucket_1"), dec!(3))
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("4. buy_nft"),
        network,
    );

    // withdraw_nft
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(water_bear_component, "withdraw_nft", (dec!(2),))
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("5. withdraw_nft"),
        network,
    );

    // withdraw_from_vault
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(water_bear_component, "withdraw_from_vault", (XRD,))
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("6. withdraw_from_vault"),
        network,
    );

    // instantiate_stake_pool
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "StakePool",
            "instantiate_stake_pool",
            manifest_args!(water_bear_resource, owner_badge, dna_token),
        )
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("7. instantiate_stake_pool"),
        network,
    );

    // deposit_reward_pool
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account_component, dna_token, dec!(250))
        .take_all_from_worktop(dna_token, "bucket")
        .call_method_with_name_lookup(stake_pool_component, "deposit_reward_pool", |lookup| {
            (lookup.bucket("bucket"),)
        });
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("8. deposit_reward_pool"),
        network,
    );

    // create_id
    let manifest = ManifestBuilder::new()
        .call_method(stake_pool_component, "create_id", ())
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("9. create_id"),
        network,
    );

    // stake nft
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            buy_account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof_1")
        .withdraw_non_fungible_from_account(
            buy_account_component,
            NonFungibleGlobalId::new(
                water_bear_resource,
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
            ),
        )
        .take_all_from_worktop(water_bear_resource, "nft_bucket_1")
        .call_method_with_name_lookup(stake_pool_component, "stake", |lookup| {
            (lookup.bucket("nft_bucket_1"), lookup.proof("proof_1"))
        })
        .create_proof_from_account_of_non_fungibles(
            buy_account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof_2")
        .withdraw_non_fungible_from_account(
            buy_account_component,
            NonFungibleGlobalId::new(
                water_bear_resource,
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_2").unwrap()),
            ),
        )
        .take_all_from_worktop(water_bear_resource, "nft_bucket_2")
        .call_method_with_name_lookup(stake_pool_component, "stake", |lookup| {
            (lookup.bucket("nft_bucket_2"), lookup.proof("proof_2"))
        });
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("10. nft"),
        network,
    );

    // update_reward_factor
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(stake_pool_component, "update_reward_factor", (dec!(4.9),));
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("11. update_reward_factor"),
        network,
    );

    // claim_rewards
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof")
        .call_method_with_name_lookup(stake_pool_component, "claim_rewards", |lookup| {
            (lookup.proof("proof"),)
        })
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("12. claim_rewards"),
        network,
    );

    // withdraw
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            buy_account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof_1")
        .call_method_with_name_lookup(stake_pool_component, "withdraw", |lookup| {
            (
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
                lookup.proof("proof_1"),
            )
        })
        .create_proof_from_account_of_non_fungibles(
            buy_account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof_2")
        .call_method_with_name_lookup(stake_pool_component, "withdraw", |lookup| {
            (
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_2").unwrap()),
                lookup.proof("proof_2"),
            )
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("13. withdraw"),
        network,
    );

    // claim_rewards
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            buy_account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof")
        .call_method_with_name_lookup(stake_pool_component, "claim_rewards", |lookup| {
            (lookup.proof("proof"),)
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("14. claim_rewards"),
        network,
    );

    // withdraw_reward_pool
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(stake_pool_component, "withdraw_reward_pool", (dec!(210),))
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("15. withdraw_reward_pool"),
        network,
    );

    // withdraw_all_reward_pool
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(stake_pool_component, "withdraw_all_reward_pool", ())
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("16. withdraw_all_reward_pool"),
        network,
    );

    // update_reward_address
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(stake_pool_component, "update_reward_address", (XRD,));
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("17. update_reward_address"),
        network,
    );

    // instantiate_breed_nft
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "BreedNft",
            "instantiate_breed_nft",
            manifest_args!(owner_badge, water_bear_resource, dec!(2)),
        )
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("18. instantiate_breed_nft"),
        network,
    );

    // update burner
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .set_role(
            dna_token,
            ObjectModuleId::Main,
            RoleKey::from("burner"),
            rule!(require(global_caller(test_tube_component))),
        );
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("19. burner"),
        network,
    );

    // update_payment
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(
            test_tube_component,
            "update_payment",
            (dna_token, dec!(300)),
        );
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("20. update_payment"),
        network,
    );

    // buy_nft
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            buy_account_component,
            water_bear_resource,
            indexset!(
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_2").unwrap()),
            ),
        )
        .pop_from_auth_zone("proof_1")
        .withdraw_from_account(buy_account_component, dna_token, dec!(900))
        .take_all_from_worktop(dna_token, "bucket")
        .call_method_with_name_lookup(test_tube_component, "buy_nft", |lookup| {
            (lookup.bucket("bucket"), dec!(3), lookup.proof("proof_1"))
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("21. buy_nft"),
        network,
    );

    // instantiate_baby_nft
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "BabyNft",
            "instantiate_baby_nft",
            manifest_args!(owner_badge, test_tube_nft),
        )
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("22. instantiate_baby_nft"),
        network,
    );

    // update burner
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .set_role(
            test_tube_nft,
            ObjectModuleId::Main,
            RoleKey::from("burner"),
            rule!(require(global_caller(baby_component))),
        );
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("23. burner"),
        network,
    );

    // breed
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(
            buy_account_component,
            test_tube_nft,
            indexset!(
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("TestTube_1").unwrap()),
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("TestTube_2").unwrap()),
            ),
        )
        .take_all_from_worktop(test_tube_nft, "bucket")
        .call_method_with_name_lookup(baby_component, "breed", |lookup| (lookup.bucket("bucket"),))
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("24. breed"),
        network,
    );
}
