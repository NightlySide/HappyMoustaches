use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use tracing::{debug, info, warn};

use crate::db::auth::{AuthSession, Credentials};

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(post_login))
        .route("/logout", get(get_logout))
        .route("/register", get(post_register))
}

pub async fn post_login(
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    debug!("Trying creds: {}|{}", creds.email, creds.password);

    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // invalid credentials
            let mut login_url = "/login".to_string();
            if let Some(next) = creds.next {
                login_url = format!("{}?next={}", login_url, next);
            };
            warn!("invalid credentials");

            return Redirect::to(&login_url).into_response();
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    info!("Successfully logged in as {}", user.email);

    if let Some(ref next) = creds.next {
        Redirect::to(next)
    } else {
        Redirect::to("/")
    }
    .into_response()
}

pub async fn post_register(
    mut _auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    // check if user does not already exist

    // create the user
    info!("Successfully logged in as");

    if let Some(ref next) = creds.next {
        Redirect::to(next)
    } else {
        Redirect::to("/")
    }
    .into_response()
}

pub async fn get_logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
