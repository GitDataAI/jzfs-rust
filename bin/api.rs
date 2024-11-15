use jzfs::api::controller::ClientController;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let server = ClientController{};
    server.run().await?;
    Ok(())
}