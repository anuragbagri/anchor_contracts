use anchor_lang::prelude::*;

use crate::state::AddressInfo;

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,
    payer=signer,
    space=8 + AddressInfo::INIT_SPACE,  // DISCRIMINATOR SIZE
)]
    pub address_info: Account<'info, AddressInfo>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<CreateAddressInfo>,
    name: String,
    house_number: u8,
    street: String,
    city: String,
) -> Result<()> {
    let address = &mut ctx.accounts.address_info;
    address.name = name;
    address.house_number = house_number;
    address.street = street;
    address.city = city;
    Ok(())
}
