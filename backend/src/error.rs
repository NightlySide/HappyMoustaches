use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
}
