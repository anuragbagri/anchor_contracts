use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::state::Pool;

pub mod add_liquididty {
    use super::*;
    pub fn function() -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_provider_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_provider_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    #[account(mut)]
    pub provider_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
