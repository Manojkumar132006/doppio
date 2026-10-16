use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::error::DoppioError;

/// Seed constant for Config PDA
pub const CONFIG_SEED: &[u8] = b"config";

/// Seed constant for VaultPosition PDA
pub const VAULT_SEED: &[u8] = b"vault";

/// 1e9 scale used for exchange rates and LST lamports
pub const RATE_DECIMALS_SCALE: u128 = 1_000_000_000;

/// Maximum allowable Loan-to-Value percentage (70%)
pub const MAX_LTV_PERCENTAGE: u128 = 70;

/// Global Config PDA
/// Layout:
/// - discriminator: [u8; 8] ("dop_conf")
/// - admin: Pubkey (32 bytes)
/// - rate: u64 (8 bytes, exchange rate scaled by 1e9, e.g. 1_000_000_000 = 1.0)
/// - price: u64 (8 bytes, price of SOL in micro-USDC, e.g. 150_000_000 = $150.00)
/// - bump: u8 (1 byte)
/// - _padding: [u8; 7] (7 bytes)
///
/// Total: 64 bytes
#[repr(C)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub discriminator: [u8; 8],
    pub admin: Pubkey,
    pub rate: u64,
    pub price: u64,
    pub bump: u8,
    pub _padding: [u8; 7],
}

impl Config {
    pub const DISCRIMINATOR: [u8; 8] = *b"dop_conf";
    pub const LEN: usize = core::mem::size_of::<Self>();

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub fn load_mut(info: &AccountInfo) -> Result<&mut Self, ProgramError> {
        let data = unsafe { info.borrow_mut_data_unchecked() };
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let config = unsafe { &mut *(data.as_mut_ptr() as *mut Self) };
        if config.discriminator != Self::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(config)
    }

    #[inline(always)]
    pub fn load(info: &AccountInfo) -> Result<&Self, ProgramError> {
        let data = unsafe { info.borrow_data_unchecked() };
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let config = unsafe { &*(data.as_ptr() as *const Self) };
        if config.discriminator != Self::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(config)
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub fn init_zero_copy(
        info: &AccountInfo,
        admin: Pubkey,
        rate: u64,
        price: u64,
        bump: u8,
    ) -> Result<&mut Self, ProgramError> {
        let data = unsafe { info.borrow_mut_data_unchecked() };
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let config = unsafe { &mut *(data.as_mut_ptr() as *mut Self) };
        if config.discriminator == Self::DISCRIMINATOR {
            return Err(DoppioError::AlreadyInitialized.into());
        }
        config.discriminator = Self::DISCRIMINATOR;
        config.admin = admin;
        config.rate = rate;
        config.price = price;
        config.bump = bump;
        config._padding = [0; 7];
        Ok(config)
    }
}

/// User Vault Position PDA
/// Layout:
/// - discriminator: [u8; 8] ("dop_vpos")
/// - owner: Pubkey (32 bytes)
/// - collateral: u64 (8 bytes, raw LST lamports)
/// - debt: u64 (8 bytes, borrowed credit in micro-USDC)
/// - rate: u64 (8 bytes, exchange rate at last deposit/harvest)
/// - bump: u8 (1 byte)
/// - _padding: [u8; 7] (7 bytes)
///
/// Total: 72 bytes
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VaultPosition {
    pub discriminator: [u8; 8],
    pub owner: Pubkey,
    pub collateral: u64,
    pub debt: u64,
    pub rate: u64,
    pub bump: u8,
    pub _padding: [u8; 7],
}

impl VaultPosition {
    pub const DISCRIMINATOR: [u8; 8] = *b"dop_vpos";
    pub const LEN: usize = core::mem::size_of::<Self>();

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub fn load_mut(info: &AccountInfo) -> Result<&mut Self, ProgramError> {
        let data = unsafe { info.borrow_mut_data_unchecked() };
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let vault = unsafe { &mut *(data.as_mut_ptr() as *mut Self) };
        if vault.discriminator != Self::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(vault)
    }

    #[inline(always)]
    pub fn load(info: &AccountInfo) -> Result<&Self, ProgramError> {
        let data = unsafe { info.borrow_data_unchecked() };
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let vault = unsafe { &*(data.as_ptr() as *const Self) };
        if vault.discriminator != Self::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(vault)
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub fn init_zero_copy(

        info: &AccountInfo,
        owner: Pubkey,
        initial_rate: u64,
        bump: u8,
    ) -> Result<&mut Self, ProgramError> {
        let data = unsafe { info.borrow_mut_data_unchecked() };
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let vault = unsafe { &mut *(data.as_mut_ptr() as *mut Self) };
        if vault.discriminator == Self::DISCRIMINATOR {
            return Err(DoppioError::AlreadyInitialized.into());
        }
        vault.discriminator = Self::DISCRIMINATOR;
        vault.owner = owner;
        vault.collateral = 0;
        vault.debt = 0;
        vault.rate = initial_rate;
        vault.bump = bump;
        vault._padding = [0; 7];
        Ok(vault)
    }
}


