use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::error::AmmError;
use crate::state::state::Pool;

pub mod initialize_pool {

    use super::*;
    pub fn function(
        ctx: Context<Initialize>,
        fee_numerator: u64,
        fee_denominator: u64,
    ) -> Result<()> {
        if ctx.accounts.vault_a.mint != ctx.accounts.mint_a.key() {
            return Err(AmmError::InavlidPoolAccounts.into());
        }
        if ctx.accounts.vault_b.mint != ctx.accounts.mint_b.key() {
            return Err(AmmError::InavlidPoolAccounts.into());
        }

        let pool = &mut ctx.accounts.pool;
        pool.admin = ctx.accounts.payer.key();
        pool.mint_a = ctx.accounts.mint_a.key();
        pool.mint_b = ctx.accounts.mint_b.key();
        pool.vault_a = ctx.accounts.vault_a.key();
        pool.vault_b = ctx.accounts.vault_b.key();
        pool.lp_mint = ctx.accounts.lp_mint.key();
        pool.fee_numerator = fee_numerator;
        pool.fee_denominator = fee_denominator;
        pool.bump = 0u8;
        Ok(())
    }
}

#[derive[Accounts]]
#[instruction(fee_numerator : u64 , fee_denominator : u64 )]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Pool::LEN
    )]
    pub pool: Account<'info, Pool>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    // vault account(hold reserve)... needs to be created
    #[account(mut)]
    pub vault_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_b: Account<'info, TokenAccount>,

    // lp mint for LP TOKENs (cleint side)
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
