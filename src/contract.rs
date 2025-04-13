use crate::access::{has_administrator, read_administrator, write_administrator};
use crate::balances::{
    read_participant_contribution_amount, read_participant_purchase_amount,
    read_total_contribution, read_total_sold, update_make_contribution_amount,
    update_participant_purchase_amount, write_total_contribution, write_total_sold,
};
use crate::payment_tokens::{
    read_active_payment_tokens, read_is_supported_payment_token, read_payment_tokens,
    write_payment_token,
};
use crate::rates::{read_sale_rate, write_sales_rate};
use crate::sale_details::{
    read_fund_recipient, read_sales_parameters, write_fund_recipient, write_sales_parameters,
};
use crate::sale_token::{read_token, send_token, take_token, write_token};
use crate::storage_types::SalesParameter;

use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

pub trait SaleTrait {
    fn initialize(e: Env, admin: Address);
    fn set_sale_token(e: Env, token_address: Address);
    fn set_payment_token(e: Env, payment_token: Address);
    fn set_sale_parameters(
        e: &Env,
        start_time: u64,
        end_time: u64,
        soft_cap: u64,
        hard_cap: u64,
        min_buy: u64,
        max_buy: u64,
        tge_time: u64,
    );

    fn set_swap_rate(e: Env, payment_token: Address, rate: u64);
    fn set_fund_recipient(e: Env, recipient: Address);

    fn contribute(e: Env, participant: Address, payment_token: Address, amount: i128);
    fn claim_purchased_tokens(e: Env, participant: Address);
    fn claim_refund(e: Env, participant: Address);
    fn withdraw_raised_funds(e: Env);

    fn get_sale_token(e: Env) -> Address;
    fn get_payment_options(e: Env) -> Vec<Address>;
    fn get_sale_rate(e: Env, payment_token: Address) -> u64;
    fn get_payment_purchases(e: Env,participant: Address, payment_token: Address) -> i128 ;
    fn get_sales_parameters(e: &Env) -> SalesParameter;
    fn get_participant_total_purchase(e: Env, participant: Address) -> i128;
    fn get_participant_contribution(e: Env, participant: Address, payment_token: Address) -> i128;

    fn get_total_sold(e: Env) -> i128;
    fn get_total_contribution(e: Env, payment_token: Address) -> i128;
    fn get_fund_recipient(e: Env) -> Address;
    fn get_admin(e: &Env) -> Address;
    fn get_supported_tokens(e: Env) -> Vec<Address>;
    fn get_current_timestamp(e: Env) -> u64;
}

#[contract]
pub struct TokenSale;

#[contractimpl]
impl SaleTrait for TokenSale {
    fn initialize(e: Env, admin: Address) {
        if has_administrator(&e) {
            panic!("already has an admin")
        }
        write_administrator(&e, &admin);
    }

    fn set_payment_token(e: Env, payment_token: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();
        write_payment_token(&e, payment_token);
    }

    fn set_sale_token(e: Env, token_address: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();
        write_token(&e, &token_address)
    }

    fn set_sale_parameters(
        e: &Env,
        start_time: u64,
        end_time: u64,
        soft_cap: u64,
        hard_cap: u64,
        min_buy: u64,
        max_buy: u64,
        tge_time: u64,
    ) {
        let admin = read_administrator(&e);
        admin.require_auth();

        write_sales_parameters(
            e, start_time, end_time, soft_cap, hard_cap, min_buy, max_buy, tge_time,
        );

        let token_address = &read_token(&e);
        take_token(e, token_address, &admin, hard_cap as i128);
    }

    fn set_swap_rate(e: Env, payment_token: Address, rate: u64) {
        let admin = read_administrator(&e);
        admin.require_auth();

        let is_supported = read_is_supported_payment_token(&e, payment_token.clone());

        if !is_supported {
            panic!("the token entered is not a supported payment option")
        }

        write_sales_rate(&e, payment_token, rate);
    }

    fn set_fund_recipient(e: Env, recipient: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();
        write_fund_recipient(&e, recipient);
    }

    fn contribute(e: Env, participant: Address, payment_token: Address, amount: i128) {
        participant.require_auth();
        let is_supported = read_is_supported_payment_token(&e, payment_token.clone());
        let token_params = read_sales_parameters(&e);
        let payment_rate = read_sale_rate(&e, payment_token.clone());
        let amount_purchased = amount * payment_rate as i128;

        if !is_supported {
            panic!("the token entered is not a supported payment option")
        }
        if token_params.end_time < e.ledger().timestamp() {
            panic!("this sale is over")
        }
        if token_params.min_buy as u128 > amount_purchased as u128 {
            panic!("the amount entered is less than min buy")
        }

        if (token_params.max_buy as u128) < amount_purchased as u128 {
            panic!("the amount entered is greater than max buy")
        }

        take_token(&e, &payment_token, &participant, amount as i128);

        update_make_contribution_amount(&e, participant.clone(), payment_token, amount as i128);
        update_participant_purchase_amount(&e, participant, amount_purchased as i128);
        write_total_sold(&e, amount_purchased as i128);
    }

    //Allow participants to claim tokens from successful sale after tge time

    fn claim_purchased_tokens(e: Env, participant: Address) {
        participant.require_auth();
        let total_raised = read_total_sold(&e);
        let token_params = read_sales_parameters(&e);
        if total_raised < token_params.soft_cap as i128 {
            panic!("sales not successful, you can withdraw your contribution")
        }
        if token_params.tge_time > e.ledger().timestamp() {
            panic!("you cannot claim before the TGE time")
        }
        let amount_claimable = read_participant_purchase_amount(&e, participant.clone());
        if amount_claimable == 0 {
            panic!("this address has nothing to claim")
        }
        let token_address = read_token(&e);
        update_participant_purchase_amount(&e, participant.clone(), 0);
        send_token(&e, &token_address, &participant, amount_claimable);
    }

    //Refund contribution if sale not successful

    fn claim_refund(e: Env, participant: Address) {
        participant.require_auth();

        let total_raised = read_total_sold(&e);
        let token_params = read_sales_parameters(&e);
        if total_raised > token_params.soft_cap as i128 {
            panic!("sale was successful, claim tokens purchased instead")
        }
        if token_params.end_time > e.ledger().timestamp() {
            panic!("you cannot claim refund before sale is over")
        }
        let active_payment_tokens = read_active_payment_tokens(&e);

        for payment_token in active_payment_tokens.iter() {
            let payment_amount_claimable = read_participant_contribution_amount(
                &e,
                participant.clone(),
                payment_token.clone(),
            );
            if payment_amount_claimable > 0 {
                update_make_contribution_amount(&e, participant.clone(), payment_token.clone(), 0);
                send_token(&e, &payment_token, &participant, payment_amount_claimable);
            }
        }
    }

    fn withdraw_raised_funds(e: Env) {
        let fund_recipient = read_fund_recipient(&e);
        let sale_parameter = read_sales_parameters(&e);
        let total_sold = read_total_sold(&e);

        fund_recipient.require_auth();
        if sale_parameter.end_time > e.ledger().timestamp() {
            panic!("the sale is not over, fund can be withdrawn only when sale is over!")
        }

        if sale_parameter.soft_cap > total_sold as u64 {
            panic!("the softcap not reached, the sale was not successful!")
        }

        let active_payment_tokens = read_active_payment_tokens(&e);

        for payment_token in active_payment_tokens.iter() {
            let withdrawable_funds = read_total_contribution(&e, payment_token.clone());
            if withdrawable_funds > 0 {
                write_total_contribution(&e, payment_token.clone(), 0);
                send_token(&e, &payment_token, &fund_recipient, withdrawable_funds);
            }
        }
    }

    fn get_sale_token(e: Env) -> Address {
        read_token(&e)
    }

    fn get_sale_rate(e: Env, payment_token: Address) -> u64 {
        read_sale_rate(&e, payment_token)
    }

    fn get_payment_purchases(e: Env,participant: Address, payment_token: Address) -> i128 {
        let rate=read_sale_rate(&e, payment_token.clone());
        let total_amount=read_participant_contribution_amount(&e, participant, payment_token);
        rate as i128*total_amount
    }

    fn get_payment_options(e: Env) -> Vec<Address> {
        read_active_payment_tokens(&e)
    }

    fn get_sales_parameters(e: &Env) -> SalesParameter {
        read_sales_parameters(e)
    }

    fn get_participant_total_purchase(e: Env, participant: Address) -> i128 {
        read_participant_purchase_amount(&e, participant)
    }

    fn get_participant_contribution(e: Env, participant: Address, payment_token: Address) -> i128 {
        read_participant_contribution_amount(&e, participant, payment_token)
    }

    fn get_total_sold(e: Env) -> i128 {
        read_total_sold(&e)
    }

    fn get_total_contribution(e: Env, payment_token: Address) -> i128 {
        read_total_contribution(&e, payment_token)
    }

    fn get_supported_tokens(e: Env) -> Vec<Address> {
        read_payment_tokens(&e)
    }

    fn get_fund_recipient(e: Env) -> Address {
        read_fund_recipient(&e)
    }

    fn get_admin(e: &Env) -> Address {
        read_administrator(e)
    }

    fn get_current_timestamp(e: Env) -> u64 {
        e.ledger().timestamp()
    }
}
