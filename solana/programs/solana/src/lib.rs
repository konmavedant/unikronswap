use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

// UNIKRON Program ID
// Replace with actual ID after deployment

declare_id!("Ez6G6H5KVD8AqUPNiVcqE8Z4yiG5Dr9oF8j4qyM93UU6");

#[program]
pub mod unikron {
    use super::*;

    /// Commit Phase: User commits their intent hash to protect against MEV
    pub fn commit_swap(ctx: Context<CommitSwap>, intent_hash: [u8; 32]) -> Result<()> {
        let swap = &mut ctx.accounts.swap;
        swap.user = *ctx.accounts.user.key;
        swap.intent_hash = intent_hash;
        swap.timestamp = Clock::get()?.unix_timestamp;
        swap.revealed = false;
        Ok(())
    }

    /// Reveal Phase: Off-chain relayer reveals and executes the signed intent
    pub fn reveal_and_execute(
        ctx: Context<RevealSwap>,
        actual_data: Vec<u8>,
        route_hash: [u8; 32],
        amount_in: u64,
    ) -> Result<()> {
        let swap = &mut ctx.accounts.swap;

        require!(!swap.revealed, UnikronError::AlreadyRevealed);

        // Recompute hash of revealed data
        let actual_hash = keccak::hash(&actual_data).0;
        require!(actual_hash == swap.intent_hash, UnikronError::HashMismatch);

        swap.revealed = true;

        // Compute fee (0.10%) and split it
        let total_fee = amount_in / 1000; // 0.1%
        let fee_50 = total_fee * 50 / 100;
        let fee_30 = total_fee * 30 / 100;
        let fee_20 = total_fee - fee_50 - fee_30;

        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? -= total_fee;
        **ctx.accounts.stakers.to_account_info().try_borrow_mut_lamports()? += fee_50;
        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += fee_30;
        **ctx.accounts.bounty.to_account_info().try_borrow_mut_lamports()? += fee_20;

        Ok(())
    }
}

/// Accounts context for committing an intent
#[derive(Accounts)]
#[instruction(intent_hash: [u8; 32])]
pub struct CommitSwap<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"swap", user.key().as_ref(), &intent_hash],
        bump
    )]
    pub swap: Account<'info, SwapIntent>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts context for revealing and executing a swap intent
#[derive(Accounts)]
pub struct RevealSwap<'info> {
    #[account(
        mut,
        seeds = [b"swap", swap.user.as_ref(), &swap.intent_hash],
        bump
    )]
    pub swap: Account<'info, SwapIntent>,

    #[account(mut, address = swap.user)]
    pub user: Signer<'info>,

    /// CHECK: Verified by routing logic or governance rules
    #[account(mut)]
    pub stakers: AccountInfo<'info>,
    /// CHECK: Verified by routing logic or governance rules
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    /// CHECK: Verified by routing logic or governance rules
    #[account(mut)]
    pub bounty: AccountInfo<'info>,
}

/// SwapIntent holds a hashed pre-signed intent
#[account]
pub struct SwapIntent {
    pub user: Pubkey,
    pub intent_hash: [u8; 32],
    pub timestamp: i64,
    pub revealed: bool,
}

/// Custom error codes for clarity
#[error_code]
pub enum UnikronError {
    #[msg("Intent hash mismatch with revealed data")]
    HashMismatch,
    #[msg("Swap already revealed")]
    AlreadyRevealed,
}