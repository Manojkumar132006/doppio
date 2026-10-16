use crate::common::*;

#[test]
fn test_harvest_and_repay_noop_when_no_yield() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64;

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    let deposit_amount = 2_000_000_000u64; // 2 JitoSOL
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, deposit_amount);
    env.user_tx(&[deposit_ix]).unwrap();

    // Draw $8.50
    let draw_ix = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 8_500_000);
    env.user_tx(&[draw_ix]).unwrap();

    // Rate did not go up (still rate_0).
    // Flow: "Rate went up? -> no: No-op (no yield yet)"
    let harvest_ix = ix_harvest_and_repay(&env.program_id, &env.vault_pda, &env.config_pda);
    env.user_tx(&[harvest_ix]).expect("Harvest should execute as a no-op");

    let vault = env.get_vault();
    assert_eq!(vault.debt, 8_500_000, "Debt must remain unchanged");
    assert_eq!(vault.collateral, deposit_amount, "Collateral must remain unchanged");
    assert_eq!(vault.rate, rate_0, "Rate must remain unchanged");
}

#[test]
fn test_harvest_and_repay_debt_cleared_when_yield_exceeds_debt() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64; // 1.000
    let price_0 = 150_000_000u64;   // $150.00

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // Deposit 2 JitoSOL (2 * 1e9)
    let deposit_collateral = 2_000_000_000u64;
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, deposit_collateral);
    env.user_tx(&[deposit_ix]).unwrap();

    // Draw $8.50 (8_500_000 micro-USDC)
    let debt_amount = 8_500_000u64;
    let draw_ix = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, debt_amount);
    env.user_tx(&[draw_ix]).unwrap();

    // Admin fast-forward: rate increases from 1.000 to 1.080 (+8% yield)
    // Collateral = 2 JitoSOL -> Yield in SOL = 2 * 0.08 = 0.16 SOL.
    // Yield value = 0.16 SOL * $150 = $24.00 (24_000_000 micro-USDC).
    // Since $24.00 >= $8.50 debt:
    // Flow: "Yield value at least debt? -> yes: Debt cleared (repay = debt)"
    let new_rate = 1_080_000_000u64;
    let ff_ix = ix_set_exchange_rate(&env.program_id, &env.admin.pubkey(), &env.config_pda, new_rate);
    env.admin_tx(&[ff_ix]).unwrap();

    // Execute harvest_and_repay
    let harvest_ix = ix_harvest_and_repay(&env.program_id, &env.vault_pda, &env.config_pda);
    env.user_tx(&[harvest_ix]).expect("Harvest and repay must succeed");

    let vault = env.get_vault();
    // Debt cleared!
    assert_eq!(vault.debt, 0, "Debt must be fully cleared to 0");
    // Collateral shrunk proportionally and rate reset
    assert!(vault.collateral < deposit_collateral, "Collateral must shrink to pay debt");
    assert_eq!(vault.rate, new_rate, "Vault rate must reset to new config rate");
}

#[test]
fn test_harvest_and_repay_debt_reduced_when_yield_less_than_debt() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64; // 1.000
    let price_0 = 150_000_000u64;   // $150.00

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // User deposits 2 JitoSOL ($300 value)
    let deposit_collateral = 2_000_000_000u64;
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, deposit_collateral);
    env.user_tx(&[deposit_ix]).unwrap();

    // User draws $50.00 (50_000_000 micro-USDC)
    let debt_amount = 50_000_000u64;
    let draw_ix = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, debt_amount);
    env.user_tx(&[draw_ix]).unwrap();

    // Admin fast-forward: rate goes from 1.000 to 1.050 (+5% yield)
    // Yield = 2 * 0.05 * 150 = $15.00 (15_000_000 micro-USDC)
    // Since $15.00 < $50.00 debt:
    // Flow: "Yield value at least debt? -> no: Debt reduced (repay = yield value)"
    let new_rate = 1_050_000_000u64;
    let ff_ix = ix_set_exchange_rate(&env.program_id, &env.admin.pubkey(), &env.config_pda, new_rate);
    env.admin_tx(&[ff_ix]).unwrap();

    // Execute harvest_and_repay
    let harvest_ix = ix_harvest_and_repay(&env.program_id, &env.vault_pda, &env.config_pda);
    env.user_tx(&[harvest_ix]).expect("Harvest and repay must succeed");

    let vault = env.get_vault();
    // Debt reduced by $15.00: from $50.00 -> $35.00
    assert_eq!(vault.debt, 35_000_000, "Debt must be reduced by yield value ($15.00)");
    assert!(vault.collateral < deposit_collateral, "Collateral must shrink");
    assert_eq!(vault.rate, new_rate, "Rate must be reset to new config rate");
}

#[test]
fn test_multi_epoch_harvesting() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64; // 1.000
    let price_0 = 150_000_000u64;   // $150.00

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // Deposit 2 JitoSOL ($300 value)
    let deposit_collateral = 2_000_000_000u64;
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, deposit_collateral);
    env.user_tx(&[deposit_ix]).unwrap();

    // Draw $40.00 debt
    let draw_ix = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 40_000_000);
    env.user_tx(&[draw_ix]).unwrap();

    // Epoch 1 Harvest: rate goes 1.000 -> 1.050 (+5% yield)
    // Yield = 2 * 0.05 * 150 = $15.00
    // Debt decreases by $15.00: from $40.00 -> $25.00
    let rate_1 = 1_050_000_000u64;
    let ff1 = ix_set_exchange_rate(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_1);
    env.admin_tx(&[ff1]).unwrap();

    let harvest1 = ix_harvest_and_repay(&env.program_id, &env.vault_pda, &env.config_pda);
    env.user_tx(&[harvest1]).unwrap();

    let vault_after_epoch1 = env.get_vault();
    assert_eq!(vault_after_epoch1.debt, 25_000_000, "Debt should be reduced to $25.00");
    assert_eq!(vault_after_epoch1.rate, rate_1);

    // Advance blockhash for new epoch
    env.svm.expire_blockhash();

    // Epoch 2 Harvest: rate goes 1.050 -> 1.150 (+10% on remaining collateral ~1.904 JitoSOL)
    // Yield = 1.904 * (1.150 - 1.050) / 1.0 * 150 = $28.57 > $25.00 debt
    // Debt should be completely cleared to 0!
    let rate_2 = 1_150_000_000u64;
    let ff2 = ix_set_exchange_rate(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_2);
    env.admin_tx(&[ff2]).unwrap();

    let harvest2 = ix_harvest_and_repay(&env.program_id, &env.vault_pda, &env.config_pda);
    env.user_tx(&[harvest2]).unwrap();

    let vault_after_epoch2 = env.get_vault();
    assert_eq!(vault_after_epoch2.debt, 0, "Debt should be completely wiped out to 0");
    assert_eq!(vault_after_epoch2.rate, rate_2);
    assert!(vault_after_epoch2.collateral > 0, "Base collateral remains protected");
}

#[test]
fn test_complete_end_to_end_architecture_lifecycle() {
    let mut env = DoppioTestEnv::new();

    // Section 1: Init Config
    let initial_rate = 1_075_000_000u64; // 1.075 SOL/JitoSOL
    let sol_price = 150_000_000u64;      // $150.00 per SOL
    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, initial_rate, sol_price);
    env.admin_tx(&[init_conf]).unwrap();

    // User inits vault
    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault]).unwrap();

    // User deposits 1 JitoSOL
    let deposit_ix = ix_deposit_collateral(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 1_000_000_000);
    env.user_tx(&[deposit_ix]).unwrap();

    // Section 2: Tap to pay $8.50
    let tap_ix = ix_draw_credit(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda, 8_500_000);
    env.user_tx(&[tap_ix]).unwrap();
    assert_eq!(env.get_vault().debt, 8_500_000);

    // Section 3: Admin fast-forwards rate to 1.150
    let ff_rate = 1_150_000_000u64;
    let ff_ix = ix_set_exchange_rate(&env.program_id, &env.admin.pubkey(), &env.config_pda, ff_rate);
    env.admin_tx(&[ff_ix]).unwrap();
    assert_eq!(env.get_config().rate, ff_rate);

    // Section 3: harvest_and_repay -> clears debt
    let harvest_ix = ix_harvest_and_repay(&env.program_id, &env.vault_pda, &env.config_pda);
    env.user_tx(&[harvest_ix]).unwrap();

    let final_vault = env.get_vault();
    assert_eq!(final_vault.debt, 0, "Outstanding debt must be 0 (cleared)");
    assert_eq!(final_vault.rate, ff_rate, "Vault rate must be updated to fast-forward rate");
    assert!(final_vault.collateral > 0, "Collateral remains intact minus small yield deduction");
}
