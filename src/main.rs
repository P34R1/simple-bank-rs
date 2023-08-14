pub struct Account {
    balance: f64,
}

impl Account {
    pub fn new(balance: f64) -> Self {
        Account { balance }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let account1 = Account::new(1000.0);
    let account2 = Account::new(500.0);

    println!("Account 1 Balance: {}", account1.get_balance());
    println!("Account 2 Balance: {}", account2.get_balance());
}
