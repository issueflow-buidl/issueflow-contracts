#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token::{Client as TokenClient, StellarAssetClient as StellarAssetClientTrait},
    Address, Env, IntoVal
};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, Address) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        contract_address,
    )
}

#[test]
fn test_create_bounty() {
    let env = Env::default();
    env.mock_all_auths();

    let creator = Address::generate(&env);
    let admin = Address::generate(&env);
    
    // Create token contract
    let (token, token_address) = create_token_contract(&env, &admin);
    
    // Mint tokens to creator
    let stellar_asset = StellarAssetClientTrait::new(&env, &token_address);
    stellar_asset.mint(&creator, &1000);
    
    // Create bounty contract
    let bounty_contract_id = env.register_contract(None, BountyContract);
    let bounty_client = BountyContractClient::new(&env, &bounty_contract_id);
    
    // Create bounty
    let amount = 100i128;
    let description = symbol_short!("test");
    
    let bounty_id = bounty_client.create_bounty(
        &creator,
        &amount,
        &token_address,
        &description,
    );
    
    assert_eq!(bounty_id, 1u64);
    
    // Verify bounty was created correctly
    let bounty = bounty_client.get_bounty(&bounty_id).unwrap();
    assert_eq!(bounty.id, 1u64);
    assert_eq!(bounty.creator, creator);
    assert_eq!(bounty.amount, amount);
    assert_eq!(bounty.token, token_address);
    assert_eq!(bounty.description, description);
    assert_eq!(bounty.is_active, true);
    assert_eq!(bounty.claimant, None);
    
    // Verify tokens were transferred to contract
    assert_eq!(token.balance(&bounty_contract_id), amount);
    assert_eq!(token.balance(&creator), 1000 - amount);
}