pub enum Command {
    Add,
    Remove,
    List,
    Exit,
    Clear,
    Stats,
    Filter,
    Unknown,
}

pub fn parse_command(command: &str) -> Command {
    match command {
        "add" => Command::Add,
        "remove" => Command::Remove,
        "list" => Command::List,
        "stats" => Command::Stats,
        "filter" => Command::Filter,
        "exit" | "quit" => Command::Exit,
        "clear" => Command::Clear,
        _ => Command::Unknown,
    }
}
