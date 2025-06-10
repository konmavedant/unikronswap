pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::commit_trade::{CommitTrade, handle_commit};
use instructions::reveal_trade::{RevealTrade, handle_reveal};
use instructions::settle_trade::{SettleFee, handler as handle_settle};
use state::TradeIntentData;

declare_id!("7saCDPbxRTGEPeyTYZgXyzNVr5LXFPEnYKpVyAqT2QNd");

#[program]
pub mod unikron {
    use super::*;

    pub fn commit_trade(
        ctx: Context<CommitTrade>,
        intent_hash: [u8; 32],
        nonce: u64,
        expiry: u64,
    ) -> Result<()> {
        handle_commit(ctx, intent_hash, nonce, expiry)
    }

    pub fn reveal_trade(
        ctx: Context<RevealTrade>,
        intent: TradeIntentData,
        expected_hash: [u8; 32],
        signature: [u8; 64],
    ) -> Result<()> {
        handle_reveal(ctx, intent, expected_hash, signature)
    }

    pub fn settle_trade(ctx: Context<SettleFee>, fee_amount: u64) -> Result<()> {
        handle_settle(ctx, fee_amount)
    }
}
