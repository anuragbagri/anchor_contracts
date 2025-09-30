use anchor_lang::prelude::*;

declare_id!("QdTg9Ufh35JZBjVTaKdsz75KQi87CiX5JXq21LnrfBp");

#[program]
pub mod nft_minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
