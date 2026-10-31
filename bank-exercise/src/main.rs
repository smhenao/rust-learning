use std::io::{self, Write};

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // This function takes ownership of 'owner'
    fn new(owner: String) -> Self {
        Self {
            owner,
            balance: 0.0,
        }
    }

    // self has to be mutable, since we modify values inside the struct
    fn deposit(&mut self, amount: f64) {
        if amount <= 0.0 {
            panic!("Can't deposit negative or null amounts");
        }
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= 0.0 {
            panic!("Can't withdraw negative or null amounts");
        } else if amount > self.balance {
            panic!("Can't withdraw more than your current balance");
        }
        self.balance -= amount;
    }
}

// Creates a bank account of the struct type
// Prompting the user for their name and initial balance
fn create_account() -> BankAccount {
    print!("Insert the name of your new bank account: ");
    io::stdout().flush().unwrap();

    let mut acc_name = String::new();
    io::stdin().read_line(&mut acc_name).unwrap();
    acc_name = acc_name.trim().to_string();

    // The actual bank account
    let mut acc = BankAccount::new(acc_name);

    print!("Insert your initial balance: ");
    io::stdout().flush().unwrap();

    let mut balance = String::new();
    io::stdin().read_line(&mut balance).unwrap();

    acc.balance = balance.trim().parse().expect("Please insert a valid balance");

    // Return the complete account with user and balance
    acc
}

fn main() {
    println!("Welcome to the bank.");

    let mut acc = create_account();

    // Print the state of the account as the user just created it
    println!("{acc:#?}");

    println!("Welcome, {}, please input the number of your operation:
1. Deposit
2. Withdraw", acc.owner);

    print!("Input: ");
    io::stdout().flush().unwrap();

    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();

    let op: u8 = op.trim().parse().expect("Please insert a valid number");

    match op {
        1 => {
            print!("Amount to deposit: ");
            io::stdout().flush().unwrap();

            let mut deposit = String::new();
            io::stdin().read_line(&mut deposit).unwrap();

            let deposit: f64 = deposit.trim().parse()
                .expect("Please enter a valid deposit amount");

            acc.deposit(deposit);
        }
        2 => {
            print!("Amount to withdraw: ");
            io::stdout().flush().unwrap();

            let mut withdraw = String::new();
            io::stdin().read_line(&mut withdraw).unwrap();

            let withdraw: f64 = withdraw.trim().parse()
                .expect("Please enter a valid withdraw amount");

            acc.withdraw(withdraw);
        }
        _ => {
            println!("Please enter a valid operation.");
        }
    }

    // Print the state of the account after the operation
    println!("{acc:#?}");
}
