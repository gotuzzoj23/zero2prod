use zero2prod::{run, create_listener};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // Use TcpListener to get random port for the App to use
    let listener = create_listener();
    // Bubble up the io::Error if we failed to bind the adddress
    // otherwise call .await on our server
    run(listener)?.await
}