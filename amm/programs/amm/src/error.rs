use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Insufficeint input amount")]
    InsufficientInputAmount,
    #[msg("Insufficient liquidity minted")]
    InsufficientLiquidityMinted,
    #[msg("Insufficient Liquidity Burned")]
    InsufficientLiquidityBurned,
    #[msg("INsufficient output amount")]
    InsufficientOutputAmount,
    #[msg(" overflow accoured")]
    Overflow,
    #[msg("Invalid pool accounts or mismatch")]
    InavlidPoolAccounts,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid instruction data")]
    InvalidInstruction,
}
