use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

use crate::error::BackendError;

use super::DB;

#[derive(Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    pub password: String,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .finish()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl DB {
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<Model>, BackendError> {
        self::Entity::find()
            .filter(self::Column::Email.eq(email))
            .one(&self.conn)
            .await
            .map_err(BackendError::DbErr)
    }
}
