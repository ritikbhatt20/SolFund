use anchor_lang::prelude::error_code;

#[error_code]
pub enum ProjectError {
    #[msg("Funds Already Been Claimed")]
    FundsAlreadyClaimed,
}