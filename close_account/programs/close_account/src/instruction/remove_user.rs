use crate::state::{user_state::UserState, *};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    mut,    // account data or lamports can be modified inside the instruction
    seeds = [
        b"user",
        user.key().as_ref()
    ],
    bump = user_account.bump,
    close = user // close the account n send the lamports to user
  )]
    pub user_account: Account<'info, UserState>,
}

pub fn remove_account(_ctx: Context<CloseAccount>) -> Result<()> {
    Ok(())
}
