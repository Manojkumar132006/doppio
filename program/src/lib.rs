pub mod error;
pub mod instruction;
pub mod math;
pub mod processor;
pub mod state;


#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Doppio Program ID: 4xu1Gz6G3zoEYqRffUNXcsfo4b4dDgEH5bRAm4UXmc3F
pub const ID: pinocchio::pubkey::Pubkey = [
    58, 228, 206, 167, 42, 175, 35, 176,
    100, 67, 182, 246, 214, 228, 133, 84,
    239, 30, 241, 56, 174, 251, 157, 114,
    13, 118, 96, 162, 88, 224, 220, 206,
];


