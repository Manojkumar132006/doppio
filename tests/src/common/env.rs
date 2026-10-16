use litesvm::LiteSVM;
use solana_sdk::{
    account::Account,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use crate::common::constants::{CONFIG_SEED, VAULT_SEED};
use crate::common::state::{ConfigState, VaultPositionState};

/// Test environment harness providing LiteSVM instances pre-loaded with the Doppio program
pub struct DoppioTestEnv {
    pub svm: LiteSVM,
    pub program_id: Pubkey,
    pub admin: Keypair,
    pub user: Keypair,
    pub config_pda: Pubkey,
    pub vault_pda: Pubkey,
}

impl DoppioTestEnv {
    pub fn new() -> Self {
        let mut svm = LiteSVM::new();
        let program_bytes = include_bytes!("../../../target/deploy/program.so");
        let program_id = Pubkey::new_unique();
        svm.add_program(program_id, program_bytes);

        let admin = Keypair::new();
        let user = Keypair::new();

        svm.airdrop(&admin.pubkey(), 10_000_000_000).unwrap();
        svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

        let (config_pda, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);
        let (vault_pda, _) = Pubkey::find_program_address(
            &[VAULT_SEED, user.pubkey().as_ref()],
            &program_id,
        );

        // Pre-allocate PDA account buffers owned by the program
        svm.set_account(
            config_pda,
            Account {
                lamports: 10_000_000,
                data: vec![0u8; 64],
                owner: program_id,
                executable: false,
                rent_epoch: 0,
            },
        ).unwrap();

        svm.set_account(
            vault_pda,
            Account {
                lamports: 10_000_000,
                data: vec![0u8; 72],
                owner: program_id,
                executable: false,
                rent_epoch: 0,
            },
        ).unwrap();

        Self {
            svm,
            program_id,
            admin,
            user,
            config_pda,
            vault_pda,
        }
    }

    pub fn admin_tx(&mut self, instructions: &[Instruction]) -> Result<(), litesvm::types::FailedTransactionMetadata> {
        let admin = &self.admin;
        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(&admin.pubkey()),
            &[admin],
            self.svm.latest_blockhash(),
        );
        self.svm.send_transaction(tx).map(|_| ())
    }

    pub fn user_tx(&mut self, instructions: &[Instruction]) -> Result<(), litesvm::types::FailedTransactionMetadata> {
        let user = &self.user;
        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(&user.pubkey()),
            &[user],
            self.svm.latest_blockhash(),
        );
        self.svm.send_transaction(tx).map(|_| ())
    }

    pub fn custom_tx(
        &mut self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<(), litesvm::types::FailedTransactionMetadata> {
        let payer = signers[0];
        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(&payer.pubkey()),
            signers,
            self.svm.latest_blockhash(),
        );
        self.svm.send_transaction(tx).map(|_| ())
    }

    pub fn get_config(&self) -> ConfigState {
        let acc = self.svm.get_account(&self.config_pda).unwrap();
        ConfigState::unpack(&acc.data)
    }

    pub fn get_vault(&self) -> VaultPositionState {
        let acc = self.svm.get_account(&self.vault_pda).unwrap();
        VaultPositionState::unpack(&acc.data)
    }
}
