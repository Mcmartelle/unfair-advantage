#![cfg(test)]

#[test]
fn always_true() {
    use unfair_advantage_lib::utils::returns_true;

    assert!(returns_true());
}
