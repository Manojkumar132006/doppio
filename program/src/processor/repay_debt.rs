use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    error::DoppioError,
    state::VaultPosition,
};

pub fn process_repay_debt(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let mut iter = accounts.iter();
    let user_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let vault_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !user_info.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault = VaultPosition::load_mut(vault_info)?;
    if vault.owner != *user_info.key() {
        return Err(DoppioError::InvalidOwner.into());
    }

    let repay_amount = amount.min(vault.debt);
    vault.debt = vault.debt.saturating_sub(repay_amount);

    Ok(())
}
