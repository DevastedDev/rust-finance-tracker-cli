use crate::transaction::Transaction;

pub fn long_line() {
    println!("——————————————————————————————————————————————————————————————————————————")
}

pub fn display_stats(total: f64, n: usize) {
    println!("┌─── 💳 Spending Report ──┐");
    println!("│ This Month: ${}     │", total);
    println!("│ Total Purchases: {}      │", n);
    println!("└─────────────────────────┘");
}

pub fn print_transactions_list(transactions: &Vec<Transaction>) {
    for (i, el) in transactions.iter().enumerate() {
        println!("{:#?}. {} Spent For {}", i + 1, el.amount, el.description)
    }
}
pub fn print_transactions_filter(transactions: Vec<&Transaction>) {
    for (i, el) in transactions.iter().enumerate() {
        println!("{:#?}. {} Spent For {}", i + 1, el.amount, el.description)
    }
}
