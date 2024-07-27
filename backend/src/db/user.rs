use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

use crate::error::BackendError;

use super::DB;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum UserRole {
    #[sea_orm(string_value = "host_family")]
    HostFamily,
    #[sea_orm(string_value = "manager")]
    Manager,
}

#[derive(Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub address: String,
    pub role: UserRole,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .field("phone_number", &self.phone_number)
            .field("address", &self.address)
            .field("role", &self.role)
            .finish()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pet::Entity")]
    Pet,
    #[sea_orm(has_many = "super::vet::Entity")]
    Vet,
}

impl Related<super::pet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pet.def()
    }
}

impl Related<super::vet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Vet.def()
    }
}

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
