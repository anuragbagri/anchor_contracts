use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create
    }
}
declare_id!("QdTg9Ufh35JZBjVTaKdsz75KQi87CiX5JXq21LnrfBp");


#[program]
pub mod nft_minter {
    use anchor_spl::token::Mint;

    use super::*;
    pub fn mint_nft() -> Result<()> {

    }

    #{derive(Accounts)}
    pub struct CreateNft<'info> {
        #[account(mut)]
        pub payer : Signer<'info>,
        // derive mint account 
        #[account(
            init,
            payer=payer,
            mint::decimals = 0,
            mint::authority = payer.key(),
            mint::freeze_authority = payer.key()
        )]
        pub mint_account : Account<'info,Mint>

        // create metadata account 
        #[account(
            mut,
            seeds = [b"metadata", token_meta]
        )]
    }
}