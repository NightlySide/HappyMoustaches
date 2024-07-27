use std::error::Error;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue};
use tracing::{error, info, warn};

use crate::{
    app::App,
    db::auth::{AuthSession, Credentials},
    db::user,
    error::BackendError,
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/login", post(post_login))
        .route("/logout", get(get_logout))
        .route("/register", post(post_register))
}

pub async fn post_login(
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    // Trying to auth the user
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // invalid credentials
            warn!("post_login: invalid credentials");
            return StatusCode::UNAUTHORIZED.into_response();
        }
        Err(err) => {
            // we already know this is of type BackendError so let's downcast it!
            let backend_error = err
                .source()
                .unwrap()
                .downcast_ref::<BackendError>()
                .unwrap();

            match backend_error {
                &BackendError::NoUser => {
                    return (StatusCode::UNAUTHORIZED, "User not found").into_response();
                }
                _ => {
                    error!("post_login: {}", backend_error);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            }
        }
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    info!("post_login: Successfully logged in as {}", user.email);
    StatusCode::OK.into_response()
}

pub async fn post_register(
    State(app): State<App>,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    // check if user does not already exist
    match app.db.get_user_by_email(&creds.email).await {
        Ok(Some(_)) => {
            return (StatusCode::CONFLICT, "User already exists").into_response();
        }
        Ok(None) => (),
        Err(err) => {
            error!("post_register: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    // create the user
    let hashed_password = password_auth::generate_hash(creds.password);
    let user = user::ActiveModel {
        id: ActiveValue::NotSet,
        email: ActiveValue::Set(creds.email),
        password: ActiveValue::Set(hashed_password),
        phone_number: ActiveValue::Set("".to_string()),
        address: ActiveValue::Set("".to_string()),
        role: ActiveValue::Set(user::UserRole::HostFamily),
    };
    match user.insert(&app.db.conn).await {
        Err(err) => {
            error!("post_register: cannot create user: {err}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Ok(user) => {
            info!("post_register: user {} ({}) created", user.id, user.email);
            StatusCode::OK.into_response()
        }
    }
}

pub async fn get_logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
