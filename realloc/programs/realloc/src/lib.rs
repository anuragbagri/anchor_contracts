use anchor_lang::prelude::*;

declare_id!("85kxahUDPgBTPHjChigZdcoCEPrca6qwGVQaHLkNUHMs");

#[program]
pub mod anchor_realloc {
    use super::*;

    pub fn initialize() -> Result<()> {
       
    }

    pub fn update() -> Result<()> {

    }


    #[derive(Accounts)]
    #[instruction(input : String)]
    pub struct InitializeAccount<'info> {
        #[account(mut)]
        pub signer : Signer<'info>,
        #[account(
            init,
            payer=signer,
            space = 
        )]

    }

    #[derive(Accounts)]
    #[instruction(input: String)]
    pub struct UpdateAccount<'info> {

    }

    #[account]
    pub struct Message {
        pub msg : String
    }

    impl Message {
        pub fn required_space(input_length : usize) -> usize {
            8 + 4 + input_length
        }
    }
}
