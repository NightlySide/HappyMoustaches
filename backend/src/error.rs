use axum::http::StatusCode;
use axum_thiserror::ErrorStatus;
use thiserror::Error;

#[derive(Error, Debug, ErrorStatus)]
pub enum BackendError {
    #[error("Database error: {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("TowerSession sqlX error: {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    TowerSessionSqlxErr(#[from] tower_sessions_sqlx_store::sqlx::Error),

    #[error("Tracing subscriber init error: {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    TracingSubscriberInitErr(#[from] tracing_subscriber::util::TryInitError),

    #[error(transparent)]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    TaskJoin(#[from] tokio::task::JoinError),

    #[error("Std io error: {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    StdIoErr(#[from] std::io::Error),

    #[error("No user found")]
    #[status(StatusCode::NOT_FOUND)]
    NoUser,
}
