use pinocchio::program_error::ProgramError;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoppioInstruction {
    /// 0: InitializeConfig { rate: u64, price: u64 }
    InitializeConfig { rate: u64, price: u64 },
    /// 1: SetExchangeRate { new_rate: u64 }
    SetExchangeRate { new_rate: u64 },
    /// 2: InitializeVault
    InitializeVault,
    /// 3: DepositCollateral { amount: u64 }
    DepositCollateral { amount: u64 },
    /// 4: DrawCredit { amount: u64 }
    DrawCredit { amount: u64 },
    /// 5: HarvestAndRepay
    HarvestAndRepay,
    /// 6: SetPrice { new_price: u64 }
    SetPrice { new_price: u64 },
    /// 7: WithdrawCollateral { amount: u64 }
    WithdrawCollateral { amount: u64 },
    /// 8: RepayDebt { amount: u64 }
    RepayDebt { amount: u64 },
}

impl DoppioInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            0 => {
                if rest.len() < 16 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let rate = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                let price = u64::from_le_bytes(rest[8..16].try_into().unwrap());
                Ok(Self::InitializeConfig { rate, price })
            }
            1 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let new_rate = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok(Self::SetExchangeRate { new_rate })
            }
            2 => Ok(Self::InitializeVault),
            3 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok(Self::DepositCollateral { amount })
            }
            4 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok(Self::DrawCredit { amount })
            }
            5 => Ok(Self::HarvestAndRepay),
            6 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let new_price = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok(Self::SetPrice { new_price })
            }
            7 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok(Self::WithdrawCollateral { amount })
            }
            8 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok(Self::RepayDebt { amount })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}



