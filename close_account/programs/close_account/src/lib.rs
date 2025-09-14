pub mod instruction;
pub mod state;

use crate::instruction::{create_user::*, remove_user::*};

use anchor_lang::prelude::*;

declare_id!("program id inside");

#[program]
pub mod close_account {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, name: String) -> Result<()> {
        instruction::create_user::create_user(ctx, name)
    }

    pub fn remove_user(ctx: Context<CloseAccount>) -> Result<()> {
        instruction::remove_user::remove_account(ctx)
    }
}
