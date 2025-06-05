use anchor_lang::prelude::*;
use crate::state::SwapIntent;

#[derive(Accounts)]
#[instruction(nonce: u64)] // Ensures `nonce` is available for the seeds expression
pub struct CommitTrade<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + SwapIntent::SIZE,
        seeds = [b"intent", user.key().as_ref(), &nonce.to_le_bytes()],
        bump
    )]
    pub swap_intent: Account<'info, SwapIntent>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<CommitTrade>, intent_hash: [u8; 32], nonce: u64, expiry: u64) -> Result<()> {
    let intent = &mut ctx.accounts.swap_intent;
    intent.user = *ctx.accounts.user.key;
    intent.intent_hash = intent_hash;
    intent.nonce = nonce;
    intent.expiry = expiry;
    intent.timestamp = Clock::get()?.unix_timestamp;
    intent.revealed = false;
    Ok(())
}
