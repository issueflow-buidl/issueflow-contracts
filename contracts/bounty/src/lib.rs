#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, Vec, Map
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    BountyCounter,
    Bounty(u64),
    UserBounties(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct Bounty {
    pub id: u64,
    pub creator: Address,
    pub amount: i128,
    pub description: Symbol,
    pub is_active: bool,
    pub claimer: Option<Address>,
}

#[contract]
pub struct BountyContract;

#[contractimpl]
impl BountyContract {
    /// Create a new bounty
    pub fn create_bounty(
        env: Env,
        creator: Address,
        amount: i128,
        description: Symbol,
    ) -> u64 {
        creator.require_auth();
        
        // Get next bounty ID
        let counter_key = DataKey::BountyCounter;
        let bounty_id: u64 = env.storage().instance()
            .get(&counter_key)
            .unwrap_or(0);
        let next_id = bounty_id + 1;
        
        // Create bounty
        let bounty = Bounty {
            id: next_id,
            creator: creator.clone(),
            amount,
            description,
            is_active: true,
            claimer: None,
        };
        
        // Store bounty
        let bounty_key = DataKey::Bounty(next_id);
        env.storage().persistent().set(&bounty_key, &bounty);
        
        // Update counter
        env.storage().instance().set(&counter_key, &next_id);
        
        // Add to user's bounties
        let user_bounties_key = DataKey::UserBounties(creator);
        let mut user_bounties: Vec<u64> = env.storage().persistent()
            .get(&user_bounties_key)
            .unwrap_or(Vec::new(&env));
        user_bounties.push_back(next_id);
        env.storage().persistent().set(&user_bounties_key, &user_bounties);
        
        next_id
    }
    
    /// Claim a bounty
    pub fn claim_bounty(
        env: Env,
        bounty_id: u64,
        claimer: Address,
    ) -> bool {
        claimer.require_auth();
        
        let bounty_key = DataKey::Bounty(bounty_id);
        let mut bounty: Bounty = env.storage().persistent()
            .get(&bounty_key)
            .expect("Bounty not found");
        
        if !bounty.is_active {
            panic!("Bounty is not active");
        }
        
        if bounty.claimer.is_some() {
            panic!("Bounty already claimed");
        }
        
        // Update bounty
        bounty.claimer = Some(claimer.clone());
        bounty.is_active = false;
        env.storage().persistent().set(&bounty_key, &bounty);
        
        true
    }
    
    /// Cancel a bounty (only by creator)
    pub fn cancel_bounty(
        env: Env,
        bounty_id: u64,
        creator: Address,
    ) -> bool {
        creator.require_auth();
        
        let bounty_key = DataKey::Bounty(bounty_id);
        let mut bounty: Bounty = env.storage().persistent()
            .get(&bounty_key)
            .expect("Bounty not found");
        
        if bounty.creator != creator {
            panic!("Only creator can cancel bounty");
        }
        
        if !bounty.is_active {
            panic!("Bounty is not active");
        }
        
        if bounty.claimer.is_some() {
            panic!("Cannot cancel claimed bounty");
        }
        
        // Deactivate bounty
        bounty.is_active = false;
        env.storage().persistent().set(&bounty_key, &bounty);
        
        true
    }
    
    /// Get bounty details
    pub fn get_bounty(env: Env, bounty_id: u64) -> Option<Bounty> {
        let bounty_key = DataKey::Bounty(bounty_id);
        env.storage().persistent().get(&bounty_key)
    }
    
    /// Get user's bounties
    pub fn get_user_bounties(env: Env, user: Address) -> Vec<u64> {
        let user_bounties_key = DataKey::UserBounties(user);
        env.storage().persistent()
            .get(&user_bounties_key)
            .unwrap_or(Vec::new(&env))
    }
    
    /// Get total bounty count
    pub fn get_bounty_count(env: Env) -> u64 {
        let counter_key = DataKey::BountyCounter;
        env.storage().instance().get(&counter_key).unwrap_or(0)
    }

    pub fn cancel_bounty(env: Env, token: Address, maintainer: Address, amount: i128) {
        maintainer.require_auth();
        let client = token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &maintainer, &amount);
    }
}
