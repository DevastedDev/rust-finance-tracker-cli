mod account;
mod command;
mod transaction;
mod utils;

use crate::utils::print_transactions_filter;
use crate::utils::print_transactions_list;
use account::Account;
use clearscreen;
use command::Command;
use command::parse_command;
use std::io;
use std::io::Write;
use transaction::Transaction;

fn main() {
    utils::long_line();
    let datafile: String = String::from("data/data.json");

    let mut transactions = match Account::init().load_transactions(&datafile) {
        Ok(account) => account,
        Err(val) => {
            println!("Error :{}", val);
            return;
        }
    };

    let stats = transactions.get_stats();
    utils::display_stats(stats.0, stats.1);

    loop {
        let mut cmd_text = String::from("");
        print!(">");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut cmd_text).unwrap();
        let mut input = cmd_text.trim().split_whitespace();
        let command = match input.next() {
            Some(val) => val,
            None => {
                println!("Nothing Provided");
                break;
            }
        };

        let cmd = parse_command(&command);
        match cmd {
            Command::Add => {
                let amount = match input.next().and_then(|s| s.parse::<f64>().ok()) {
                    Some(t) => t,
                    None => return println!("invalid amount"),
                };
                let description: String = input.collect::<Vec<&str>>().join(" ");
                transactions.add_transaction(Transaction::new(amount, description), &datafile)
            }
            Command::Remove => {
                let id = match input.next().and_then(|s| s.parse::<usize>().ok()) {
                    Some(t) => t,
                    None => return println!("invalid amount"),
                };
                transactions.remove_transaction(&id, &datafile);
                println!("Removed ID: {}", id)
            }
            Command::List => {
                let transactions = transactions.get_transactions();
                print_transactions_list(transactions);
            }
            Command::Stats => {
                let stats = transactions.get_stats();
                utils::display_stats(stats.0, stats.1);
            }
            Command::Filter => {
                let keywords: Vec<&str> = input.collect::<Vec<&str>>();
                let found = transactions.find_transactions(keywords);
                print_transactions_filter(found);
            }
            Command::Clear => {
                clearscreen::clear().expect("failed to clear screen");
            }
            Command::Exit => {
                println!("Quitting");
                break;
            }
            Command::Unknown => {
                println!("No Command Exists");
            }
        };
    }
}
