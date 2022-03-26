use zero2prod::{run, create_listerner};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // Use TcpListener to get random port for the App to use
    let address = create_listerner();

    // Bubble up the io::Error if we failed to bind the adddress
    // otherwise call .await on our server
    run(address)?.await
}