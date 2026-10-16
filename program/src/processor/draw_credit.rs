use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    ProgramResult,
};
use crate::{
    error::DoppioError,
    state::{Config, VaultPosition, MAX_LTV_PERCENTAGE, RATE_DECIMALS_SCALE, VAULT_SEED},
};

pub fn process_draw_credit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let mut iter = accounts.iter();
    let user_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let vault_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let config_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    // Signer valid?
    if !user_info.is_signer() {
        return Err(DoppioError::BadAccount.into());
    }

    // PDA valid?
    let (expected_pda, _bump) = find_program_address(
        &[VAULT_SEED, user_info.key().as_ref()],
        program_id,
    );
    if vault_info.key() != &expected_pda {
        return Err(DoppioError::BadAccount.into());
    }

    let vault = VaultPosition::load_mut(vault_info)?;
    if vault.owner != *user_info.key() {
        return Err(DoppioError::BadAccount.into());
    }

    let config = Config::load(config_info)?;

    // Calculate collateral value in micro-USDC:
    // collateral (LST lamports) * rate (LST->SOL, 1e9 scale) / 1e9 = SOL lamports
    // SOL lamports * price (micro-USDC per SOL) / 1e9 = micro-USDC value
    let collateral_u128 = vault.collateral as u128;
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

    // Debt + draw within 70% LTV?
    let max_borrow = (collateral_value_usdc * MAX_LTV_PERCENTAGE) / 100;
    let new_debt = (vault.debt as u128)
        .checked_add(amount as u128)
        .ok_or(DoppioError::NumericalOverflow)?;

    if new_debt > max_borrow {
        // Decline: over limit
        return Err(DoppioError::DeclineOverLimit.into());
    }

    // Approved: debt += amount
    vault.debt = new_debt as u64;
    Ok(())
}
