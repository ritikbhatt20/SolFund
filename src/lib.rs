use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, hash::hash, program::invoke, system_instruction::transfer},
};

mod constants;
mod errors;

use crate::{constants::*, errors::*};  

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

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        // Ensure the project is not claimed yet
        let project = &mut ctx.accounts.project;

        // Ensure the project is not claimed yet
        if project.claimed_fund {
            return err!(ProjectError::FundsAlreadyClaimed)
        }

        // Transfer SOL from contributor to project account
        invoke(
            &transfer(&ctx.accounts.contributor.key(), &project.key(), amount),
            &[
                ctx.accounts.contributor.to_account_info(),
                project.to_account_info(),
                ctx.accounts.system_program.to_account_info()
            ],
        )?;

        // Update total funded amount in project account
        project.total_funded += amount;

        Ok(())
    }

    pub fn claim_funds(ctx: Context<ClaimFunds>) -> Result<()> {

        let project = &mut ctx.accounts.project;

        let claimer = &ctx.accounts.owner;

        // Ensure the project owner is the signer
        if ctx.accounts.owner.key() != project.owner {
        return err!(ProjectError::UnauthorizedToClaim)
        }

        // Ensure the funding goal has been reached
        if project.total_funded < project.funding_goal {
            return err!(ProjectError::FundingGoalNotReached)
        }

        // Ensure the deadline has passed
        // let current_timestamp = Clock::get()?.unix_timestamp;
        // if current_timestamp < project.deadline {
        //     return Err(ErrorCode::DeadlineNotPassed.into());
        // }

        **project.to_account_info().try_borrow_mut_lamports()? -= project.total_funded;
        **claimer.to_account_info().try_borrow_mut_lamports()? += project.total_funded;

        project.claimed_fund = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProject<'info> {
    #[account(
        init,
        payer = owner,
        space = 32 + 8 + 8 + 8 + 1 + 8,
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
    pub claimed_fund: bool,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        mut,
        seeds = [PROJECT_SEED.as_bytes()],
        bump
    )]
    pub project: Account<'info, Project>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimFunds<'info> {
    #[account(
        mut,
        seeds = [PROJECT_SEED.as_bytes()],
        bump
    )]
    pub project: Account<'info, Project>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}