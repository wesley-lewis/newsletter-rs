#[cfg(test)]
mod tests;

use newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let address = "localhost:3000";
    run(address)?.await
}

