#[test]
fn test_collateral_value_calculation() {
    let val = program::math::compute_collateral_value_usdc(1_000_000_000, 1_000_000_000, 150_000_000).unwrap();
    assert_eq!(val, 150_000_000, "1 JitoSOL at $150 should equal $150.00 USDC");

    let val2 = program::math::compute_collateral_value_usdc(2_000_000_000, 1_075_000_000, 200_000_000).unwrap();
    assert_eq!(val2, 430_000_000);
}

#[test]
fn test_max_borrow_70_percent_ltv() {
    let val = 150_000_000u128;
    let max_borrow = program::math::compute_max_borrow_usdc(val);
    assert_eq!(max_borrow, 105_000_000);

    assert_eq!(program::math::compute_max_borrow_usdc(1_000_000_000), 700_000_000);
}

#[test]
fn test_yield_and_shares_calculation() {
    let (yield_val, shares) = program::math::compute_yield_and_shares(
        10_000_000_000,
        1_000_000_000,
        1_100_000_000,
        100_000_000,
    ).unwrap();

    assert_eq!(yield_val, 100_000_000);
    assert_eq!(shares, 10_000_000_000 * 100_000_000 / 1_100_000_000);
}

#[test]
fn test_zero_yield_when_rate_unchanged_or_lower() {
    let (yield_val, shares) = program::math::compute_yield_and_shares(
        10_000_000_000,
        1_000_000_000,
        1_000_000_000,
        150_000_000,
    ).unwrap();
    assert_eq!(yield_val, 0);
    assert_eq!(shares, 0);

    let (yield_val2, shares2) = program::math::compute_yield_and_shares(
        10_000_000_000,
        1_050_000_000,
        1_000_000_000,
        150_000_000,
    ).unwrap();
    assert_eq!(yield_val2, 0);
    assert_eq!(shares2, 0);
}
