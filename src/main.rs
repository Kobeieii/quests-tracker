use std::sync::Arc;

use quests_tracker::{
    config::config_loader,
    infrastructure::{
        axum_http::http_server::start, postgres::postgres_connection::establish_connection,
    },
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(err) => {
            error!("Failed to load ENV: {}", err);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded");

    let postgres_pool = match establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(err) => {
            error!("Failed to establish connection {}", err);
            std::process::exit(1)
        }
    };

    info!("Connect database successfully");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start the server");
}
