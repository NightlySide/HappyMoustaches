use axum::{routing::get, Router};
use db::DB;
use error::BackendError;
use tracing::info;

mod db;
mod error;

#[tokio::main]
async fn main() -> Result<(), BackendError> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db = DB::new("db.sqlite").await?;
    db.create_tables().await?;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
