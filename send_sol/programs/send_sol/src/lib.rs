use anchor_lang::*;

declare_id("");


#[program]
pub mod send_sol {
    use super::*;
    
    pub fn sendviacpi() ->  Result<()>{

    }


    pub fn sendviaprogram() -> Result<()> {

    }

    #[derive(Accounts)]
    pub struct SendViaCpi<'info> {

    }

    #[derive(Accounts)]
    pub struct SendViaProgram<'info> {

    }
}