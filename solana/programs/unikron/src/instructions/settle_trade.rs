use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::constants::*;

#[derive(Accounts)]
pub struct SettleFee<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub source_fee_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub liquidity_staker_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub bounty_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<SettleFee>, fee_amount: u64) -> Result<()> {
    let treasury_fee = fee_amount * 30 / 100;
    let stakers_fee = fee_amount * 50 / 100;
    let bounty_fee = fee_amount - (treasury_fee + stakers_fee);

    for (amount, dest) in [
        (stakers_fee, &ctx.accounts.liquidity_staker_account),
        (treasury_fee, &ctx.accounts.treasury_account),
        (bounty_fee, &ctx.accounts.bounty_account),
    ] {
        let cpi_accounts = Transfer {
            from: ctx.accounts.source_fee_account.to_account_info(),
            to: dest.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
    }
    Ok(())
}