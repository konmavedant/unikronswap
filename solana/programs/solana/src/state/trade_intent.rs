use anchor_lang::prelude::*;
use sha2::{Sha256, Digest};

#[account]
pub struct SwapIntent {
    pub user: Pubkey,
    pub intent_hash: [u8; 32],
    pub nonce: u64,
    pub expiry: u64,
    pub timestamp: i64,
    pub revealed: bool,
}

impl SwapIntent {
    pub const SIZE: usize = 32 + 32 + 8 + 8 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TradeIntentData {
    pub user: Pubkey,
    pub nonce: u64,
    pub expiry: u64,
    pub relayer: Pubkey,
    pub relayer_fee: u64,
    pub token_in: Pubkey,
    pub token_out: Pubkey,
    pub amount_in: u64,
    pub min_out: u64,
}

impl TradeIntentData {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.serialize(&mut buf).unwrap();
        buf
    }

    pub fn hash(&self) -> [u8; 32] {
        let bytes = self.to_bytes();
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}