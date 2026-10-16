use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    error::DoppioError,
    state::{Config, VaultPosition, MAX_LTV_PERCENTAGE, RATE_DECIMALS_SCALE},
};

pub fn process_withdraw_collateral(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let mut iter = accounts.iter();
    let user_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let vault_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let config_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !user_info.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault = VaultPosition::load_mut(vault_info)?;
    if vault.owner != *user_info.key() {
        return Err(DoppioError::InvalidOwner.into());
    }

    if vault.collateral < amount {
        return Err(DoppioError::InsufficientCollateral.into());
    }

    let remaining_collateral = vault.collateral - amount;

    // If there is outstanding debt, ensure remaining collateral satisfies 70% LTV
    if vault.debt > 0 {
        let config = Config::load(config_info)?;
        let collateral_u128 = remaining_collateral as u128;
        let rate_u128 = config.rate as u128;
        let price_u128 = config.price as u128;

        let numerator = collateral_u128
            .checked_mul(rate_u128)
            .ok_or(DoppioError::NumericalOverflow)?
            .checked_mul(price_u128)
            .ok_or(DoppioError::NumericalOverflow)?;

        let denominator = RATE_DECIMALS_SCALE
            .checked_mul(RATE_DECIMALS_SCALE)
            .ok_or(DoppioError::NumericalOverflow)?;

        let collateral_value_usdc = numerator / denominator;
        let max_borrow = (collateral_value_usdc * MAX_LTV_PERCENTAGE) / 100;

        if (vault.debt as u128) > max_borrow {
            return Err(DoppioError::DeclineOverLimit.into());
        }
    }

    vault.collateral = remaining_collateral;
    Ok(())
}
