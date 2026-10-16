use pinocchio::program_error::ProgramError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoppioError {
    /// 0: Caller is not the admin
    NotAdmin = 0,
    /// 1: Signer or PDA is invalid (bad account)
    BadAccount = 1,
    /// 2: Credit draw exceeds 70% LTV limit (over limit)
    DeclineOverLimit = 2,
    /// 3: Instruction data is invalid
    InvalidInstructionData = 3,
    /// 4: Account already initialized
    AlreadyInitialized = 4,
    /// 5: Account is uninitialized
    UninitializedAccount = 5,
    /// 6: Arithmetic overflow occurred
    NumericalOverflow = 6,
    /// 7: Insufficient collateral for operation
    InsufficientCollateral = 7,
    /// 8: Invalid account owner
    InvalidOwner = 8,
}

impl From<DoppioError> for ProgramError {
    fn from(e: DoppioError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
