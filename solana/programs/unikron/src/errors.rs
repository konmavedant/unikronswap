use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Intent already revealed")]
    AlreadyRevealed,

    #[msg("Trade intent expired")]
    IntentExpired,

    #[msg("Nonce does not match")]
    NonceMismatch,

    #[msg("Signature verification failed")]
    InvalidSignature,

    #[msg("Hash mismatch between reveal and commit")]
    HashMismatch,
}