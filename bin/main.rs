use rand::distributions::Alphanumeric;
use rand::Rng;

#[tokio::main]
async fn main() {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(96)
        .map(char::from)
        .collect();
    println!("generated token: {}", token);
}
