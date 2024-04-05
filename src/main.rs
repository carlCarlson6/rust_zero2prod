use rust_zero2prod::run;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run()?.await
}
