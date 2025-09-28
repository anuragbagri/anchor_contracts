use anchor_lang::prelude::*;

declare_id("");


#[program]
pub mod send_sol {

    use anchor_lang::{solana_program::system_program, system_program};

    use super::*;
    
    pub fn sendviacpi(ctx : Context<SendViaCpi> , amount : u64) ->  Result<()>{
     system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from : ctx.accounts.sender.to_account_info(),
            to : ctx.accounts.reciever.to_account_info()
        },
    ),
    amount
     )?;
     Ok(())
    }


    pub fn sendviaprogram(
        ctx: Context<SendViaCpi>,
        amount: u64,
    ) -> Result<()> {
        **ctx.accounts.sender.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.receiver.try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    #[derive(Accounts)]
    pub struct SendViaCpi<'info> {
     #[account(mut)]
     pub sender : Signer<'info>,
     #[account(mut)]
     pub receive : SystemAccount<'info>,
     pub system_program : Program<'info, System>
    }

    #[derive(Accounts)]
    pub struct SendViaProgram<'info> {
     #[account(
        mut,
        owner = id()
     )]
     pub sender : Signer<'info>,
     #[account(mut)]
     pub reciever : SystemAccount<'info> 
    }
}