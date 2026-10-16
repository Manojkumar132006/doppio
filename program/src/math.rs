use crate::{
    error::DoppioError,
    state::{MAX_LTV_PERCENTAGE, RATE_DECIMALS_SCALE},
};

/// Computes the USD/USDC value of collateral in micro-USDC (6 decimals)
/// Formula: (collateral_lamports * rate * price) / (1e9 * 1e9)
pub fn compute_collateral_value_usdc(
    collateral_lamports: u64,
    rate: u64,
    price_micro_usdc: u64,
) -> Result<u128, DoppioError> {
    let c = collateral_lamports as u128;
    let r = rate as u128;
    let p = price_micro_usdc as u128;

    let numerator = c
        .checked_mul(r)
        .ok_or(DoppioError::NumericalOverflow)?
        .checked_mul(p)
        .ok_or(DoppioError::NumericalOverflow)?;

    let denominator = RATE_DECIMALS_SCALE
        .checked_mul(RATE_DECIMALS_SCALE)
        .ok_or(DoppioError::NumericalOverflow)?;

    Ok(numerator / denominator)
}

/// Computes the maximum borrowable credit based on 70% LTV
pub fn compute_max_borrow_usdc(collateral_value_usdc: u128) -> u128 {
    (collateral_value_usdc * MAX_LTV_PERCENTAGE) / 100
}

/// Computes harvestable yield value in micro-USDC and harvestable collateral shares
/// Yield Value (micro-USDC) = (N * (R1 - R0) * P) / 1e18
/// Harvestable Collateral = (N * (R1 - R0)) / R1
pub fn compute_yield_and_shares(
    collateral: u64,
    old_rate: u64,
    new_rate: u64,
    price: u64,
) -> Result<(u128, u128), DoppioError> {
    if new_rate <= old_rate || collateral == 0 {
        return Ok((0, 0));
    }

    let n = collateral as u128;
    let r0 = old_rate as u128;
    let r1 = new_rate as u128;
    let p = price as u128;

    let delta_r = r1.checked_sub(r0).ok_or(DoppioError::NumericalOverflow)?;

    let denominator = RATE_DECIMALS_SCALE
        .checked_mul(RATE_DECIMALS_SCALE)
        .ok_or(DoppioError::NumericalOverflow)?;

    let yield_value = n
        .checked_mul(delta_r)
        .ok_or(DoppioError::NumericalOverflow)?
        .checked_mul(p)
        .ok_or(DoppioError::NumericalOverflow)?
        / denominator;

    let delta_shares = n
        .checked_mul(delta_r)
        .ok_or(DoppioError::NumericalOverflow)?
        / r1;

    Ok((yield_value, delta_shares))
}

