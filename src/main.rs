use std::{env, sync::Arc};

use anyhow::Ok;
use axum::{
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use dotenv::dotenv;
use melding::routes;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

pub mod domain;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(|| async { StatusCode::OK }))
        .route(
            "/melding",
            post(routes::mattilsynmelding::opprett_mattilsynmelding),
        )
        .route("/meldinger", get(routes::mattilsynmelding::get_meldinger))
        .route(
            "/meldinger/:id",
            delete(routes::mattilsynmelding::slett_melding),
        )
        .with_state(Arc::new(pool));

    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
