#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,
};

#[test]
fn test_create_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let maintainer = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&maintainer, &amount, &token);
    
    assert_eq!(bounty_id, 1);
    
    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.id, 1);
    assert_eq!(bounty.maintainer, maintainer);
    assert_eq!(bounty.amount, amount);
    assert_eq!(bounty.token, token);
    assert_eq!(bounty.is_claimed, false);
    assert_eq!(bounty.is_cancelled, false);
}

#[test]
fn test_claim_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let maintainer = Address::generate(&env);
    let claimant = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&maintainer, &amount, &token);
    client.claim_bounty(&bounty_id, &claimant);
    
    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.is_claimed, true);
    assert_eq!(bounty.is_cancelled, false);
}

#[test]
fn test_cancel_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let maintainer = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&maintainer, &amount, &token);
    client.cancel_bounty(&bounty_id);
    
    let bounty = client.get_bounty(&bounty_id);
    assert_eq!(bounty.is_claimed, false);
    assert_eq!(bounty.is_cancelled, true);
}

#[test]
#[should_panic(expected = "Cannot cancel claimed bounty")]
fn test_cannot_cancel_claimed_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let maintainer = Address::generate(&env);
    let claimant = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&maintainer, &amount, &token);
    client.claim_bounty(&bounty_id, &claimant);
    client.cancel_bounty(&bounty_id);
}

#[test]
#[should_panic(expected = "Bounty already cancelled")]
fn test_cannot_cancel_twice() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let maintainer = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&maintainer, &amount, &token);
    client.cancel_bounty(&bounty_id);
    client.cancel_bounty(&bounty_id);
}

#[test]
#[should_panic(expected = "Bounty is cancelled")]
fn test_cannot_claim_cancelled_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let maintainer = Address::generate(&env);
    let claimant = Address::generate(&env);
    let token = Address::generate(&env);
    let amount = 1000i128;
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&maintainer, &amount, &token);
    client.cancel_bounty(&bounty_id);
    client.claim_bounty(&bounty_id, &claimant);
}