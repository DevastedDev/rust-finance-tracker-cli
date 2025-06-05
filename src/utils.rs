pub fn long_line(){
    println!("——————————————————————————————————————————————————————————————————————————")
}


pub fn display_stats(total: f64, n: usize) {
    println!("┌─── 💳 Spending Report ──┐");
    println!("│ This Month: ${}     │", total);
    println!("│ Total Purchases: {}      │", n);
    println!("└─────────────────────────┘");
}