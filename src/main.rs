pub struct Account {
    balance: f64,
}

#[derive(Debug)]
pub enum TransferError {
    Withdraw(WithdrawError),
    Deposit(DepositError),
}

#[derive(Debug)]
pub enum WithdrawError {
    Negative,
    InvalidNumber,
    InsufficientFunds,
}

#[derive(Debug)]
pub enum DepositError {
    Negative,
    InvalidNumber,
}

impl Account {
    pub fn new(balance: f64) -> Self {
        Account { balance }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) -> Result<(), DepositError> {
        match amount {
            x if x.is_sign_negative() => Err(DepositError::Negative),
            x if x.is_normal() => {
                self.balance += amount;
                Ok(())
            }
            _ => Err(DepositError::InvalidNumber),
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), WithdrawError> {
        match amount {
            x if x.is_sign_negative() => Err(WithdrawError::Negative),
            x if x > self.balance => Err(WithdrawError::InsufficientFunds),
            x if x <= self.balance && x.is_normal() => {
                self.balance -= x;
                Ok(())
            }
            _ => Err(WithdrawError::InvalidNumber),
        }
    }
}

fn main() {
    let account1 = Account::new(1000.0);
    let account2 = Account::new(500.0);

    println!("Account 1 Balance: {}", account1.get_balance());
    println!("Account 2 Balance: {}", account2.get_balance());
}
