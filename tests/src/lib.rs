//! Doppio LiteSVM Integration & Unit Test Suite
//!
//! Organized into modular test suites and shared test harnesses:
//! - `common`: LiteSVM test environment (`DoppioTestEnv`), instruction builders, state mirrors, constants.
//! - `suites`:
//!   - `math`: Core LTV (70%), collateral valuation, and yield math.
//!   - `layout`: Zero-copy byte layout, alignment, and size guarantees.
//!   - `instruction`: Unpacking, discriminator dispatch, and boundary checks.
//!   - `initialization`: Account initializations and admin access controls.
//!   - `credit`: Collateral deposits, card taps ($8.50), and LTV limits.
//!   - `repay_withdraw`: Manual repayments and collateral withdrawals under LTV safety margins.
//!   - `harvest`: Automated self-repayment from yield, multi-epoch amortization, and lifecycle.

pub mod common;

#[cfg(test)]
mod suites;
