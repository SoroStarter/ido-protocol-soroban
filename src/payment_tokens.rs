use soroban_sdk::{Address, Env, Vec};

use crate::rates::read_sale_rate;
use crate::storage_types::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD};

pub fn read_payment_count(e: &Env) -> u32 {
    let key = DataKey::PaymentTokenCount;
    if let Some(count) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        count
    } else {
        0
    }
}

pub fn read_payment_tokens(e: &Env) -> Vec<Address> {
    let mut payment_tokens: Vec<Address> = Vec::new(&e);
    let payment_count = read_payment_count(e);
    for index in 1..=payment_count {
        let key = DataKey::PaymentToken(index);
        let payment_token = e.storage().instance().get(&key).unwrap();
        payment_tokens.push_back(payment_token);
    }
    payment_tokens
}

pub fn read_is_supported_payment_token(e: &Env, payment_token: Address) -> bool {
    let key = DataKey::IsSupportedPayment(payment_token);
    e.storage().instance().get(&key).unwrap()
}

pub fn write_payment_token(e: &Env, token_address: Address) {
    let index = read_payment_count(e) + 1;
    let key_token = DataKey::PaymentToken(index);
    let key_support = DataKey::IsSupportedPayment(token_address.clone());
    e.storage().instance().set(&key_support, &true);
    e.storage().instance().set(&key_token, &token_address);
}

pub fn read_active_payment_tokens(e: &Env) -> Vec<Address> {
    let mut payment_tokens: Vec<Address> = Vec::new(&e);
    let payment_count = read_payment_count(e);
    for index in 1..=payment_count {
        let key = DataKey::PaymentToken(index);
        let payment_token: Address = e.storage().instance().get(&key).unwrap();
        let payment_rate = read_sale_rate(e, payment_token.clone());
        if payment_rate > 0 {
            payment_tokens.push_back(payment_token);
        }
    }
    payment_tokens
}
