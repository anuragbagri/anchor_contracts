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

// sqrt
