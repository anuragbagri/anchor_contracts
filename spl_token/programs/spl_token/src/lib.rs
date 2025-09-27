use anchor_lang::prelude::*;
use anchor_spl::token::{Mint , MintTo , Token};
use anchor_lang::accounts::program;
use anchor_spl::{associated_token::AssociatedToken, token::{self, InitializeMint}};



declare_id!("")
#[program]
pub mod create_mint_anchor {
        use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3};
        use mpl_token_metadata::{instructions::CreateMetadataAccountV3, types::DataV2};

        use super::*;

    pub fn create_mint_metadata_and_mint_to(
        ctx : Context<CreateMintAnchor>,
        decimals : u8,
        amount : u64,
        name : String,
        symbol : String,
        uri : String
    ) -> Result<()> {
     let cpi_account = token::InitializeMint {
        mint : ctx.accounts.mint.to_account_info(),
        rent : ctx.accounts.rent.to_account_info(),
     };

     let cpi_token_program = ctx.accounts.token_program.to_account_info();


     let cpi_context = CpiContext::new(cpi_token_program, cpi_account);

     token::initialize_mint(cpi_context, decimals, &ctx.accounts.mint_authority.key(), Some(&ctx.accounts.mint_authority.key()))?;

     

     // meatadata account via cpi 
     let token_metadata_data = DataV2 {
        name : name,
        symbol : symbol,
        uri : uri,
        seller_fee_basis_points : 0 ,
        creators : None,
        collection : None,
        uses : None,
     };


    let metadata_cpi_context = CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(), CreateMetadataAccountsV3 {
        metadata : ctx.accounts.metadata.to_account_info(),
        mint : ctx.accounts.mint.to_account_info(),
        mint_authority : ctx.accounts.mint_authority.to_account_info(),
        payer : ctx.accounts.signer.to_account_info(),
        update_authority : Some(ctx.accounts.mint_authority.to_account_info()),
        system_program : ctx.accounts.system_program.to_account_info(),
        rent : ctx.accounts.rent.to_account_info()
    } , &[b"metadata_account"]);

    create_metadata_accounts_v3(metadata_cpi_context, token_metadata_data, false, true, None)?;
     // user ata 
     let ata_cpi_accounts = anchor_spl::associated_token::Create {
        payer : ctx.accounts.signer.to_account_info(),
        associated_token : ctx.accounts.user_ata.to_account_info(),
        authority : ctx.accounts.mint_authority.to_account_info(),
        mint : ctx.accounts.mint.to_account_info(),
        system_program : ctx.accounts.system_program.to_account_info(),
        token_program : ctx.accounts.token_program.to_account_info(),
        rent : ctx.accounts.rent.to_account_info(),
     };
    
    let ata_cpi_prorgam = ctx.accounts.associated_token_program.to_account_info();


    let ata_cpi_context = CpiContext::new(ata_cpi_prorgam, ata_cpi_accounts);

    anchor_spl::associated_token::create(ata_cpi_context)?;


    // mint tokens to user Ata
    let transfer_cpi_accounts = MintTo {
        mint : ctx.accounts.mint.to_account_info(),
        to : ctx.accounts.user_ata.to_account_info(),
        authority : ctx.accounts.mint_authority.to_account_info(),
    };

    let cpi_token_program = ctx.accounts.token_program.to_account_info();

    let cpi_transfer_context = CpiContext::new(cpi_token_program, transfer_cpi_accounts);

    token::mint_to(cpi_transfer_context, amount)?;

     Ok(())
     
    }


    #[derive(Accounts)]
    pub struct CreateMintAnchor<'info> {
        #[account(mut)]
        pub signer : Signer<'info>,

        // mint account initialized on-chain ..lmaports and rent handled by anchor account struct 
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