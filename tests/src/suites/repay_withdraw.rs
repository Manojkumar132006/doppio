use crate::common::*;

#[test]
fn test_manual_repay_debt_flow() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64;

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // User deposits 1 JitoSOL
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 1_000_000_000);
    env.user_tx(&[deposit_ix]).unwrap();

    // User draws $20.00 debt
    let draw_ix = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 20_000_000);
    env.user_tx(&[draw_ix]).unwrap();
    assert_eq!(env.get_vault().debt, 20_000_000);

    // User manually repays $12.00
    let repay_ix = ix_repay_debt(&env.program_id, &env.user.pubkey(), &env.vault_pda, 12_000_000);
    env.user_tx(&[repay_ix]).expect("Manual repay should succeed");
    assert_eq!(env.get_vault().debt, 8_000_000);

    // User manually repays remaining $10.00 (more than remaining $8.00 debt)
    let repay_all_ix = ix_repay_debt(&env.program_id, &env.user.pubkey(), &env.vault_pda, 10_000_000);
    env.user_tx(&[repay_all_ix]).expect("Manual repay all should succeed");
    assert_eq!(env.get_vault().debt, 0, "Debt must be 0");
}

#[test]
fn test_withdraw_collateral_ltv_enforcement() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64; // 1.000
    let price_0 = 150_000_000u64;   // $150.00

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // User deposits 2 JitoSOL ($300 value)
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 2_000_000_000);
    env.user_tx(&[deposit_ix]).unwrap();

    // User draws $80.00 debt
    let draw_ix = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 80_000_000);
    env.user_tx(&[draw_ix]).unwrap();

    // If user tries to withdraw 1.5 JitoSOL, remaining is 0.5 JitoSOL ($75.00 value).
    // 70% LTV on $75.00 is $52.50, but user owes $80.00 -> MUST BE DECLINED!
    let invalid_withdraw = ix_withdraw_collateral(
        &env.program_id,
        &env.user.pubkey(),
        &env.vault_pda,
        &env.config_pda,
        1_500_000_000,
    );
    let res = env.user_tx(&[invalid_withdraw]);
    assert!(res.is_err(), "Withdrawal violating 70% LTV must be declined");

    // If user withdraws 0.5 JitoSOL, remaining is 1.5 JitoSOL ($225.00 value).
    // 70% LTV on $225.00 is $157.50 >= $80.00 debt -> MUST SUCCEED!
    let valid_withdraw = ix_withdraw_collateral(
        &env.program_id,
        &env.user.pubkey(),
        &env.vault_pda,
        &env.config_pda,
        500_000_000,
    );
    env.user_tx(&[valid_withdraw]).expect("Withdrawal within 70% LTV must succeed");

    assert_eq!(env.get_vault().collateral, 1_500_000_000);
    assert_eq!(env.get_vault().debt, 80_000_000);
}
