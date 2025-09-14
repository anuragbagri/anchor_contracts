use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub bump: u8,
    pub user: Pubkey,
    #[max_len(50)]
    pub name: String,
}
