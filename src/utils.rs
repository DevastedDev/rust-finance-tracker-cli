use crate::errors::CommonError;
use crate::transaction::Transaction;

pub fn long_line() {
    println!("——————————————————————————————————————————————————————————————————————————")
}

pub fn display_stats(total: f64, n: usize) -> Result<bool, CommonError> {
    println!("┌─── 💳 Spending Report ──┐");
    println!("│ This Month: ${}     │", total);
    println!("│ Total Purchases: {}      │", n);
    println!("└─────────────────────────┘");
    Ok(true)
}

pub fn print_transactions_list(transactions: &Vec<Transaction>) -> Result<bool, CommonError> {
    for (i, el) in transactions.iter().enumerate() {
        println!("{:#?}. {} Spent For {}", i + 1, el.amount, el.description)
    }
    Ok(true)
}
pub fn print_transactions_filter(transactions: Vec<&Transaction>) -> Result<bool, CommonError> {
    for (i, el) in transactions.iter().enumerate() {
        println!("{:#?}. {} Spent For {}", i + 1, el.amount, el.description)
    }
    Ok(true)
}
