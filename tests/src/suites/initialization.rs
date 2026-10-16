use solana_sdk::signature::{Keypair, Signer};
use crate::common::*;

#[test]
fn test_initialize_accounts_flow() {
    let mut env = DoppioTestEnv::new();

    // 1. Admin initializes Config PDA
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64;

    let init_conf_ix = ix_initialize_config(
        &env.program_id,
        &env.admin.pubkey(),
        &env.config_pda,
        rate_0,
        price_0,
    );
    env.admin_tx(&[init_conf_ix]).expect("Failed to initialize config");

    let config = env.get_config();
    assert_eq!(config.admin, env.admin.pubkey());
    assert_eq!(config.rate, rate_0);
    assert_eq!(config.price, price_0);

    // 2. User initializes VaultPosition PDA
    let init_vault_ix = ix_initialize_vault(
        &env.program_id,
        &env.user.pubkey(),
        &env.vault_pda,
        &env.config_pda,
    );
    env.user_tx(&[init_vault_ix]).expect("Failed to initialize vault");

    let vault = env.get_vault();
    assert_eq!(vault.owner, env.user.pubkey());
    assert_eq!(vault.collateral, 0);
    assert_eq!(vault.debt, 0);
    assert_eq!(vault.rate, rate_0);
}

#[test]
fn test_admin_fast_forward_access_control() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64;

    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf]).unwrap();

    // 1. Non-admin calls fast-forward -> Reject (not admin)
    let imposter = Keypair::new();
    env.svm.airdrop(&imposter.pubkey(), 1_000_000_000).unwrap();

    let unauthorized_ff = ix_set_exchange_rate(
        &env.program_id,
        &imposter.pubkey(),
        &env.config_pda,
        1_080_000_000,
    );
    let res = env.custom_tx(&[unauthorized_ff], &[&imposter]);
    assert!(res.is_err(), "Must reject: caller is not admin");

    // 2. Admin calls fast-forward -> Config rate updated
    let new_rate = 1_082_000_000u64; // 1.082
    let admin_ff = ix_set_exchange_rate(
        &env.program_id,
        &env.admin.pubkey(),
        &env.config_pda,
        new_rate,
    );
    env.admin_tx(&[admin_ff]).expect("Admin fast-forward must succeed");

    let config = env.get_config();
    assert_eq!(config.rate, new_rate, "Config rate must be updated");
}

#[test]
fn test_double_initialization_rejected() {
    let mut env = DoppioTestEnv::new();
    let rate_0 = 1_000_000_000u64;
    let price_0 = 150_000_000u64;

    // Init config first time
    let init_conf = ix_initialize_config(&env.program_id, &env.admin.pubkey(), &env.config_pda, rate_0, price_0);
    env.admin_tx(&[init_conf.clone()]).unwrap();

    // Attempt to re-initialize config second time -> must fail
    let res_config = env.admin_tx(&[init_conf]);
    assert!(res_config.is_err(), "Double-initialization of Config must fail");

    // Init vault first time
    let init_vault = ix_initialize_vault(&env.program_id, &env.user.pubkey(), &env.vault_pda, &env.config_pda);
    env.user_tx(&[init_vault.clone()]).unwrap();

    // Attempt to re-initialize vault second time -> must fail
    let res_vault = env.user_tx(&[init_vault]);
    assert!(res_vault.is_err(), "Double-initialization of Vault must fail");
}
