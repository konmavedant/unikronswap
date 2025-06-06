use anchor_lang::prelude::*;
use crate::state::{FeePools, Config};

#[derive(Accounts)]
pub struct SettleTrade<'info> {
    #[account(mut)]
    pub fee_pools: Account<'info, FeePools>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

pub fn handle(_ctx: Context<SettleTrade>) -> Result<()> {
    let total_fee = 1000; // simulated amount
    let treasury_fee = total_fee * 50 / 100;
    let stakers_fee = total_fee * 30 / 100;
    let bounty_fee = total_fee * 20 / 100;

    msg!("Fees distributed: Treasury={}, Stakers={}, Bounty={}", treasury_fee, stakers_fee, bounty_fee);
    Ok(())
}