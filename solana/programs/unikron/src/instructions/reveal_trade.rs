use anchor_lang::prelude::*;
use crate::{state::*, errors::ErrorCode};
use anchor_lang::solana_program::{
    ed25519_program,
    sysvar::{instructions::load_instruction_at_checked, Sysvar},
};

#[derive(Accounts)]
pub struct RevealTrade<'info> {
    #[account(mut, has_one = user, seeds = [b"intent", user.key().as_ref(), &swap_intent.nonce.to_le_bytes()], bump)]
    pub swap_intent: Account<'info, SwapIntent>,

    pub user: Signer<'info>,

    /// CHECK: We're verifying instruction manually
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions_sysvar: AccountInfo<'info>,
}

pub fn handle_reveal(
    ctx: Context<RevealTrade>,
    intent: TradeIntentData,
    expected_hash: [u8; 32],
    _signature: [u8; 64],
) -> Result<()> {
    let stored = &mut ctx.accounts.swap_intent;
    require!(!stored.revealed, ErrorCode::AlreadyRevealed);
    require!(stored.expiry > Clock::get()?.unix_timestamp as u64, ErrorCode::IntentExpired);
    require!(stored.nonce == intent.nonce, ErrorCode::NonceMismatch);

    // Reconstruct hash of TradeIntentData
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(intent.try_to_vec()?);
    let hash_result = hasher.finalize();
    require!(hash_result[..] == expected_hash[..], ErrorCode::HashMismatch);

    // ed25519 syscall simulation: check if 0th instruction is from ed25519_program
    let ix = load_instruction_at_checked(0, &ctx.accounts.instructions_sysvar)?;
    require_keys_eq!(ix.program_id, ed25519_program::id(), ErrorCode::InvalidSignature);

    // Assume valid signature for PoC
    stored.revealed = true;
    Ok(())
}