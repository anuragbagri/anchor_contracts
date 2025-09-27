use anchor_lang::prelude::*;

declare_id!("85kxahUDPgBTPHjChigZdcoCEPrca6qwGVQaHLkNUHMs");

#[program]
pub mod anchor_realloc {

    use anchor_lang::solana_program::message::Message;

    use super::*;

    pub fn initialize(ctx: Context<InitializeAccount>, input: String) -> Result<()> {
        ctx.accounts.message_account.msg = input;
        Ok(())
    }

    pub fn update(ctx: Context<UpdateAccount>, input: String) -> Result<()> {}

    #[derive(Accounts)]
    #[instruction(input : String)]
    pub struct InitializeAccount<'info> {
        #[account(mut)]
        pub signer: Signer<'info>,
        #[account(
            init,
            payer=signer,
            space = Message::required_space(input)
        )]
        pub message_account: Account<'info, Message>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    #[instruction(input: String)]
    pub struct UpdateAccount<'info> {
        #[account(mut)]
        pub signer: Signer<'info>,

        #[account(
            mut,
            realloc = Message::required_space(input.len()),
            realloc::payer = signer,
            realloc::zero = true
        )]
        pub message_account: Account<'info, Message>,
        pub system_program: Program<'info, System>,
    }

    #[account]
    pub struct Message {
        pub msg: String,
    }

    impl Message {
        pub fn required_space(input_length: usize) -> usize {
            8 + 4 + input_length
        }
    }
}
