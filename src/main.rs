use std::io;

fn main() -> io::Result<()>{
    println!("Enter Your Input String:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
