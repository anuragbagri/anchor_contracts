
use anchor_lang::prelude::*;

declare_id!("5MWqNUsshQGuCnyPTQC9avzc9Qq4ZVubdGNSwN5mdvD2");

#[program]
pub mod counter_program {

    use super::*;

    pub fn initialize(ctx: Context<InitializeCounter>) -> Result<()> {
        let account = &mut ctx.accounts.counter_account;
        account.counter = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<IncrementCounter>) -> Result<()> {
        let account = &mut ctx.accounts.counter_account;
        account.counter = account.counter.checked_add(1).unwrap();
        Ok(())
    }

    pub fn decrement(ctx: Context<DecrementCounter>) -> Result<()> {
        let account = &mut ctx.accounts.counter_account;
        account.counter = account.counter.checked_sub(1).unwrap();
        Ok(())
    }

    #[account]
    #[derive(InitSpace)]
    pub struct CounterAccount {
        pub counter: u64,
    }

    #[derive(Accounts)]
    pub struct InitializeCounter<'info> {
        #[account(mut)]
        pub signer: Signer<'info>,

        #[account(init,
        payer = signer ,
        space = 8 + CounterAccount::INIT_SPACE)]
        pub counter_account: Account<'info, CounterAccount>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct IncrementCounter<'info> {
        #[account(mut)]
        pub counter_account: Account<'info, CounterAccount>,
    }

    #[derive(Accounts)]
    pub struct DecrementCounter<'info> {
        #[account(mut)]
        pub counter_account: Account<'info, CounterAccount>,
    }
}
