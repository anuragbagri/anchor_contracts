use anchor_lang::prelude::*;
use anchor_spl::token::{Mint , MintTo , Token};



declare_id!("")
#[program]
pub mod create_mint_anchor {
    use super::*;

    pub fn create_mint_metadata_mint_to() -> Result<()> {
 
    }


    #[derive(Accounts)]
    pub struct CreateMintAnchor<'info> {
        #[account(mut)]
        pub signer : Signer<'info>,

        // mint account initialized on-chain 
        #[account(
            init,
            payer= signer,
            space = 82, // spl token mint size 
            seeds = [b"mint"],
            bump,
        )]

        pub mint : Account<'info , Mint>
    }
}