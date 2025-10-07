use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub admin : Pubkey,
    pub mint_a : Pubkey,
    pub mint_b : Pubkey,
    pub vault_a : Pubkey,
    pub vault_b : Pubkey,
    pub lp_mint : Pubkey,   // lp mint liquidity provider tokens
    pub fee_numerator  : u64,
    pub fee_denominator : u64,

    pub bump : u8,

    pub padding : [u8 ;32],    // reserved 
}

impl Pool {
    pub const LEN : usize = 8 + //discrimainator
    32 + 
    32 +
    32 +
    32 + 
    32 +
    32 +
    8 + 
    8 +
    1 +
    32; //padding
}
