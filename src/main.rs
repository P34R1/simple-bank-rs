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

    pub fn get_balance(&self) -> String {
        format!("{:.2}", self.balance)
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

    pub fn transfer_funds_to(
        &mut self,
        receiver: &mut Account,
        amount: f64,
    ) -> Result<(), TransferError> {
        if let Err(e) = self.withdraw(amount) {
            return Err(TransferError::Withdraw(e));
        }
        if let Err(e) = receiver.deposit(amount) {
            return Err(TransferError::Deposit(e));
        }
        Ok(())
    }
}

fn main() {
    let mut account1 = Account::new(1000.0);
    let mut account2 = Account::new(500.0);

    println!("Account 1 Balance: {}", account1.get_balance());
    println!("Account 2 Balance: {}", account2.get_balance());

    if let Err(err) = account1.transfer_funds_to(&mut account2, 3.00) {
        println!("Error: {:?}", err)
    }

    println!(
        "Account 1 Balance after transfer: {}",
        account1.get_balance()
    );
    println!(
        "Account 2 Balance after transfer: {}",
        account2.get_balance()
    );
}
