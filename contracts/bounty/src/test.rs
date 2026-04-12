#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_bounty_flow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);

    let maintainer = Address::generate(&env);
    let contributor = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;

    env.mock_all_auths();

    // Create bounty
    let bounty_id = client.create_bounty(&maintainer, &contributor, &amount, &token);
    assert_eq!(bounty_id, 1);

    // Verify bounty creation
    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.id, bounty_id);
    assert_eq!(bounty.maintainer, maintainer);
    assert_eq!(bounty.contributor, contributor);
    assert_eq!(bounty.amount, amount);
    assert_eq!(bounty.token, token);
    assert!(!bounty.approved);
    assert!(!bounty.claimed);

    // Approve bounty
    client.approve_bounty(&bounty_id);
    let bounty = client.get_bounty(&bounty_id);
    assert!(bounty.approved);
    assert!(!bounty.claimed);

    // Claim bounty
    client.claim_bounty(&bounty_id);
    let bounty = client.get_bounty(&bounty_id);
    assert!(bounty.approved);
    assert!(bounty.claimed);
}

#[test]
#[should_panic(expected = "Bounty not approved by maintainer")]
fn test_claim_unapproved_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);

    let maintainer = Address::generate(&env);
    let contributor = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;

    env.mock_all_auths();

    // Create bounty
    let bounty_id = client.create_bounty(&maintainer, &contributor, &amount, &token);

    // Try to claim without approval
    client.claim_bounty(&bounty_id);
}

#[test]
#[should_panic(expected = "Bounty already claimed")]
fn test_claim_already_claimed_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);

    let maintainer = Address::generate(&env);
    let contributor = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;

    env.mock_all_auths();

    // Create and approve bounty
    let bounty_id = client.create_bounty(&maintainer, &contributor, &amount, &token);
    client.approve_bounty(&bounty_id);

    // Claim bounty first time
    client.claim_bounty(&bounty_id);

    // Try to claim again
    client.claim_bounty(&bounty_id);
}