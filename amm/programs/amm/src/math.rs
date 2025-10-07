use crate::error::AmmError;
use anchor_lang::prelude::*;

// safe multiplication
pub fn checked_mul(a: u128, b: u128) -> Result<u128> {
    a.checked_mul(b).ok_or(AmmError::Overflow.into())
}

// division ... hell i hate writing these utility functions
pub fn mul_div(a: u128, numerator: u128, denominator: u128) -> Result<U128> {
    // return a * numerator / denominator
    if denominator == 0 {
        return Err(AmmError::Overflow.into());
    }
    let product = checked_mul(a, numerator)?;
    Ok(product / denominator);
}

// get amount out using the  fee_numertor / fee_denominator

pub fn get_amount_out(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_numerator: u64,
    fee_denominator: u64,
) -> Result<u64> {
    require!(amount_in > 0, AmmError::InsufficientInputAmount);

    let amount_in_u128 = amount_in as u128;
    let reserve_in_u128 = reserve_in as u128;
    let reserve_out_u128 = reserve_out as u128;

    let amount_in_with_fee = checked_mul(amount_in_u128, fee_numerator as u128)?;
    let numerator = checked_mul(amount_in_with_fee, reserve_out_u128)?;
    let denom_part = checked_mul(reserve_in_u128, fee_denominator as u128)?;
    let denominator = denom_part
        .checked_add(amount_in_with_fee)
        .ok_or(AmmError::Overflow)?;

    let out = numerator / denominator;
}

pub fn get_amount_in(
    amount_out: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_numerator: u64,
    fee_denominator: u64,
) -> Result<u64> {
    require!(amount_out > 0, AmmError::InsufficientOutputAmount);
    let amount_out_u128 = amount_out as u128;
    let reserve_in_u128 = reserve_in as u128;
    let reserve_out_u128 = reserve_out as u128;

    let numerator = checked_mul(
        reserve_in_u128,
        checked_mul(amount_out_u128, fee_denominator as u128)?,
    )?;
    let denom_part = reserve_out_u128
        .checked_sub(amount_out_u128)
        .ok_or(AmmError::Overflow)?;
    let denominator = checked_mul(denom_part, fee_numerator as u128)?;
    let result = numerator
        .checked_div(denominator)
        .ok_or(AmmError::Overflow)?;
    // Add 1 to round up
    let result_plus_one = result.checked_add(1).ok_or(AmmError::Overflow)?;
    Ok(result_plus_one.try_into().map_err(|_| AmmError::Overflow)?)
}
