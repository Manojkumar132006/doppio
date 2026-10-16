use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    ProgramResult,
};
use crate::{
    error::DoppioError,
    state::{Config, CONFIG_SEED},
};

pub fn process_initialize_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    rate: u64,
    price: u64,
) -> ProgramResult {
    let mut iter = accounts.iter();
    let admin_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let config_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !admin_info.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (expected_pda, bump) = find_program_address(&[CONFIG_SEED], program_id);
    if config_info.key() != &expected_pda {
        return Err(DoppioError::BadAccount.into());
    }

    Config::init_zero_copy(config_info, *admin_info.key(), rate, price, bump)?;
    Ok(())
}
