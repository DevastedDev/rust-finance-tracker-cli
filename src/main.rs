mod account;
mod command;
mod transaction;
mod utils;

use command::parse_command;
use transaction::Transaction;
use account::Account;
use clearscreen;
use command::Command;
use std::io;
use std::io::Write;


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
                let amount = input.next().unwrap().parse::<f64>().unwrap();
                let description: String = input.collect::<Vec<&str>>().join(" ");
                transactions.add_transaction(Transaction::new(amount, description), &datafile)
            }
            Command::Remove => {
                let id = &input.next().unwrap().parse::<usize>().unwrap();
                transactions.remove_transaction(id, &datafile);
                println!("Removed ID: {}", id)
            }
            Command::List => {
                let transactions = transactions.get_transactions();
                for (i, el) in transactions.iter().enumerate() {
                    println!("{:#?}. {} Spent For {}", i + 1, el.amount, el.description)
                }
            }
            Command::Stats => {
                utils::long_line();
                let stats = transactions.get_stats();
                utils::display_stats(stats.0, stats.1);
                utils::long_line();
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
