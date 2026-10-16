#[test]
fn test_config_layout_and_size() {
    assert_eq!(program::state::Config::LEN, 64, "Config PDA must be exactly 64 bytes");
    assert_eq!(core::mem::size_of::<program::state::Config>(), 64);
    assert_eq!(core::mem::align_of::<program::state::Config>(), 8);
    assert_eq!(program::state::Config::DISCRIMINATOR, *b"dop_conf");
}

#[test]
fn test_vault_position_layout_and_size() {
    assert_eq!(program::state::VaultPosition::LEN, 72, "VaultPosition PDA must be exactly 72 bytes");
    assert_eq!(core::mem::size_of::<program::state::VaultPosition>(), 72);
    assert_eq!(core::mem::align_of::<program::state::VaultPosition>(), 8);
    assert_eq!(program::state::VaultPosition::DISCRIMINATOR, *b"dop_vpos");
}
