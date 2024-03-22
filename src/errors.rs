use anchor_lang::prelude::error_code;

#[error_code]
pub enum ProjectError {
    #[msg("Funds Already Been Claimed")]
    FundsAlreadyClaimed,
    #[msg("Cannot Claim As You Are Not The Owner")]
    UnauthorizedToClaim,
    #[msg("Funds Raised Less Than Funding Goal")]
    FundingGoalNotReached,
}