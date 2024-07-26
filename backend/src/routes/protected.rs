use crate::{app::App, db::auth::AuthSession};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

pub fn router() -> Router<App> {
    Router::new()
        .route("/", get(protected))
        .route("/user", get(get_user))
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub email: String,
}

pub async fn get_user(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => (
            StatusCode::OK,
            Json(UserInfo {
                id: user.id,
                email: user.email.clone(),
            }),
        )
            .into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => (StatusCode::OK, user.email).into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
