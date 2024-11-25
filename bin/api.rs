use jzfs_router::starter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    starter::run().await?;
    Ok(())
}

