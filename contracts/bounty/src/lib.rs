#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bounty {
    pub id: u64,
    pub maintainer: Address,
    pub contributor: Address,
    pub amount: i128,
    pub token: Address,
    pub approved: bool,
    pub claimed: bool,
}

#[contracttype]
pub enum DataKey {
    Bounty(u64),
    NextBountyId,
}

#[contract]
pub struct BountyContract;

#[contractimpl]
impl BountyContract {
    pub fn create_bounty(
        env: Env,
        maintainer: Address,
        contributor: Address,
        amount: i128,
        token: Address,
    ) -> u64 {
        maintainer.require_auth();

        let next_id = env
            .storage()
            .instance()
            .get::<DataKey, u64>(&DataKey::NextBountyId)
            .unwrap_or(1);

        let bounty = Bounty {
            id: next_id,
            maintainer,
            contributor,
            amount,
            token,
            approved: false,
            claimed: false,
        };

        env.storage()
            .instance()
            .set(&DataKey::Bounty(next_id), &bounty);
        env.storage()
            .instance()
            .set(&DataKey::NextBountyId, &(next_id + 1));

        next_id
    }

    pub fn approve_bounty(env: Env, bounty_id: u64) {
        let mut bounty: Bounty = env
            .storage()
            .instance()
            .get(&DataKey::Bounty(bounty_id))
            .expect("Bounty not found");

        bounty.maintainer.require_auth();

        bounty.approved = true;
        env.storage()
            .instance()
            .set(&DataKey::Bounty(bounty_id), &bounty);
    }

    pub fn claim_bounty(env: Env, bounty_id: u64) {
        let mut bounty: Bounty = env
            .storage()
            .instance()
            .get(&DataKey::Bounty(bounty_id))
            .expect("Bounty not found");

        bounty.contributor.require_auth();

        assert!(bounty.approved, "Bounty not approved by maintainer");
        assert!(!bounty.claimed, "Bounty already claimed");

        // Transfer tokens to contributor
        use soroban_sdk::token::Client as TokenClient;
        let token_client = TokenClient::new(&env, &bounty.token);
        token_client.transfer(
            &env.current_contract_address(),
            &bounty.contributor,
            &bounty.amount,
        );

        bounty.claimed = true;
        env.storage()
            .instance()
            .set(&DataKey::Bounty(bounty_id), &bounty);
    }

    pub fn get_bounty(env: Env, bounty_id: u64) -> Bounty {
        env.storage()
            .instance()
            .get(&DataKey::Bounty(bounty_id))
            .expect("Bounty not found")
    }
}