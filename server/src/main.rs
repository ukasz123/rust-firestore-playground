mod server;

#[tokio::main]
async fn main() {
    server::run_http_server().await;
}
