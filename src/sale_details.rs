use crate::storage_types::{DataKey, SalesParameter};
use soroban_sdk::{Address, Env};

pub fn read_sales_parameters(e: &Env) -> SalesParameter {
    let key = DataKey::SaleParametersKey;

    if let Some(parameters) = e.storage().instance().get::<_, SalesParameter>(&key) {
        if parameters.end_time < e.ledger().timestamp() {
            panic!("no sales parameters as no ongoing sale")
        } else {
            parameters
        }
    } else {
        SalesParameter {
            start_time: 0,
            end_time: 0,
            soft_cap: 0,
            hard_cap: 0,
            min_buy: 0,
            max_buy: 0,
            tge_time: 0,
        }
    }
}

pub fn write_sales_parameters(
    e: &Env,
    start_time: u64,
    end_time: u64,
    soft_cap: u64,
    hard_cap: u64,
    min_buy: u64,
    max_buy: u64,
    tge_time: u64,
) {
    if end_time <= e.ledger().timestamp()
        || end_time < start_time
        || soft_cap == 0
        || hard_cap < soft_cap
        || max_buy < min_buy
        || tge_time < end_time
    {
        panic!("invalid parameter(s) entered!")
    }

    let parameters = SalesParameter {
        start_time,
        end_time,
        soft_cap,
        hard_cap,
        min_buy,
        max_buy,
        tge_time,
    };

    let key = DataKey::SaleParametersKey;
    e.storage().instance().set(&key.clone(), &parameters);
}

pub fn write_fund_recipient(e: &Env, recipient: Address) {
    let key = DataKey::FundsRecipient;
    e.storage().instance().set(&key, &recipient);
}

pub fn read_fund_recipient(e: &Env) -> Address {
    let key = DataKey::FundsRecipient;
    e.storage().instance().get(&key).unwrap()
}
