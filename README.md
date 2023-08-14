# Simple-Bank-rs
This project serves as a simple starter project to help you learn Rust programming. It's a basic implementation of object-oriented programming logic for creating a basic account bank system. Though it lacks complexity, it offers a practical way to practice Rust fundamentals.

## Features
- Account creation and management.
- Checking account balances.
- Transferring funds between accounts.

## Getting Started
To run this project on your local machine, follow these steps:

1. **Clone the Repository:**

   Start by cloning this repository to your local machine using the following command:
   ```bash
   git clone P34R1/simple-bank-rs
   ```

2. **Navigate to Project Directory:**

   Change your current directory to the project directory:
   ```bash
   cd simple-bank-rs
   ```

3. **Run**:

   To run the project, use the following command:
   ```bash
   cargo run
   ```

   The output will display the balances of two accounts, perform a fund transfer, and show the updated balances.

## Usage

The project consists of the following main components:
- `account`: This module defines the `Account` struct and its associated methods.

  The `main` function in the `src/main.rs` file demonstrates the usage of the `Account` struct and its methods.

It then will
- Create two `Account` instances with initial balances.
- Print the initial balances of both accounts.
- Attempt to transfer funds between the accounts and handle errors if any.
- Print the updated balances after the transfer.

Feel free to modify the initial balances, perform multiple transfers, or experiment with the code to further practice Rust programming concepts.

## Contribution
While this project is designed as a basic learning exercise, contributions are welcome. If you have improvements, bug fixes, or additional features to suggest, feel free to fork the repository, make your changes, and submit a pull request.

## License
This project is licensed under the [MIT License](LICENSE). You're free to use, modify, and distribute the code as per the terms of the license.

Have fun exploring Rust and learning about basic object-oriented programming concepts with this simple account bank system project! Happy coding!
