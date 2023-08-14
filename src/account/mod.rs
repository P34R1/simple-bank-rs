pub struct Account {
    balance: f64,
}

mod types;
use types::{DepositError, TransferError, WithdrawError};

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
