use wallet_sim::wallet::{Wallet, WalletError};

#[test]
fn test_new_wallet_starts_with_zero_balance() {
    let wallet = Wallet::new();
    assert_eq!(wallet.balance(), 0.0);
}

#[test]
fn test_deposit_increases_balance() {
    let mut wallet = Wallet::new();
    wallet.deposit(100.0).unwrap();
    assert_eq!(wallet.balance(), 100.0);
}

#[test]
fn test_withdraw_decreases_balance() {
    let mut wallet = Wallet::new();
    wallet.deposit(200.0).unwrap();
    wallet.withdraw(50.0).unwrap();
    assert_eq!(wallet.balance(), 150.0);
}

#[test]
fn test_withdraw_more_than_balance_fails() {
    let mut wallet = Wallet::new();
    wallet.deposit(50.0).unwrap();
    let result = wallet.withdraw(100.0);
    assert_eq!(result, Err(WalletError::InsufficientFunds));
    assert_eq!(wallet.balance(), 50.0);
}

#[test]
fn test_deposit_negative_amount_fails() {
    let mut wallet = Wallet::new();
    let result = wallet.deposit(-10.0);
    assert_eq!(result, Err(WalletError::InvalidAmount));
    assert_eq!(wallet.balance(), 0.0);
}

#[test]
fn test_withdraw_negative_amount_fails() {
    let mut wallet = Wallet::new();
    let result = wallet.withdraw(-20.0);
    assert_eq!(result, Err(WalletError::InvalidAmount));
    assert_eq!(wallet.balance(), 0.0);
}
