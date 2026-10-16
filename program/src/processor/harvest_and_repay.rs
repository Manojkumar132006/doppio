use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    error::DoppioError,
    state::{Config, VaultPosition, RATE_DECIMALS_SCALE},
};

pub fn process_harvest_and_repay(
    accounts: &[AccountInfo],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let vault_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let config_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    let vault = VaultPosition::load_mut(vault_info)?;
    let config = Config::load(config_info)?;

    // Rate went up?
    if config.rate <= vault.rate {
        // No-op (no yield yet)
        return Ok(());
    }

    if vault.collateral == 0 {
        vault.rate = config.rate;
        return Ok(());
    }

    let collateral_u128 = vault.collateral as u128;
    let old_rate_u128 = vault.rate as u128;
    let new_rate_u128 = config.rate as u128;
    let price_u128 = config.price as u128;

    let rate_delta = new_rate_u128
        .checked_sub(old_rate_u128)
        .ok_or(DoppioError::NumericalOverflow)?;

    // Delta SOL (lamports) = N * (R1 - R0) / 1e9
    // Yield value (micro-USDC) = Delta SOL * price / 1e9 = N * (R1 - R0) * price / 1e18
    let denominator = RATE_DECIMALS_SCALE
        .checked_mul(RATE_DECIMALS_SCALE)
        .ok_or(DoppioError::NumericalOverflow)?;

    let yield_value = collateral_u128
        .checked_mul(rate_delta)
        .ok_or(DoppioError::NumericalOverflow)?
        .checked_mul(price_u128)
        .ok_or(DoppioError::NumericalOverflow)?
        / denominator;

    // Harvestable collateral shares: Delta N = N * (R1 - R0) / R1
    let delta_collateral = collateral_u128
        .checked_mul(rate_delta)
        .ok_or(DoppioError::NumericalOverflow)?
        / new_rate_u128;

    let current_debt = vault.debt as u128;

    // Yield value at least debt?
    if yield_value >= current_debt {
        // Debt cleared (repay = debt)
        if yield_value > 0 && current_debt > 0 {
            // Collateral to shrink is proportional to debt cleared
            let collateral_shrink = delta_collateral
                .checked_mul(current_debt)
                .ok_or(DoppioError::NumericalOverflow)?
                / yield_value;

            vault.collateral = (vault.collateral as u128)
                .saturating_sub(collateral_shrink) as u64;
        }
        vault.debt = 0;
        vault.rate = config.rate;
    } else {
        // Debt reduced (repay = yield value)
        vault.debt = (vault.debt as u128)
            .saturating_sub(yield_value) as u64;
        vault.collateral = (vault.collateral as u128)
            .saturating_sub(delta_collateral) as u64;
        vault.rate = config.rate;
    }

    Ok(())
}
