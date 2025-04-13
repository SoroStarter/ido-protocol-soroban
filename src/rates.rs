use soroban_sdk::{Address, Env};

use crate::storage_types::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD};

pub fn read_sale_rate(e: &Env, payment_token: Address) -> u64 {
    let key = DataKey::SalesRate(payment_token);
    // e.storage().instance().get(&key).unwrap()
    if let Some(rate) = e.storage().persistent().get::<DataKey, u64>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        rate
    } else {
        0
    }
}

pub fn write_sales_rate(e: &Env, payment_token: Address, rate: u64) {
    let key = DataKey::SalesRate(payment_token);
    e.storage().persistent().set(&key, &rate);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}
