#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal
};

#[test]
fn test_create_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let amount = 1000i128;
    let description = symbol_short!("test");
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&creator, &amount, &description);
    
    assert_eq!(bounty_id, 1);
    
    let bounty = client.get_bounty(&bounty_id).unwrap();
    assert_eq!(bounty.id, 1);
    assert_eq!(bounty.creator, creator);
    assert_eq!(bounty.amount, amount);
    assert_eq!(bounty.description, description);
    assert_eq!(bounty.is_active, true);
    assert_eq!(bounty.claimer, None);
}

#[test]
fn test_claim_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let claimer = Address::generate(&env);
    let amount = 1000i128;
    let description = symbol_short!("test");
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&creator, &amount, &description);
    let result = client.claim_bounty(&bounty_id, &claimer);
    
    assert_eq!(result, true);
    
    let bounty = client.get_bounty(&bounty_id).unwrap();
    assert_eq!(bounty.is_active, false);
    assert_eq!(bounty.claimer, Some(claimer));
}

#[test]
fn test_cancel_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let amount = 1000i128;
    let description = symbol_short!("test");
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&creator, &amount, &description);
    let result = client.cancel_bounty(&bounty_id, &creator);
    
    assert_eq!(result, true);
    
    let bounty = client.get_bounty(&bounty_id).unwrap();
    assert_eq!(bounty.is_active, false);
    assert_eq!(bounty.claimer, None);
}

#[test]
#[should_panic(expected = "Bounty already claimed")]
fn test_claim_already_claimed_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let claimer1 = Address::generate(&env);
    let claimer2 = Address::generate(&env);
    let amount = 1000i128;
    let description = symbol_short!("test");
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&creator, &amount, &description);
    client.claim_bounty(&bounty_id, &claimer1);
    client.claim_bounty(&bounty_id, &claimer2); // Should panic
}

#[test]
#[should_panic(expected = "Cannot cancel claimed bounty")]
fn test_cancel_claimed_bounty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let claimer = Address::generate(&env);
    let amount = 1000i128;
    let description = symbol_short!("test");
    
    env.mock_all_auths();
    
    let bounty_id = client.create_bounty(&creator, &amount, &description);
    client.claim_bounty(&bounty_id, &claimer);
    client.cancel_bounty(&bounty_id, &creator); // Should panic
}

#[test]
fn test_get_user_bounties() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let amount = 1000i128;
    let description = symbol_short!("test");
    
    env.mock_all_auths();
    
    let bounty_id1 = client.create_bounty(&creator, &amount, &description);
    let bounty_id2 = client.create_bounty(&creator, &amount, &description);
    
    let user_bounties = client.get_user_bounties(&creator);
    assert_eq!(user_bounties.len(), 2);
    assert_eq!(user_bounties.get(0).unwrap(), bounty_id1);
    assert_eq!(user_bounties.get(1).unwrap(), bounty_id2);
}

#[test]
fn test_get_bounty_count() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BountyContract);
    let client = BountyContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let amount = 1000i128;
    let description = symbol_short!("test");
    
    env.mock_all_auths();
    
    assert_eq!(client.get_bounty_count(), 0);
    
    client.create_bounty(&creator, &amount, &description);
    assert_eq!(client.get_bounty_count(), 1);
    
    client.create_bounty(&creator, &amount, &description);
    assert_eq!(client.get_bounty_count(), 2);
}