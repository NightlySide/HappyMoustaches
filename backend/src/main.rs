use app::App;
use error::BackendError;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app;
mod db;
mod error;
mod routes;

#[tokio::main]
async fn main() -> Result<(), BackendError> {
    // initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| {
                "backend=debug,axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug"
                    .into()
            },
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    App::new().await?.serve().await?;
    Ok(())
}
