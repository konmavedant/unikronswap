use anchor_lang::prelude::*;
use crate::{state::{SwapIntent, TradeIntentData}, errors::ErrorCode};

#[derive(Accounts)]
pub struct RevealTrade<'info> {
    #[account(mut, seeds = [b"intent", swap_intent.user.as_ref(), &swap_intent.nonce.to_le_bytes()], bump)]
    pub swap_intent: Account<'info, SwapIntent>,
}

pub fn handle(ctx: Context<RevealTrade>, intent: TradeIntentData, expected_hash: [u8; 32]) -> Result<()> {
    let intent_acc = &mut ctx.accounts.swap_intent;

    require!(!intent_acc.revealed, ErrorCode::AlreadyRevealed);
    require!(intent_acc.expiry > Clock::get()?.unix_timestamp as u64, ErrorCode::IntentExpired);
    require!(intent.nonce == intent_acc.nonce, ErrorCode::NonceMismatch);
    require!(intent.user == intent_acc.user, ErrorCode::InvalidSignature);

    let calculated_hash = intent.hash();
    require!(calculated_hash == expected_hash, ErrorCode::HashMismatch);
    require!(calculated_hash == intent_acc.intent_hash, ErrorCode::HashMismatch);

    intent_acc.revealed = true;
    Ok(())
}