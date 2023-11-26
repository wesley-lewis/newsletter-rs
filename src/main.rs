#[cfg(test)]
mod tests;

use std::net::TcpListener;

use newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let address = "localhost:3000";
    let listener = TcpListener::bind(address).unwrap();
    run(listener)?.await
}
