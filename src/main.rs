use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use state::AppState;

mod app;
mod config;
mod state;
mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    // loda .env
    dotenvy::dotenv().ok();

    // init log
    tracing_subscriber::fmt::init();

    // read db url
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create link pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    info!("Database connected successfully");

    // construct app
    let jwt_secret = 
        std::env::var("jwt_secret").expect("JWT_SECRET must be set");
    let state = AppState{
        db: pool,
        jwt_secret,
    };
    let app = app::create_app(state);

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
