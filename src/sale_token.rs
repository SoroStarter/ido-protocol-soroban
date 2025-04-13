use soroban_sdk::{token, Address, Env};

use crate::storage_types::DataKey;

// pub fn sales_token_has_been_set(e: &Env) -> bool {
//     let key = DataKey::Token;
//     e.storage().instance().has(&key)
// }

pub fn read_token(e: &Env) -> Address {
    let key = DataKey::Token;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_token(e: &Env, token_address: &Address) {
    let key = DataKey::Token;
    e.storage().instance().set(&key, token_address);
}

pub fn take_token(env: &Env, token_address: &Address, from: &Address, transfer_amount: i128) {
    let token = token::Client::new(env, token_address);
    let contract_address = env.current_contract_address();

    token.transfer(from, &contract_address, &transfer_amount);
}

pub fn send_token(env: &Env, token_address: &Address, to: &Address, transfer_amount: i128) {
    let token = token::Client::new(env, token_address);
    let contract_address = env.current_contract_address();

    token.transfer(&contract_address, to, &transfer_amount);
}
