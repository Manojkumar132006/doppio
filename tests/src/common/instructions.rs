use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub fn ix_initialize_config(
    program_id: &Pubkey,
    admin: &Pubkey,
    config_pda: &Pubkey,
    rate: u64,
    price: u64,
) -> Instruction {
    let mut data = Vec::with_capacity(17);
    data.push(0u8); // tag 0
    data.extend_from_slice(&rate.to_le_bytes());
    data.extend_from_slice(&price.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*admin, true),
            AccountMeta::new(*config_pda, false),
        ],
        data,
    }
}

pub fn ix_set_exchange_rate(
    program_id: &Pubkey,
    admin: &Pubkey,
    config_pda: &Pubkey,
    new_rate: u64,
) -> Instruction {
    let mut data = Vec::with_capacity(9);
    data.push(1u8); // tag 1
    data.extend_from_slice(&new_rate.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*admin, true),
            AccountMeta::new(*config_pda, false),
        ],
        data,
    }
}

pub fn ix_initialize_vault(
    program_id: &Pubkey,
    user: &Pubkey,
    vault_pda: &Pubkey,
    config_pda: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new(*vault_pda, false),
            AccountMeta::new_readonly(*config_pda, false),
        ],
        data: vec![2u8], // tag 2
    }
}

pub fn ix_deposit_collateral(
    program_id: &Pubkey,
    user: &Pubkey,
    vault_pda: &Pubkey,
    config_pda: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = Vec::with_capacity(9);
    data.push(3u8); // tag 3
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new(*vault_pda, false),
            AccountMeta::new_readonly(*config_pda, false),
        ],
        data,
    }
}

pub fn ix_draw_credit(
    program_id: &Pubkey,
    user: &Pubkey,
    vault_pda: &Pubkey,
    config_pda: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = Vec::with_capacity(9);
    data.push(4u8); // tag 4
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new(*vault_pda, false),
            AccountMeta::new_readonly(*config_pda, false),
        ],
        data,
    }
}

pub fn ix_harvest_and_repay(
    program_id: &Pubkey,
    vault_pda: &Pubkey,
    config_pda: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*vault_pda, false),
            AccountMeta::new_readonly(*config_pda, false),
        ],
        data: vec![5u8], // tag 5
    }
}

pub fn ix_withdraw_collateral(
    program_id: &Pubkey,
    user: &Pubkey,
    vault_pda: &Pubkey,
    config_pda: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = Vec::with_capacity(9);
    data.push(7u8); // tag 7
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new(*vault_pda, false),
            AccountMeta::new_readonly(*config_pda, false),
        ],
        data,
    }
}

pub fn ix_repay_debt(
    program_id: &Pubkey,
    user: &Pubkey,
    vault_pda: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = Vec::with_capacity(9);
    data.push(8u8); // tag 8
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new(*vault_pda, false),
        ],
        data,
    }
}
