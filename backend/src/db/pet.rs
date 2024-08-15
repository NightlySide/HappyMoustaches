use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::error::BackendError;

use super::DB;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum PetType {
    #[sea_orm(string_value = "dog")]
    Dog,
    #[sea_orm(string_value = "cat")]
    Cat,
    #[sea_orm(string_value = "kitten")]
    Kitten,
    #[sea_orm(string_value = "other")]
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "pets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub identification_number: Option<i64>,
    pub host_family_id: i32,
    pub manager_id: i32,
    pub name: String,
    pub kind: PetType,
    pub birthday: DateTime,
    // TODO: use array
    pub previous_hf_ids: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::HostFamilyId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::pet_weight::Entity")]
    PetWeight,
    #[sea_orm(has_many = "super::medication::Entity")]
    Medication,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::pet_weight::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PetWeight.def()
    }
}

impl Related<super::medication::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Medication.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl DB {
    pub async fn get_pets_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Vec<self::Model>, BackendError> {
        // first get the user
        let user = super::user::Entity::find()
            .filter(super::user::Column::Id.eq(user_id))
            .one(&self.conn)
            .await?
            .unwrap();

        // then from the user get all the related pets
        let pets = user.find_related(self::Entity).all(&self.conn).await?;

        Ok(pets)
    }
}
