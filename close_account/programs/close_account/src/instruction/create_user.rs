use crate::state::user_state::UserState;
use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct CreateUser<'info>{
  #[account(mut)]
  pub user : Signer<'info>,

  #[account(init, 
payer = user,
space = UserState::INIT_SPACE,
seeds = [
    b"USER",
    user.key().as_ref()
],
bump)]

pub user_account : Account<'info , UserState>,
pub system_program : Program<'info, System>
}


pub fn create_user(ctx : Context<CreateUser> , name : String) -> Result<()> {
    let account = &mut ctx.accounts.user_account;

    // get the bump from pda contraint 
    let bump = ctx.bumps.user_account;

    account.name = name;
    account.user = ctx.accounts.user.key();
    account.bump = bump;
    Ok(())

}