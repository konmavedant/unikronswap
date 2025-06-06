// lib.rs
use anchor_lang::prelude::*;
declare_id!("7saCDPbxRTGEPeyTYZgXyzNVr5LXFPEnYKpVyAqT2QNd");

pub mod instructions;
pub mod state;
pub mod constants;
pub mod errors;

use instructions::*;
use instructions::commit_trade::handle as handle_commit;
use instructions::reveal_trade::handle as handle_reveal;
use instructions::settle_trade::handle as handle_settle;
use state::TradeIntentData;

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
    ) -> Result<()> {
        handle_reveal(ctx, intent, expected_hash)
    }

    pub fn settle_trade(ctx: Context<SettleTrade>) -> Result<()> {
        handle_settle(ctx)
    }
}