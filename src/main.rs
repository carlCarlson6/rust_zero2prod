use rust_zero2prod::run;
use std::{io::Error, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello world! - starting server");
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind port");
    println!(
        "app will run on port {}",
        listener.local_addr().unwrap().port()
    );
    run(listener)?.await
}
