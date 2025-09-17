use anchor_lang::prelude::*;
use anchor_lang::system_program::create_account;

declare_id!("FkxkXWbUBMroCLeRPAwZir9vd2BHaqH4QivrK7C29M1V");

#[program]
pub mod create_system_account {

    use anchor_lang::system_program::CreateAccount;

    use super::*;

    pub fn create_system_account(ctx: Context<CreateSystemAccount>) -> Result<()> {
        msg!(
            "new account public key is : {} ",
            &mut ctx.accounts.new_account.key().to_string()
        );
        let lamports = (Rent::get()?).minimum_balance(0);

        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.new_account.to_account_info(),
                },
            ),
            lamports,
            0,
            &ctx.accounts.system_program.key(),
        )?;
        Ok(())
    }

    #[derive(Accounts)]
    pub struct CreateSystemAccount<'info> {
        #[account(mut)]
        pub signer: Signer<'info>,
        #[account(mut)]
        pub new_account: Signer<'info>,
        pub system_program: Program<'info, System>,
    }
}
