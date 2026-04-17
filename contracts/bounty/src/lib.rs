#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, vec, Address, Env, Map, Symbol, Val, Vec
};

#[derive(Clone)]
#[contracttype]
pub struct Bounty {
    pub id: u64,
    pub creator: Address,
    pub amount: i128,
    pub token: Address,
    pub description: Symbol,
    pub is_active: bool,
    pub claimant: Option<Address>,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    BountyCounter,
    Bounty(u64),
}

#[contract]
pub struct BountyContract;

#[contractimpl]
impl BountyContract {
    pub fn create_bounty(
        env: Env,
        creator: Address,
        amount: i128,
        token: Address,
        description: Symbol,
    ) -> u64 {
        creator.require_auth();
        
        // Get the next bounty ID
        let counter_key = DataKey::BountyCounter;
        let bounty_id: u64 = env
            .storage()
            .instance()
            .get(&counter_key)
            .unwrap_or(0u64) + 1;
        
        // Transfer tokens from creator to contract
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&creator, &env.current_contract_address(), &amount);
        
        // Create bounty struct
        let bounty = Bounty {
            id: bounty_id,
            creator: creator.clone(),
            amount,
            token: token.clone(),
            description,
            is_active: true,
            claimant: None,
        };
        
        // Store bounty
        let bounty_key = DataKey::Bounty(bounty_id);
        env.storage().instance().set(&bounty_key, &bounty);
        
        // Update counter
        env.storage().instance().set(&counter_key, &bounty_id);
        
        bounty_id
    }
    
    pub fn get_bounty(env: Env, bounty_id: u64) -> Option<Bounty> {
        let bounty_key = DataKey::Bounty(bounty_id);
        env.storage().instance().get(&bounty_key)
    }

    pub fn cancel_bounty(env: Env, token: Address, maintainer: Address, amount: i128) {
        maintainer.require_auth();
        let client = token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &maintainer, &amount);
    }
}
