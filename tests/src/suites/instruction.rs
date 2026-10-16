#[test]
fn test_unpack_initialize_config() {
    let mut data = vec![0u8];
    data.extend_from_slice(&1_000_000_000u64.to_le_bytes());
    data.extend_from_slice(&150_000_000u64.to_le_bytes());

    let ix = program::instruction::DoppioInstruction::unpack(&data).unwrap();
    assert_eq!(
        ix,
        program::instruction::DoppioInstruction::InitializeConfig {
            rate: 1_000_000_000,
            price: 150_000_000,
        }
    );
}

#[test]
fn test_unpack_set_exchange_rate() {
    let mut data = vec![1u8];
    data.extend_from_slice(&1_080_000_000u64.to_le_bytes());

    let ix = program::instruction::DoppioInstruction::unpack(&data).unwrap();
    assert_eq!(
        ix,
        program::instruction::DoppioInstruction::SetExchangeRate {
            new_rate: 1_080_000_000,
        }
    );
}

#[test]
fn test_unpack_simple_instructions() {
    assert_eq!(
        program::instruction::DoppioInstruction::unpack(&[2u8]).unwrap(),
        program::instruction::DoppioInstruction::InitializeVault
    );
    assert_eq!(
        program::instruction::DoppioInstruction::unpack(&[5u8]).unwrap(),
        program::instruction::DoppioInstruction::HarvestAndRepay
    );
}

#[test]
fn test_unpack_deposit_and_draw() {
    let mut dep = vec![3u8];
    dep.extend_from_slice(&5_000_000_000u64.to_le_bytes());
    assert_eq!(
        program::instruction::DoppioInstruction::unpack(&dep).unwrap(),
        program::instruction::DoppioInstruction::DepositCollateral { amount: 5_000_000_000 }
    );

    let mut draw = vec![4u8];
    draw.extend_from_slice(&8_500_000u64.to_le_bytes());
    assert_eq!(
        program::instruction::DoppioInstruction::unpack(&draw).unwrap(),
        program::instruction::DoppioInstruction::DrawCredit { amount: 8_500_000 }
    );
}

#[test]
fn test_unpack_withdraw_and_repay() {
    let mut with = vec![7u8];
    with.extend_from_slice(&1_000_000_000u64.to_le_bytes());
    assert_eq!(
        program::instruction::DoppioInstruction::unpack(&with).unwrap(),
        program::instruction::DoppioInstruction::WithdrawCollateral { amount: 1_000_000_000 }
    );

    let mut rep = vec![8u8];
    rep.extend_from_slice(&4_250_000u64.to_le_bytes());
    assert_eq!(
        program::instruction::DoppioInstruction::unpack(&rep).unwrap(),
        program::instruction::DoppioInstruction::RepayDebt { amount: 4_250_000 }
    );
}

#[test]
fn test_unpack_invalid_cases() {
    assert!(program::instruction::DoppioInstruction::unpack(&[]).is_err());
    assert!(program::instruction::DoppioInstruction::unpack(&[99u8]).is_err());
    let mut truncated = vec![0u8];
    truncated.extend_from_slice(&100u64.to_le_bytes());
    assert!(program::instruction::DoppioInstruction::unpack(&truncated).is_err());
}
