#[tokio::main]
async fn main() {
    if let Err(e) = api::init().await {
        tracing::info!("api init error:{}", e);
    }
}
