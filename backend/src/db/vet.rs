use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

#[derive(Clone, PartialEq, Eq, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "vets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub added_by_hf: i32,
    pub name: String,
    pub address: String,
    pub phone_number: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::vet_visit::Entity")]
    VetVisit,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AddedByHf",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::vet_visit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VetVisit.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
