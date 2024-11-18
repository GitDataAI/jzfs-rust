use jzfs::api::controller::ClientController;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    jzfs::db::create::InitDatabase::init().await;
    let server = ClientController{};
    server.run().await?;
    Ok(())
}