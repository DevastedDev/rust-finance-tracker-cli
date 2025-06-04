mod account;
mod transaction;

use account::Account;
use std::io;
use std::io::Write;
use std::ops::Index;
use transaction::Transaction;
fn main() {
    loop {

        // We Get Transactions, After Loading the Account, we get Account struct from the function and unwrapping
        let transactions = Account::init()
            .load_transactions(String::from("data/data.json"))
            .unwrap();


        let mut cmd_text = String::from("");
        print!(">");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut cmd_text).unwrap();
        let mut input = cmd_text.trim().split_whitespace();
        let command = &input.next();

        match command {
            Some("add") => {
                let amount = &input.next().unwrap();
                let description: &str = input.next().unwrap();
                println!("{} for {}", amount, description)
            }
            Some("remove") => {
                let amount = &input.next().unwrap();
                let description: &str = input.next().unwrap();
                println!("{} for {}", amount, description)
            }
            Some("clear") => {
                println!("Quitting");
                break;
            }
            Some("list") => {

                /** List Transactions ***/
                let transactions = transactions.get_transactions();
                for (i, el) in transactions.iter().enumerate() {
                    println!("{:#?}. {} Spent For {}", i, el.amount, el.description)
                }
            }
            Some(_) => {
                println!("No Command Exists");
            }
            None => {
                break;
            }
        }
    }
}
