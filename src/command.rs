pub enum Command {
    Add,
    Remove,
    List,
    Exit,
    Clear,
    Stats,
    Filter,
    Help,
    Unknown,
}

pub fn parse_command(command: &str) -> Command {
    match command {
        "add" => Command::Add,
        "remove" => Command::Remove,
        "list" => Command::List,
        "stats" => Command::Stats,
        "filter" => Command::Filter,
        "help" => Command::Help,
        "exit" | "quit" => Command::Exit,
        "clear" => Command::Clear,
        _ => Command::Unknown,
    }
}
