// programs/solana/src/lib.rs

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod constants;
pub mod errors;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgmqz7uYb6qV");

use instructions::*;

#[program]
pub mod unikron {
    use super::*;

    pub fn commit_trade(
        ctx: Context<commit_trade::CommitTrade>,
        intent_hash: [u8; 32],
        nonce: u64,
        expiry: u64,
    ) -> Result<()> {
        commit_trade::handle(ctx, intent_hash, nonce, expiry)
    }

    pub fn reveal_trade(
        ctx: Context<reveal_trade::RevealTrade>,
        intent: state::TradeIntentData,
        expected_hash: [u8; 32],
    ) -> Result<()> {
        reveal_trade::handle(ctx, intent, expected_hash)
    }

    pub fn settle_trade(ctx: Context<settle_trade::SettleTrade>) -> Result<()> {
        settle_trade::handle(ctx)
    }
}
