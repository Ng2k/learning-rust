use hello_world::greetings::say_hello;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = std::env::args().nth(1).unwrap_or("World".to_string());
    let greeting = say_hello(&name)?;
    println!("{}", greeting);
    Ok(())
}
