use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),

    #[error("TowerSession sqlX error: {0}")]
    TowerSessionSqlxErr(#[from] tower_sessions_sqlx_store::sqlx::Error),

    #[error("Tracing subscriber init error: {0}")]
    TracingSubscriberInitErr(#[from] tracing_subscriber::util::TryInitError),

    #[error(transparent)]
    TaskJoin(#[from] tokio::task::JoinError),

    #[error("Std io error: {0}")]
    StdIoErr(#[from] std::io::Error),

    #[error("No user found")]
    NoUser,
}
