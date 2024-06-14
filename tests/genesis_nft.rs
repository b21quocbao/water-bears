use radix_common::types::{Epoch, Round};
use scrypto_test::prelude::*;

#[test]
fn test() {
    // Setup the environment
    let mut ledger = LedgerSimulatorBuilder::new()
        .with_custom_genesis(CustomGenesis::default(
            Epoch::of(1),
            CustomGenesis::default_consensus_manager_config(),
        ))
        .build();

    // Create an account
    let (public_key, _private_key, account_component) = ledger.new_allocated_account();

    // Publish package
    let package_address = ledger.compile_and_publish(this_package!());

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .get_free_xrd_from_faucet()
        .take_from_worktop(XRD, dec!(1000), "bucket_1")
        .try_deposit_entire_worktop_or_abort(account_component, None)
        .try_deposit_batch_or_abort(account_component, ["bucket_1"], None)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // Test the `instantiate_meme_nft` function.
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "MemeNft",
            "instantiate_meme_nft",
            manifest_args!(),
        )
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();
    let owner_badge = receipt.expect_commit(true).new_resource_addresses()[0];
    let water_bear_resource = receipt.expect_commit(true).new_resource_addresses()[1];
    let water_bear_component = receipt.expect_commit(true).new_component_addresses()[0];

    // update_payment
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(water_bear_component, "update_payment", (XRD, dec!(250)))
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // mint_nft
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(
            water_bear_component,
            "mint_nft",
            (
                [dec!(1), dec!(2), dec!(3), dec!(4), dec!(5)],
                [
                    "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
                    "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
                    "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
                    "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
                    "https://arweave.net/SJy8goVGtZFy8FOgS3h8-RlyZzIwk8RucGMQ7e3N04c",
                ],
                ["Green", "Green", "Green", "Green", "Green"],
                ["Red", "Red", "Red", "Red", "Red"],
                ["", "", "", "", ""],
                ["", "", "", "", ""],
                ["", "", "", "", ""],
                ["", "", "", "", ""],
                ["", "", "", "", ""],
            ),
        )
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // buy_nft
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account_component, XRD, dec!(750))
        .take_all_from_worktop(XRD, "nft_bucket_1")
        .call_method_with_name_lookup(water_bear_component, "buy_nft", |lookup| {
            (lookup.bucket("nft_bucket_1"), dec!(3))
        })
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // withdraw_nft
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(water_bear_component, "withdraw_nft", (dec!(2),))
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // withdraw_from_vault
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(water_bear_component, "withdraw_from_vault", (XRD,))
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // Test the `instantiate_stake_pool` function.
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_fungible_resource(
            OwnerRole::Fixed(rule!(require(owner_badge))),
            true,
            18,
            FungibleResourceRoles {
                mint_roles: mint_roles! {
                    minter => OWNER;
                    minter_updater => OWNER;
                },
                burn_roles: burn_roles! {
                    burner => OWNER;
                    burner_updater => OWNER;
                },
                ..Default::default()
            },
            Default::default(),
            Option::<Decimal>::from(dec!(1000000)),
        )
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    let dna_token = receipt.expect_commit(true).new_resource_addresses()[0];

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "StakePool",
            "instantiate_stake_pool",
            manifest_args!(water_bear_resource, owner_badge, dna_token),
        )
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();
    let stake_pool_component = receipt.expect_commit(true).new_component_addresses()[0];
    let id_resource = receipt.expect_commit(true).new_resource_addresses()[0];

    // deposit_reward_pool
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(account_component, dna_token, dec!(250))
        .take_all_from_worktop(dna_token, "bucket")
        .call_method_with_name_lookup(stake_pool_component, "deposit_reward_pool", |lookup| {
            (lookup.bucket("bucket"),)
        })
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // create_id
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(stake_pool_component, "create_id", ())
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // stake nft
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungibles(
            account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof_1")
        .withdraw_non_fungible_from_account(
            account_component,
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
            account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof_2")
        .withdraw_non_fungible_from_account(
            account_component,
            NonFungibleGlobalId::new(
                water_bear_resource,
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_2").unwrap()),
            ),
        )
        .take_all_from_worktop(water_bear_resource, "nft_bucket_2")
        .call_method_with_name_lookup(stake_pool_component, "stake", |lookup| {
            (lookup.bucket("nft_bucket_2"), lookup.proof("proof_2"))
        })
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    let date_ms =
        ledger.get_current_proposer_timestamp_ms() + 3 * 24 * 3600 * 1000 + 12 * 3600 * 1000;
    let receipt = ledger.advance_to_round_at_timestamp(Round::of(1), date_ms);
    receipt.expect_commit_success();

    // update_reward_factor
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(stake_pool_component, "update_reward_factor", (dec!(4.9),))
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // claim_rewards
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungibles(
            account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof")
        .call_method_with_name_lookup(stake_pool_component, "claim_rewards", |lookup| {
            (lookup.proof("proof"),)
        })
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // withdraw
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
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
        .create_proof_from_account_of_non_fungibles(
            account_component,
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
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    let date_ms =
        ledger.get_current_proposer_timestamp_ms() + 10 * 24 * 3600 * 1000 + 12 * 3600 * 1000;
    let receipt = ledger.advance_to_round_at_timestamp(Round::of(1), date_ms);
    receipt.expect_commit_success();

    // claim_rewards
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungibles(
            account_component,
            id_resource,
            indexset!(NonFungibleLocalId::integer(1)),
        )
        .pop_from_auth_zone("proof")
        .call_method_with_name_lookup(stake_pool_component, "claim_rewards", |lookup| {
            (lookup.proof("proof"),)
        })
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // withdraw_reward_pool
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(stake_pool_component, "withdraw_reward_pool", (dec!(210),))
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // withdraw_all_reward_pool
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(stake_pool_component, "withdraw_all_reward_pool", ())
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // update_reward_address
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(stake_pool_component, "update_reward_address", (XRD,))
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // Test the `instantiate_breed_nft` function.
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "BreedNft",
            "instantiate_breed_nft",
            manifest_args!(owner_badge, water_bear_resource, dec!(2)),
        )
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();
    let test_tube_component = receipt.expect_commit(true).new_component_addresses()[0];
    let test_tube_nft = receipt.expect_commit(true).new_resource_addresses()[0];

    // update burner
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .set_role(
            dna_token,
            ObjectModuleId::Main,
            RoleKey::from("burner"),
            rule!(require(global_caller(test_tube_component))),
        )
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // update_payment
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .call_method(
            test_tube_component,
            "update_payment",
            (dna_token, dec!(300)),
        )
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // buy_nft
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_non_fungibles(
            account_component,
            water_bear_resource,
            indexset!(
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_1").unwrap()),
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("WaterBear_2").unwrap()),
            ),
        )
        .pop_from_auth_zone("proof_1")
        .withdraw_from_account(account_component, dna_token, dec!(900))
        .take_all_from_worktop(dna_token, "bucket")
        .call_method_with_name_lookup(test_tube_component, "buy_nft", |lookup| {
            (lookup.bucket("bucket"), dec!(3), lookup.proof("proof_1"))
        })
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // Test the `instantiate_baby_nft` function.
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "BabyNft",
            "instantiate_baby_nft",
            manifest_args!(owner_badge, test_tube_nft),
        )
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();
    let baby_component = receipt.expect_commit(true).new_component_addresses()[0];

    // update burner
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_proof_from_account_of_amount(account_component, owner_badge, dec!(1))
        .set_role(
            test_tube_nft,
            ObjectModuleId::Main,
            RoleKey::from("burner"),
            rule!(require(global_caller(baby_component))),
        )
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // breed
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_non_fungibles_from_account(
            account_component,
            test_tube_nft,
            indexset!(
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("TestTube_1").unwrap()),
                NonFungibleLocalId::String(StringNonFungibleLocalId::new("TestTube_2").unwrap()),
            ),
        )
        .take_all_from_worktop(test_tube_nft, "bucket")
        .call_method_with_name_lookup(baby_component, "breed", |lookup| (lookup.bucket("bucket"),))
        .deposit_batch(account_component)
        .build();
    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();
}
