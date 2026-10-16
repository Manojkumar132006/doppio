use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    ProgramResult,
};
use crate::{
    error::DoppioError,
    state::{Config, VaultPosition, VAULT_SEED},
};

pub fn process_initialize_vault(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let user_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let vault_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let config_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !user_info.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (expected_pda, bump) = find_program_address(
        &[VAULT_SEED, user_info.key().as_ref()],
        program_id,
    );
    if vault_info.key() != &expected_pda {
        return Err(DoppioError::BadAccount.into());
    }

    let config = Config::load(config_info)?;
    VaultPosition::init_zero_copy(vault_info, *user_info.key(), config.rate, bump)?;
    Ok(())
}
