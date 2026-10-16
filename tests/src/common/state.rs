use solana_sdk::pubkey::Pubkey;
use crate::common::constants::{CONFIG_DISCRIMINATOR, VAULT_DISCRIMINATOR};

/// State mirror for deserialization and validation in tests
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigState {
    pub discriminator: [u8; 8],
    pub admin: Pubkey,
    pub rate: u64,
    pub price: u64,
    pub bump: u8,
}

impl ConfigState {
    pub fn unpack(data: &[u8]) -> Self {
        assert!(data.len() >= 64, "Config data too small");
        let mut disc = [0u8; 8];
        disc.copy_from_slice(&data[0..8]);
        assert_eq!(disc, CONFIG_DISCRIMINATOR);
        let admin = Pubkey::new_from_array(data[8..40].try_into().unwrap());
        let rate = u64::from_le_bytes(data[40..48].try_into().unwrap());
        let price = u64::from_le_bytes(data[48..56].try_into().unwrap());
        let bump = data[56];
        Self {
            discriminator: disc,
            admin,
            rate,
            price,
            bump,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultPositionState {
    pub discriminator: [u8; 8],
    pub owner: Pubkey,
    pub collateral: u64,
    pub debt: u64,
    pub rate: u64,
    pub bump: u8,
}

impl VaultPositionState {
    pub fn unpack(data: &[u8]) -> Self {
        assert!(data.len() >= 72, "VaultPosition data too small");
        let mut disc = [0u8; 8];
        disc.copy_from_slice(&data[0..8]);
        assert_eq!(disc, VAULT_DISCRIMINATOR);
        let owner = Pubkey::new_from_array(data[8..40].try_into().unwrap());
        let collateral = u64::from_le_bytes(data[40..48].try_into().unwrap());
        let debt = u64::from_le_bytes(data[48..56].try_into().unwrap());
        let rate = u64::from_le_bytes(data[56..64].try_into().unwrap());
        let bump = data[64];
        Self {
            discriminator: disc,
            owner,
            collateral,
            debt,
            rate,
            bump,
        }
    }
}
