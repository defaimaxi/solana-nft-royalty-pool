use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge,
    #[msg("MyAccount may only hold data below 100")]
    RoyaltyDistributeFailed,
    #[msg("Could not add shares into user's account")]
    AddShares,
}
