#[derive(Debug)]
pub struct Wallet {
    balance: f64,
}

#[derive(Debug, PartialEq)]
pub enum WalletError {
    InsufficientFunds,
    InvalidAmount,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet { balance: 0.0 }
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), WalletError> {
        if amount <= 0.0 {
            Err(WalletError::InvalidAmount)
        } else {
            self.balance += amount;
            Ok(())
        }
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), WalletError> {
        if amount <= 0.0 {
            return Err(WalletError::InvalidAmount);
        }

        if amount > self.balance {
            return Err(WalletError::InsufficientFunds);
        }

        self.balance -= amount;
        Ok(())
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet_starts_with_zero_balance() {
        let wallet = Wallet::new();
        assert_eq!(wallet.balance(), 0.0);
    }
}
