use std::net::SocketAddr;
use tracing::info;

mod app;
mod config;


#[tokio::main]
async fn main() {
    // loda .env
    dotenvy::dotenv().ok();

    // init log
    tracing_subscriber::fmt::init();

    // construct app
    let app = app::create_app();

    // set the port
    let addr = SocketAddr::from(([0, 0, 0, 0], 9916));
    info!("listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
