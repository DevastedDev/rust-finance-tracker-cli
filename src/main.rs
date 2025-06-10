mod account;
mod command;
mod errors;
mod transaction;
mod utils;

use crate::utils::print_transactions_filter;
use crate::utils::print_transactions_list;
use account::Account;
use clearscreen;
use command::Command;
use command::parse_command;
use simply_colored::*;
use std::io;
use std::io::Write;
use transaction::Transaction;

fn main() {
    utils::long_line();

    let datafile: String = String::from("data/data.json");

    let mut transactions = match Account::init().load_transactions(&datafile) {
        Ok(account) => account,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let stats = transactions.get_stats();
    match utils::display_stats(stats.0, stats.1) {
        Ok(_) => (),
        Err(_) => println!("Unable to print the stats"),
    }
    loop {
        let mut cmd_text = String::new();
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
                if amount < 0.0 {
                    println!("{RED}Spent Amount Cannot Be Negative{RESET}");
                    continue;
                }
                let description: String = input.collect::<Vec<&str>>().join(" ");
                match transactions.add_transaction(Transaction::new(amount, description), &datafile)
                {
                    Ok(_) => println!("Added Transaction Successfully"),
                    Err(e) => println!("{e}"),
                }
            }
            Command::Remove => {
                let id = match input.next().and_then(|s| s.parse::<usize>().ok()) {
                    Some(t) => t,
                    None => return println!("invalid amount"),
                };
                println!("Removed ID: {}", id);
                match transactions.remove_transaction(&id, &datafile) {
                    Ok(_) => println!("Removed Transaction Successfully"),
                    Err(e) => println!("{e}"),
                }
            }
            Command::List => {
                let transactions = transactions.get_transactions();
                match print_transactions_list(transactions) {
                    Ok(_) => (),
                    Err(_) => println!("Unable to list the transactions"),
                }
            }
            Command::Help => {
                println!("{BLUE} Below Are the Available Commands: ");
                println!("{BOLD}{YELLOW}● add amount(integer) description , {GREEN} eg : {UNDERLINE}add 500 claude.ai subscription{RESET}");
                println!("{BOLD}{YELLOW}● remove id(integer) , {GREEN} eg : {UNDERLINE}remove 1{RESET}");
                println!("{BOLD}{YELLOW}● list all the transactions , {GREEN} eg : {UNDERLINE}list{RESET}");
                println!("{BOLD}{YELLOW}● show stats  `stats` , {GREEN} eg : {UNDERLINE}stats{RESET}");
                println!("{BOLD}{YELLOW}● filter keyword - lists all the filtered transactions containing the keyword, {GREEN} eg : {UNDERLINE}filter claude{RESET}");
                println!("{BOLD}{YELLOW}● clear tui `clear` - Clears all the terminal screen , {GREEN} eg : {UNDERLINE}clear{RESET}");
                println!("{BOLD}{YELLOW}● Exit use `exit` or `quit` , {GREEN} eg : {UNDERLINE}exit{RESET}");
            }

            Command::Stats => {
                let stats = transactions.get_stats();
                match utils::display_stats(stats.0, stats.1) {
                    Ok(_) => (),
                    Err(_) => println!("Unable to print the stats"),
                }
            }
            Command::Filter => {
                let keywords: Vec<&str> = input.collect::<Vec<&str>>();
                let found = transactions.find_transactions(keywords);
                match print_transactions_filter(found) {
                    Ok(_) => (),
                    Err(_) => println!("Unable to filter the list"),
                }
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
