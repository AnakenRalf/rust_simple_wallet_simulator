use std::io::{self, Write};
use wallet_sim::wallet::{Wallet, WalletError};

fn main() {
    let mut wallet = Wallet::new();

    println!("Hello, this is wallet simulator!");
    println!("Possible commands: deposit, withdraw, balance, exit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() == 0 {
            continue;
        }

        match parts[0] {
            "deposit" => {
                if parts.len() < 2 {
                    println!("Usage: deposit <amount>");
                    continue;
                }
                let amount: f64 = parts[1].parse().unwrap();
                match wallet.deposit(amount) {
                    Ok(()) => println!("Deposit successful!"),
                    Err(WalletError::InvalidAmount) => println!("Invalid amount"),
                    Err(WalletError::InsufficientFunds) => println!("Insufficient funds"),
                }
            }
            "withdraw" => {
                if parts.len() < 2 {
                    println!("Usage: withdraw <amount>");
                    continue;
                }
                let amount: f64 = parts[1].parse().unwrap();
                match wallet.withdraw(amount) {
                    Ok(()) => println!("Withdrawal successful!"),
                    Err(WalletError::InvalidAmount) => println!("Invalid amount"),
                    Err(WalletError::InsufficientFunds) => println!("Insufficient funds"),
                }
            }
            "balance" => println!("Balance: {}", wallet.balance()),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
