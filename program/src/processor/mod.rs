pub mod deposit_collateral;
pub mod draw_credit;
pub mod harvest_and_repay;
pub mod initialize_config;
pub mod initialize_vault;
pub mod repay_debt;
pub mod set_exchange_rate;
pub mod withdraw_collateral;

use pinocchio::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    ProgramResult,
};
use crate::instruction::DoppioInstruction;

pub use deposit_collateral::process_deposit_collateral;
pub use draw_credit::process_draw_credit;
pub use harvest_and_repay::process_harvest_and_repay;
pub use initialize_config::process_initialize_config;
pub use initialize_vault::process_initialize_vault;
pub use repay_debt::process_repay_debt;
pub use set_exchange_rate::{process_set_exchange_rate, process_set_price};
pub use withdraw_collateral::process_withdraw_collateral;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = DoppioInstruction::unpack(instruction_data)?;

    match instruction {
        DoppioInstruction::InitializeConfig { rate, price } => {
            process_initialize_config(program_id, accounts, rate, price)
        }
        DoppioInstruction::SetExchangeRate { new_rate } => {
            process_set_exchange_rate(accounts, new_rate)
        }
        DoppioInstruction::InitializeVault => {
            process_initialize_vault(program_id, accounts)
        }
        DoppioInstruction::DepositCollateral { amount } => {
            process_deposit_collateral(accounts, amount)
        }
        DoppioInstruction::DrawCredit { amount } => {
            process_draw_credit(program_id, accounts, amount)
        }
        DoppioInstruction::HarvestAndRepay => {
            process_harvest_and_repay(accounts)
        }
        DoppioInstruction::SetPrice { new_price } => {
            process_set_price(accounts, new_price)
        }
        DoppioInstruction::WithdrawCollateral { amount } => {
            process_withdraw_collateral(accounts, amount)
        }
        DoppioInstruction::RepayDebt { amount } => {
            process_repay_debt(accounts, amount)
        }
    }
}

