use solana_sdk::signature::{Keypair, Signer};
use crate::common::*;

#[test]
fn test_deposit_collateral_flow() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64;

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // Deposit 10 JitoSOL (10 * 1e9 lamports)
    let deposit_amount = 10_000_000_000u64;
    let deposit_ix = ix_deposit_collateral(
        &env.program_id,
        &env.user.pubkey(),
        &env.vault_pda,
        &env.config_pda,
        deposit_amount,
    );
    env.user_tx(&[deposit_ix]).expect("Deposit failed");

    let vault = env.get_vault();
    assert_eq!(vault.collateral, deposit_amount);
    assert_eq!(vault.debt, 0);
    assert_eq!(vault.rate, rate_0);
}

#[test]
fn test_card_tap_and_draw_credit_approved_8_50() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64; // $150.00 per SOL

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // User deposits 1 JitoSOL = $150.00 collateral value
    // 70% LTV max credit = $105.00 (105_000_000 micro-USDC)
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 1_000_000_000);
    env.user_tx(&[deposit_ix]).unwrap();

    // Section 2: "Tap to pay $8.50" (8_500_000 micro-USDC)
    let tap_amount = 8_500_000u64;
    let draw_ix = ix_draw_credit(
        &env.program_id,
        &env.user.pubkey(),
        &env.vault_pda,
        &env.config_pda,
        tap_amount,
    );
    env.user_tx(&[draw_ix]).expect("Tap to pay $8.50 should succeed");

    let vault = env.get_vault();
    assert_eq!(vault.debt, tap_amount, "Debt must be updated to $8.50");
}

#[test]
fn test_draw_credit_declined_over_limit_70_percent_ltv() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64; // $150.00 per SOL

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // User deposits 1 JitoSOL ($150.00 value). 70% LTV limit is $105.00
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 1_000_000_000);
    env.user_tx(&[deposit_ix]).unwrap();

    // Attempt to draw $110.00 (> $105.00 limit)
    let over_limit_amount = 110_000_000u64;
    let draw_ix = ix_draw_credit(
        &env.program_id,
        &env.user.pubkey(),
        &env.vault_pda,
        &env.config_pda,
        over_limit_amount,
    );
    let res = env.user_tx(&[draw_ix]);
    assert!(res.is_err(), "Must be declined: over limit");
}

#[test]
fn test_exact_70_percent_ltv_boundary() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64; // 1.000
    let price_0 = 100_000_000u64;   // $100.00

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // Deposit 1 JitoSOL ($100.00 value). 70% LTV exact limit = $70.00 (70_000_000 micro-USDC)
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 1_000_000_000);
    env.user_tx(&[deposit_ix]).unwrap();

    // Exactly at 70% ($70.00) -> MUST SUCCEED
    let exact_draw = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 70_000_000);
    env.user_tx(&[exact_draw]).expect("Drawing exact 70% LTV limit should succeed");
    assert_eq!(env.get_vault().debt, 70_000_000);

    // Attempting to draw even $0.01 more -> MUST FAIL
    let over_draw = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 10_000);
    let res = env.user_tx(&[over_draw]);
    assert!(res.is_err(), "Drawing even $0.01 above 70% LTV limit must fail");
}

#[test]
fn test_draw_credit_rejected_bad_signer_or_pda() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64;

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 1_000_000_000);
    env.user_tx(&[deposit_ix]).unwrap();

    // Attacker attempts to draw against user's vault
    let attacker = Keypair::new();
    env.svm.airdrop(&attacker.pubkey(), 1_000_000_000).unwrap();

    let malicious_draw_ix = ix_draw_credit(
        &env.program_id,
        &attacker.pubkey(),
        &env.vault_pda,
        &env.config_pda,
        5_000_000,
    );
    let res = env.custom_tx(&[malicious_draw_ix], &[&attacker]);
    assert!(res.is_err(), "Must reject bad account / invalid signer");
}

#[test]
fn test_multiple_consecutive_card_taps() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64;

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // Deposit 5 JitoSOL ($750.00 value -> max borrow = $525.00)
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 5_000_000_000);
    env.user_tx(&[deposit_ix]).unwrap();

    // Tap 1: Coffee ($8.50)
    let tap1 = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 8_500_000);
    env.user_tx(&[tap1]).unwrap();
    assert_eq!(env.get_vault().debt, 8_500_000);

    // Tap 2: Lunch ($14.40)
    let tap2 = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 14_400_000);
    env.user_tx(&[tap2]).unwrap();
    assert_eq!(env.get_vault().debt, 22_900_000); // 8.50 + 14.40 = $22.90

    // Tap 3: Grocery ($50.00)
    let tap3 = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 50_000_000);
    env.user_tx(&[tap3]).unwrap();
    assert_eq!(env.get_vault().debt, 72_900_000); // $72.90 total debt
}
