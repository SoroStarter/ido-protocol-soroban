use soroban_sdk::{contracttype, Address};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct SalesParameter {
    pub start_time: u64, // Duration of the sale
    pub end_time: u64,
    pub soft_cap: u64, //sales softcap
    pub hard_cap: u64,
    pub min_buy: u64,
    pub max_buy: u64,
    pub tge_time: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,                                     //Admin of the contract
    Token,                                     // Token to be sold
    SalesRate(Address), //The rate (swap ratio) of the token sale with respect to the purchase token
    PaymentToken(u32),  // Supported Payment tokens
    IsSupportedPayment(Address), //Supported payment token validation
    PaymentTokenCount,  //Number of supported payment tokens
    ParticipantContribution(Address, Address), //Amount spent by each participants (the keys are the participants address and the payment token address)
    TotalContribution(Address),                //Total funds raised (the key is token address)
    ParticipantsCount,                         //the total number of unique participants
    SaleParametersKey,                         // stores all the sales parameters
    FundsRecipient,                            //Wallet that received or claims the funds
    AmountPurchased(Address), //the amount of tokens purchased by a participants (amount contributed*rate)
    Trefund(Address), //time until participants can withdraw contribution and opt out of the sale
    //sales hardcap
    TotalTokensSold, //Amount of tokens already sold
    TokensRemaining, // Amount of tokens left
}
