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
