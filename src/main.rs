mod account;
use account::Account;

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
