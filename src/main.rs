mod account;
mod transaction;
use account::Account;
use clearscreen;
use std::io;
use std::io::Write;
use std::ops::Index;
use crate::transaction::Transaction;

fn main() {
    let mut transactions = Account::init()
        .load_transactions(String::from("data/data.json"))
        .unwrap();
    loop {
        let mut cmd_text = String::from("");
        print!(">");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut cmd_text).unwrap();
        let mut input = cmd_text.trim().split_whitespace();
        let command = &input.next();

        match command {

            // Application Commands
            Some("add") => {
                let mut amount = input.next().unwrap().parse::<f64>().unwrap();
                let description: String = input.collect::<Vec<&str>>().join(" ");
                transactions.add_transaction(Transaction::new(amount,description))
            }

            Some("remove") => {
                let id = &input.next().unwrap().parse::<usize>().unwrap();
                transactions.remove_transaction(id);
                println!("Removed ID: {}",id)
            }
            Some("list") => {
                let transactions = transactions.get_transactions();
                for (i, el) in transactions.iter().enumerate() {
                    println!("{:#?}. {} Spent For {}", i+1, el.amount, el.description)
                }
            }
            // Normal Commands
            Some("clear") => {
                clearscreen::clear().expect("failed to clear screen");
            }
            Some("exit" | "quit") => {
                println!("Quitting");
                break;
            }
            // Fallbacks`
            Some(_) => {
                println!("No Command Exists");
            }
            None => {
                break;
            }
        }
    }
}
