use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    app::App,
    db::{self, auth::AuthSession, pet::PetType},
    error::BackendError,
};

pub fn router() -> Router<App> {
    Router::new().route("/pet", get(list_pets).post(create_pet))
}

pub async fn list_pets(
    auth_session: AuthSession,
    State(app): State<App>,
) -> Result<impl IntoResponse, BackendError> {
    // get the authed user
    if auth_session.user.is_none() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
    let user = auth_session.user.unwrap();

    // get pets for user
    let pets = app.db.get_pets_by_user_id(user.id).await?;
    Ok((StatusCode::OK, Json(pets)).into_response())
}

#[derive(Serialize, Deserialize)]
pub struct CreatePetRequest {
    pub name: String,
    pub kind: PetType,
    pub identification_number: Option<i64>,
}

pub async fn create_pet(
    auth_session: AuthSession,
    State(app): State<App>,
    Json(req): Json<CreatePetRequest>,
) -> Result<impl IntoResponse, BackendError> {
    // get the authed user
    if auth_session.user.is_none() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
    let user = auth_session.user.unwrap();

    let pet = db::pet::ActiveModel {
        id: ActiveValue::NotSet,
        identification_number: ActiveValue::Set(req.identification_number),
        birthday: ActiveValue::Set(Utc::now().naive_utc()),
        kind: ActiveValue::Set(req.kind),
        host_family_id: ActiveValue::Set(user.id),
        manager_id: ActiveValue::Set(user.id),
        name: ActiveValue::Set(req.name),
        previous_hf_ids: ActiveValue::Set("".to_string()),
    };

    match pet.insert(&app.db.conn).await {
        Err(err) => {
            error!("create_pet: cannot create pet: {err}");
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
        Ok(pet) => {
            info!("create_pet: pet {} ({}) created", pet.id, pet.name);
            Ok(StatusCode::OK.into_response())
        }
    }
}
