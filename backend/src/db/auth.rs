use axum::async_trait;
use axum_login::UserId;
use axum_login::{AuthUser, AuthnBackend};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::Deserialize;

use crate::error::BackendError;

use super::user::Entity as User;
use super::user::Model as UserModel;
use super::DB;

impl AuthUser for UserModel {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthBackend {
    db: DB,
}

impl AuthBackend {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = UserModel;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = User::find()
            .filter(super::user::Column::Email.contains(creds.email))
            .one(&self.db.conn)
            .await?;

        // Verifying the password is blocking and potentially slow, so we'll do so via
        // `spawn_blocking`.
        tokio::task::spawn_blocking(|| {
            // We're using password-based authentication--this works by comparing our form
            // input with an argon2 password hash.
            Ok(user.filter(|user| {
                password_auth::verify_password(creds.password, &user.password).is_ok()
            }))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<UserModel> = User::find_by_id(*user_id).one(&self.db.conn).await?;

        Ok(user)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<AuthBackend>;
