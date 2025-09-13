use anchor_lang::prelude::*;
pub mod instruction;
pub mod state;
use instruction::*;

declare_id!("");

#[program]
pub mod address_account {

    use super::*;
    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        create::initialize(ctx, name, house_number, street, city);
        Ok(())
    }
}
