#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct Bounty {
    pub id: u32,
    pub maintainer: Address,
    pub amount: i128,
    pub token: Address,
    pub is_claimed: bool,
    pub is_cancelled: bool,
}

const BOUNTIES: Symbol = symbol_short!("BOUNTIES");
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contract]
pub struct BountyContract;

#[contractimpl]
impl BountyContract {
    pub fn create_bounty(
        env: Env,
        maintainer: Address,
        amount: i128,
        token: Address,
    ) -> u32 {
        maintainer.require_auth();
        
        let id = env.storage().instance().get(&NEXT_ID).unwrap_or(1u32);
        env.storage().instance().set(&NEXT_ID, &(id + 1));
        
        let bounty = Bounty {
            id,
            maintainer: maintainer.clone(),
            amount,
            token: token.clone(),
            is_claimed: false,
            is_cancelled: false,
        };
        
        let mut bounties: Vec<Bounty> = env.storage().instance().get(&BOUNTIES).unwrap_or(Vec::new(&env));
        bounties.push_back(bounty);
        env.storage().instance().set(&BOUNTIES, &bounties);
        
        // Transfer tokens from maintainer to contract
        let contract_address = env.current_contract_address();
        env.invoke_contract::<()>(
            &token,
            &symbol_short!("transfer"),
            soroban_sdk::vec![&env, maintainer.into_val(&env), contract_address.into_val(&env), amount.into_val(&env)]
        );
        
        id
    }
    
    pub fn claim_bounty(
        env: Env,
        bounty_id: u32,
        claimant: Address,
    ) {
        claimant.require_auth();
        
        let mut bounties: Vec<Bounty> = env.storage().instance().get(&BOUNTIES).unwrap_or(Vec::new(&env));
        
        for i in 0..bounties.len() {
            let mut bounty = bounties.get(i).unwrap();
            if bounty.id == bounty_id {
                if bounty.is_claimed {
                    panic!("Bounty already claimed");
                }
                if bounty.is_cancelled {
                    panic!("Bounty is cancelled");
                }
                
                bounty.is_claimed = true;
                bounties.set(i, bounty.clone());
                env.storage().instance().set(&BOUNTIES, &bounties);
                
                // Transfer tokens to claimant
                env.invoke_contract::<()>(
                    &bounty.token,
                    &symbol_short!("transfer"),
                    soroban_sdk::vec![&env, env.current_contract_address().into_val(&env), claimant.into_val(&env), bounty.amount.into_val(&env)]
                );
                
                return;
            }
        }
        
        panic!("Bounty not found");
    }
    
    pub fn cancel_bounty(
        env: Env,
        bounty_id: u32,
    ) {
        let mut bounties: Vec<Bounty> = env.storage().instance().get(&BOUNTIES).unwrap_or(Vec::new(&env));
        
        for i in 0..bounties.len() {
            let mut bounty = bounties.get(i).unwrap();
            if bounty.id == bounty_id {
                bounty.maintainer.require_auth();
                
                if bounty.is_claimed {
                    panic!("Cannot cancel claimed bounty");
                }
                if bounty.is_cancelled {
                    panic!("Bounty already cancelled");
                }
                
                bounty.is_cancelled = true;
                bounties.set(i, bounty.clone());
                env.storage().instance().set(&BOUNTIES, &bounties);
                
                // Return tokens to maintainer
                env.invoke_contract::<()>(
                    &bounty.token,
                    &symbol_short!("transfer"),
                    soroban_sdk::vec![&env, env.current_contract_address().into_val(&env), bounty.maintainer.into_val(&env), bounty.amount.into_val(&env)]
                );
                
                return;
            }
        }
        
        panic!("Bounty not found");
    }
    
    pub fn get_bounty(env: Env, bounty_id: u32) -> Bounty {
        let bounties: Vec<Bounty> = env.storage().instance().get(&BOUNTIES).unwrap_or(Vec::new(&env));
        
        for i in 0..bounties.len() {
            let bounty = bounties.get(i).unwrap();
            if bounty.id == bounty_id {
                return bounty;
            }
        }
        
        panic!("Bounty not found");
    }
    
    pub fn get_all_bounties(env: Env) -> Vec<Bounty> {
        env.storage().instance().get(&BOUNTIES).unwrap_or(Vec::new(&env))
    }

    pub fn cancel_bounty(env: Env, token: Address, maintainer: Address, amount: i128) {
        maintainer.require_auth();
        let client = token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &maintainer, &amount);
    }
}
