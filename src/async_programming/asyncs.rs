use tokio::time::{Duration, sleep};

async fn say_hello() {
    let message = message().await;
    println!("{:?}", message);
}

async fn message() -> String {
    sleep(Duration::from_secs(5)).await;
    "Hello Bro".to_string()
}

#[tokio::main]
pub async fn hello() {
    say_hello().await;
}
