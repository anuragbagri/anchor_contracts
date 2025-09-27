use anchor_lang::prelude::*;
use anchor_spl::token::{Mint , MintTo , Token};
use anchor_lang::accounts::program;
use anchor_spl::{associated_token::AssociatedToken, token::{self, InitializeMint}, token_interface::spl_token_metadata_interface::instruction::Initialize};



declare_id!("")
#[program]
pub mod create_mint_anchor {
        use super::*;

    pub fn create_mint_metadata_and_mint_to(
        ctx : Context<CreateMintAnchor>,
        decimals : u8,
        amount : u64,
        name : String,
        symbol : String,
        uri : String
    ) -> Result<()> {
     let cpi_account = InitializeMint {
        mint : ctx.accounts.mint.to_account_info(),
        rent : ctx.accounts.rent.to_account_info(),
     };
     
     
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

        pub mint : Account<'info , Mint>,


        // pda mint authority 
        #[account(
            seeds = [b"mint_authority"],
            bump
        )]

        pub mint_authority : UncheckedAccount<'info>,

        // metadata account, cpi will create it 
        #[account(mut)]
        pub metadata : UncheckedAccount<'info>,

        pub token_metadata_program : UncheckedAccount<'info>,
        

        // user wallet 
        #[account(mut)]
        pub user : Signer<'info>,

        // user ata
        pub user_ata : UncheckedAccount<'info>,

        pub token_program : Program<'info, Token>,
        pub associated_token_program  : Program<'info , AssociatedToken>,
        pub system_program : Program<'info , System>,
        pub rent : Sysvar<'info , Rent>,
    }
}