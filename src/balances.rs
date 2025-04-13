use crate::{
    sale_details::read_sales_parameters,
    storage_types::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD},
};
use soroban_sdk::{Address, Env};

pub fn read_participant_contribution_amount(
    e: &Env,
    participant: Address,
    payment_token: Address,
) -> i128 {
    let key = DataKey::ParticipantContribution(participant, payment_token);
    if let Some(amount) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        amount
    } else {
        0
    }
}

fn write_participant_contribution_amount(
    e: &Env,
    participant: Address,
    payment_token: Address,
    amount: i128,
) {
    let key = DataKey::ParticipantContribution(participant, payment_token);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn update_make_contribution_amount(
    e: &Env,
    participant: Address,
    payment_token: Address,
    contribution: i128,
) {
    let balance =
        read_participant_contribution_amount(e, participant.clone(), payment_token.clone());
    write_participant_contribution_amount(
        e,
        participant,
        payment_token.clone(),
        balance + contribution,
    );
    write_total_contribution(e, payment_token, contribution);
}

pub fn read_participant_purchase_amount(e: &Env, addr: Address) -> i128 {
    let key = DataKey::AmountPurchased(addr);
    if let Some(amount) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        amount
    } else {
        0
    }
}

fn write_participant_purchase_amount(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::AmountPurchased(addr);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn update_participant_purchase_amount(e: &Env, addr: Address, amount_purchased: i128) {
    let pre_purchase_amount = read_participant_purchase_amount(e, addr.clone());
    let max_buy = read_sales_parameters(e).max_buy as i128;
    let total_purchased = pre_purchase_amount + amount_purchased;
    if total_purchased > max_buy {
        panic!("this put the total amount purchased above the max buy limit")
    }
    write_participant_purchase_amount(e, addr, total_purchased);
    if pre_purchase_amount == 0 {
        write_participants_count(e, 1);
    }
}

pub fn read_total_contribution(e: &Env, token_address: Address) -> i128 {
    let key = DataKey::TotalContribution(token_address);
    if let Some(total_contribution) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        total_contribution
    } else {
        0
    }
}

pub fn write_total_contribution(e: &Env, token_address: Address, contribution: i128) {
    let key = DataKey::TotalContribution(token_address.clone());
    let cur_total_contribution = read_total_contribution(e, token_address);
    let new_total_contribution = contribution + cur_total_contribution;
    e.storage().persistent().set(&key, &new_total_contribution);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn read_total_sold(e: &Env) -> i128 {
    let key = DataKey::TotalTokensSold;
    if let Some(total_sold) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        total_sold
    } else {
        0
    }
}

pub fn write_total_sold(e: &Env, amount_sold: i128) {
    let key = DataKey::TotalTokensSold;
    let cur_total_sold = read_total_sold(e);
    let new_total_sold = amount_sold + cur_total_sold;
    e.storage().persistent().set(&key, &new_total_sold);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn read_participants_count(e: &Env) -> i128 {
    let key = DataKey::ParticipantsCount;
    if let Some(count) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        count
    } else {
        0
    }
}

fn write_participants_count(e: &Env, inc: i128) {
    let key = DataKey::ParticipantsCount;
    let cur_count = read_participants_count(e);
    let new_count = cur_count + inc;
    e.storage().persistent().set(&key, &new_count);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}
