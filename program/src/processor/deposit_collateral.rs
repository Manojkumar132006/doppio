use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    error::DoppioError,
    state::{Config, VaultPosition},
};

pub fn process_deposit_collateral(
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

    let config = Config::load(config_info)?;

    // If there was no collateral or no debt, align the baseline rate to current config rate
    if vault.collateral == 0 || vault.debt == 0 {
        vault.rate = config.rate;
    }

    vault.collateral = vault
        .collateral
        .checked_add(amount)
        .ok_or(DoppioError::NumericalOverflow)?;

    Ok(())
}
