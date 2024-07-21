use crate::db::auth::AuthSession;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn router() -> Router<()> {
    Router::new().route("/", get(protected))
}

pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => (StatusCode::OK, user.email).into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
