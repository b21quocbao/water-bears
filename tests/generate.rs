use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

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

    let contract_package = PackageAddress::try_from_bech32(
        &net_decoder,
        "package_tdx_2_1p495kgzwts767tmapdtknvhdg9uuepvjklnjhxfr3snuy9zp6hh4ev",
    )
    .unwrap();

    let nft_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_tdx_2_1cqh6ktv53ktlq9fcjwmerh55jee86qyg23w9avfl6vk04nmlzqlc7g",
    )
    .unwrap();
    let owner_badge_resource: ResourceAddress = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1thgpxr4h0hh65wt6k375sua3quefw4ynzhtag3c73guxavafldc4fj",
    )
    .unwrap();
    let nft_resource = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1n2khts5x5l632l9ehtm0724erlh8rmch0xu7fq5q8mrfcsftnkvj89",
    )
    .unwrap();

    let stake_pool_component = ComponentAddress::try_from_bech32(
        &net_decoder,
        "component_tdx_2_1cz5wzsgmcwwhx4vl5e70jtenjn9mf28hyxn8ung2kwmlkdutttl5s2",
    )
    .unwrap();
    let id_resource = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1nfk3devv4zat0dgn0q00v9fe0ggpmkuaeq8wtzl662m60wuvs59dml",
    )
    .unwrap();
    let dna_resource = ResourceAddress::try_from_bech32(
        &net_decoder,
        "resource_tdx_2_1tk706nn7fehxxenug2re2u8pkzf5a0zjgu7ea6up7hjs2whfu6nu7g",
    )
    .unwrap();

    // let network = &NetworkDefinition::mainnet();
    // let net_decoder = AddressBech32Decoder::new(network);
    // let account = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "account_rdx12xv9rcgd5l4cyt9tx0ghzdryl4kkalktlhxe94uy4szy5pa9xly7ky",
    // )
    // .unwrap();
    // let buy_account = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "account_rdx12xv9rcgd5l4cyt9tx0ghzdryl4kkalktlhxe94uy4szy5pa9xly7ky",
    // )
    // .unwrap();
    // let owner_badge_resource: ResourceAddress = ResourceAddress::try_from_bech32(
    //     &net_decoder,
    //     "resource_rdx1t5jsrm3snfaasqfpn8xven7f7ywp7pv2396gmu9qtdzg6aspw3c6c9",
    // )
    // .unwrap();
    // let hug_resource = ResourceAddress::try_from_bech32(
    //     &net_decoder,
    //     "resource_rdx1t5kmyj54jt85malva7fxdrnpvgfgs623yt7ywdaval25vrdlmnwe97",
    // )
    // .unwrap();
    // let contract_package = PackageAddress::try_from_bech32(
    //     &net_decoder,
    //     "package_rdx1p45tdrzx59a0kcnrp79fva8njq7auu8fsfc0qkd74fzywnfnmv46zs",
    // )
    // .unwrap();
    // let nft_component = ComponentAddress::try_from_bech32(
    //     &net_decoder,
    //     "component_rdx1cz8sufk6wunsnnnh5ckrhrn3n6stl4ve6dctrphaa8zf3q06kle6hv",
    // )
    // .unwrap();

    // instantiate_game_nft
    let manifest = ManifestBuilder::new()
        .call_function(
            contract_package,
            "MemeNft",
            "instantiate_meme_nft",
            manifest_args!(),
        )
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("1. instantiate_game_nft"),
        network,
    );

    // // mint_nft
    // let manifest = ManifestBuilder::new()
    //     .create_proof_from_account_of_amount(account_component, owner_badge_resource, dec!(1))
    //     .call_method(
    //         nft_component,
    //         "mint_nft",
    //         (
    //             dec!(1),
    //             "https://arweave.net/KWKvTzCxWuS6KoJJeBkT-lK_r329ucGnbiLZ9M8fllE",
    //             "pink",
    //             "Base",
    //             "Lava",
    //             "Topper",
    //             "",
    //             "",
    //             "rune armor",
    //         ),
    //     );
    // let _ = dump_manifest_to_file_system(
    //     manifest.object_names(),
    //     &manifest.build(),
    //     "./transaction_manifest",
    //     Some("2. mint_nft"),
    //     network,
    // );

    // update_payment
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge_resource, dec!(1))
        .call_method(nft_component, "update_payment", (XRD, dec!(250)));
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("3. update_payment"),
        network,
    );

    // buy_nft
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(buy_account_component, XRD, dec!(250))
        .take_all_from_worktop(XRD, "nft_bucket")
        .call_method_with_name_lookup(nft_component, "buy_nft", |lookup| {
            (lookup.bucket("nft_bucket"),)
        })
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("4. buy_nft"),
        network,
    );

    // withdraw_from_vault
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge_resource, dec!(1))
        .call_method(nft_component, "withdraw_from_vault", (XRD,))
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("5. withdraw_from_vault"),
        network,
    );

    // instantiate_stake_pool
    let manifest = ManifestBuilder::new()
        .call_function(
            contract_package,
            "StakePool",
            "instantiate_stake_pool",
            manifest_args!(nft_resource, owner_badge_resource, dna_resource),
        )
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("6. instantiate_stake_pool"),
        network,
    );

    // deposit_reward_pool
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(account_component, dna_resource, dec!(250))
        .take_all_from_worktop(dna_resource, "dna_bucket")
        .call_method_with_name_lookup(stake_pool_component, "deposit_reward_pool", |lookup| {
            (lookup.bucket("dna_bucket"),)
        });
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("7. deposit_reward_pool"),
        network,
    );

    // create_id
    let manifest = ManifestBuilder::new()
        .call_method(stake_pool_component, "create_id", ())
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("8. create_id"),
        network,
    );

    // stake
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof_1")
        .withdraw_non_fungible_from_account(
            account_component,
            NonFungibleGlobalId::new(
                nft_resource,
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
            ),
        )
        .take_all_from_worktop(nft_resource, "nft_bucket_1")
        .call_method_with_name_lookup(stake_pool_component, "stake", |lookup| {
            (lookup.bucket("nft_bucket_1"), lookup.proof("proof_1"))
        });
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("9. stake"),
        network,
    );

    // update_reward_factor
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge_resource, dec!(1))
        .call_method(stake_pool_component, "update_reward_factor", (dec!(5.0),));
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("10. update_reward_factor"),
        network,
    );

    // withdraw
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            account_component,
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
        .deposit_batch(buy_account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("11. withdraw"),
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

    // withdraw_reward_pool
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account_component, owner_badge_resource, dec!(1))
        .call_method(stake_pool_component, "withdraw_reward_pool", (dec!(230),))
        .deposit_batch(account_component);
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./transaction_manifest",
        Some("13. withdraw_reward_pool"),
        network,
    );
}
