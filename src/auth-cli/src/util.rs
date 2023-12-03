use std::{
    io::{self, Write},
    time::Instant,
};

/// Asks user input and returns it trimmed.
pub fn ask_user_input(prompt: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush()?;
    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(e);
    }
    let input = input.trim().to_string();

    Ok(input)
}

/// Prints how long successful network request took in milliseconds.
pub fn print_response_time(instant: &Instant) {
    println!("Response time: {} ms", instant.elapsed().as_millis());
}

/// Prints info that request is using gRPC.
pub fn print_using_grpc() {
    println!("Using gRPC");
}

/// Prints info that request is using REST.
pub fn print_using_rest() {
    println!("Using REST API");
}