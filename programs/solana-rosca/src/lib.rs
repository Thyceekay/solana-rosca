
use anchor_lang::prelude::*;

declare_id!("G9NbuKyKKfyyAYm22rWTcG8SZ7FVM5sdJ19Leymh3nqf"); 

#[program]
pub mod solana_rosca {
    use super::*;

    pub fn create_group(ctx: Context<CreateGroup>, contribution_amount: u64, max_participants: u8) -> Result<()> {
        let group = &mut ctx.accounts.group;
        group.admin = *ctx.accounts.admin.key;
        group.contribution_amount = contribution_amount;
        group.max_participants = max_participants;
        group.participants = Vec::new();
        group.current_week = 0;
        group.pot_received = vec![false; max_participants as usize];
        Ok(())
    }

/* 
    pub fn join_group(ctx: Context<JoinGroup>) -> Result<()> {
    msg!("Group account: {:?}", ctx.accounts.group);
    let group = &mut ctx.accounts.group;
    let participant = &mut ctx.accounts.participant;

    if group.participants.len() >= group.max_participants as usize {
        return Err(ErrorCode::GroupFull.into());
    }

    group.participants.push(*ctx.accounts.group.key); // Error here
    participant.group = *ctx.accounts.group.key; // Error here
    participant.user = *ctx.accounts.user.key;
    participant.contributions = vec![0; group.max_participants as usize];
    Ok(())
}
    */
    pub fn join_group(ctx: Context<JoinGroup>) -> Result<()> {
        let group_key = ctx.accounts.group.key();
        let group = &mut ctx.accounts.group;
        let participant = &mut ctx.accounts.participant;

        if group.participants.len() >= group.max_participants as usize {
            return Err(ErrorCode::GroupFull.into());
        }

        group.participants.push(ctx.accounts.user.key());
        participant.group = group_key;
        participant.user = *ctx.accounts.user.key;
        participant.contributions = vec![0; group.max_participants as usize];
        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let group = &ctx.accounts.group;
        let participant = &mut ctx.accounts.participant;

        if amount != group.contribution_amount {
            return Err(ErrorCode::IncorrectAmount.into());
        }
        if participant.user != *ctx.accounts.user.key {
            return Err(ErrorCode::Unauthorized.into());
        }

        participant.contributions[group.current_week as usize] += amount;
        // Token transfer logic omitted to simplify my code 
        Ok(())
    }

    pub fn distribute_pot(ctx: Context<DistributePot>) -> Result<()> {
        let group = &mut ctx.accounts.group;

        if ctx.accounts.admin.key != &group.admin {
            return Err(ErrorCode::Unauthorized.into());
        }

        let eligible: Vec<usize> = group.pot_received
            .iter()
            .enumerate()
            .filter(|(_, &received)| !received)
            .map(|(index, _)| index)
            .collect();

        if eligible.is_empty() {
            return Err(ErrorCode::CycleComplete.into());
        }

        // For simplicity to pick the first eligible participant
        let winner_index = eligible[0];
        group.pot_received[winner_index] = true;
        group.current_week += 1;
        // Pot transfer logic omitted
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGroup<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1 + 32 * 10 + 1 + 1 * 10)]
    pub group: Account<'info, Group>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinGroup<'info> {
    #[account(mut)]
    pub group: Account<'info, Group>,
    #[account(init, payer = user, space = 8 + 32 + 32 + 8 * 10)]
    pub participant: Account<'info, Participant>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub group: Account<'info, Group>,
    #[account(mut)]
    pub participant: Account<'info, Participant>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DistributePot<'info> {
    #[account(mut)]
    pub group: Account<'info, Group>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Group {
    pub admin: Pubkey,
    pub contribution_amount: u64,
    pub max_participants: u8,
    pub participants: Vec<Pubkey>,
    pub current_week: u8,
    pub pot_received: Vec<bool>,
}

#[account]
pub struct Participant {
    pub group: Pubkey,
    pub user: Pubkey,
    pub contributions: Vec<u64>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Group is already full")]
    GroupFull,
    #[msg("Incorrect contribution amount")]
    IncorrectAmount,
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("ROSCA cycle is complete")]
    CycleComplete,
}
