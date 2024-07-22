use app::App;
use error::BackendError;
use tracing::{error, info};
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
                "backend=debug,axum_login=warn,tower_sessions=warn,sqlx=warn,tower_http=warn".into()
            },
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    if let Err(err) = App::new().await?.serve().await {
        if let BackendError::TaskJoin(_) = err {
        } else {
            error!("error while stopping task: {}", err);
        }
    }
    info!("Server shut down");
    Ok(())
}
