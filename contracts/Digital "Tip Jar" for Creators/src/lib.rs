#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

// We define a key to store our total tips in the contract's storage
#[contracttype]
#[derive(Clone)]
enum StorageKey {
    TotalTips,
    Creator,
    Token,
}

#[contract]
pub struct TipJarContract;

#[contractimpl]
impl TipJarContract {
    /// Initialize the Tip Jar with the creator's address and the token address (e.g., USDC)
    pub fn initialize(env: Env, creator: Address, token: Address) {
        // Ensure this can only be initialized once
        if env.storage().instance().has(&StorageKey::Creator) {
            panic!("Contract already initialized");
        }
        
        env.storage().instance().set(&StorageKey::Creator, &creator);
        env.storage().instance().set(&StorageKey::Token, &token);
        env.storage().instance().set(&StorageKey::TotalTips, &0i128);
    }

    /// The ONE On-Chain Transaction: Fan sends tips to the creator
    pub fn tip(env: Env, fan: Address, amount: i128) {
        // 1. Enforce security: Verify that the fan actually signed this transaction
        fan.require_auth();

        // 2. Retrieve setup data from storage
        let creator: Address = env.storage().instance().get(&StorageKey::Creator).unwrap();
        let token_address: Address = env.storage().instance().get(&StorageKey::Token).unwrap();
        let mut total_tips: i128 = env.storage().instance().get(&StorageKey::TotalTips).unwrap_or(0);

        // 3. Create a client to interact with the standard Token Contract
        let token_client = token::Client::new(&env, &token_address);

        // 4. Execute the transfer: Move tokens directly from Fan to Creator
        token_client.transfer(&fan, &creator, &amount);

        // 5. Update the internal progress counter
        total_tips += amount;
        env.storage().instance().set(&StorageKey::TotalTips, &total_tips);
    }

    /// Read-only function to check total tips collected so far (Free/Off-chain call)
    pub fn get_total_tips(env: Env) -> i128 {
        env.storage().instance().get(&StorageKey::TotalTips).unwrap_or(0)
    }
}