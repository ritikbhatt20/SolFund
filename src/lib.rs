use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, hash::hash, program::invoke, system_instruction::transfer},
};

mod constants;

use crate::{constants::*};

declare_id!("2NNwECQ3bRpSUM9BSojyuUzsfZ2yddNQ3mGq7Nd2Z8Aa");

#[program]
mod crowdfunding {
    use super::*;

    pub fn initialize_project(ctx: Context<InitializeProject>, funding_goal: u64, deadline: i64,) -> Result<()> {

        let project = &mut ctx.accounts.project;

        project.owner = ctx.accounts.owner.key();
        project.funding_goal = funding_goal;
        project.deadline = deadline;

        msg!("Owner: {}", project.owner);
        msg!("Funding Goal: {}", project.funding_goal);
        msg!("Project Deadline: {}", project.deadline);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProject<'info> {
    #[account(
        init,
        payer = owner,
        space = 1000,
        seeds = [PROJECT_SEED.as_bytes()],
        bump,
    )]
    pub project: Account<'info, Project>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Project {
    pub owner: Pubkey,
    pub funding_goal: u64,
    pub deadline: i64, // Unix timestamp
    pub total_funded: u64,
}


