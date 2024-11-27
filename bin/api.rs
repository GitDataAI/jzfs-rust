#[tokio::main]
async fn main() {
    app::boot::api::run().await.unwrap();
}