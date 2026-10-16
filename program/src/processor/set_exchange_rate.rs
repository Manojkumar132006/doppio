use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    error::DoppioError,
    state::Config,
};

pub fn process_set_exchange_rate(
    accounts: &[AccountInfo],
    new_rate: u64,
) -> ProgramResult {
    let mut iter = accounts.iter();
    let admin_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let config_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !admin_info.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let config = Config::load_mut(config_info)?;
    if admin_info.key() != &config.admin {
        return Err(DoppioError::NotAdmin.into());
    }

    config.rate = new_rate;
    Ok(())
}

pub fn process_set_price(
    accounts: &[AccountInfo],
    new_price: u64,
) -> ProgramResult {
    let mut iter = accounts.iter();
    let admin_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let config_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !admin_info.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let config = Config::load_mut(config_info)?;
    if admin_info.key() != &config.admin {
        return Err(DoppioError::NotAdmin.into());
    }

    config.price = new_price;
    Ok(())
}
