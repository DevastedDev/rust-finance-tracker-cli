mod account;
mod transaction;
mod utils;

use crate::transaction::Transaction;
use account::Account;
use clearscreen;
use std::io;
use std::io::Write;
fn main() {

    utils::long_line();
    let datafile: String = String::from("data/data.json");

    // GET Stats and Initialize the account data
    let mut transactions = Account::init().load_transactions(&datafile).unwrap();
    let stats = transactions.get_stats();
    utils::display_stats(stats.0, stats.1);

    loop {
        let mut cmd_text = String::from("");
        print!(">");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut cmd_text).unwrap();
        let mut input = cmd_text.trim().split_whitespace();
        let command = &input.next();

        match command {

            Some("add") => {
                let amount = input.next().unwrap().parse::<f64>().unwrap();
                let description: String = input.collect::<Vec<&str>>().join(" ");
                transactions.add_transaction(Transaction::new(amount, description), &datafile)
            }

            Some("remove") => {
                let id = &input.next().unwrap().parse::<usize>().unwrap();
                transactions.remove_transaction(id, &datafile);
                println!("Removed ID: {}", id)
            }
            Some("list") => {
                let transactions = transactions.get_transactions();
                for (i, el) in transactions.iter().enumerate() {
                    println!("{:#?}. {} Spent For {}", i + 1, el.amount, el.description)
                }
            }
            Some("stats") => {
                utils::long_line();
                let stats = transactions.get_stats();
                utils::display_stats(stats.0, stats.1);
                utils::long_line();
            }

            Some("clear") => {
                clearscreen::clear().expect("failed to clear screen");
            }
            Some("exit" | "quit") => {
                println!("Quitting");
                break;
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
