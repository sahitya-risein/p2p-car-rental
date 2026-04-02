#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub struct Listing {
    pub seller: Address,
    pub token_id: u128,
    pub price: i128,
}

#[contracttype]
pub enum DataKey {
    Listing(u128),
    Count,
}

#[contract]
pub struct SimpleMarketplace;

#[contractimpl]
impl SimpleMarketplace {

    // 🪙 List NFT
    pub fn list(env: Env, seller: Address, token_id: u128, price: i128) -> u128 {
        seller.require_auth();

        let mut count: u128 = env.storage().instance().get(&DataKey::Count).unwrap_or(0);
        count += 1;

        let listing = Listing {
            seller,
            token_id,
            price,
        };

        env.storage().instance().set(&DataKey::Listing(count), &listing);
        env.storage().instance().set(&DataKey::Count, &count);

        count
    }

    // 💰 Buy NFT
    pub fn buy(env: Env, buyer: Address, listing_id: u128) {
        buyer.require_auth();

        let listing: Listing = env
            .storage()
            .instance()
            .get(&DataKey::Listing(listing_id))
            .expect("Not found");

        // 💸 Send payment (native token logic simplified)
        // In real case: call token contract

        // 🔁 Transfer NFT (simplified)
        // In real case: call NFT contract

        // ❌ Remove listing
        env.storage().instance().remove(&DataKey::Listing(listing_id));
    }

    // 📦 View listing
    pub fn get(env: Env, listing_id: u128) -> Listing {
        env.storage()
            .instance()
            .get(&DataKey::Listing(listing_id))
            .expect("Not found")
    }
}