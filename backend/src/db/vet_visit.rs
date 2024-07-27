use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "vet_visits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pet_id: i32,
    pub vet_id: f32,
    pub reason: String,
    pub date: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::pet::Entity",
        from = "Column::PetId",
        to = "super::pet::Column::Id"
    )]
    Pet,
    #[sea_orm(
        belongs_to = "super::vet::Entity",
        from = "Column::VetId",
        to = "super::vet::Column::Id"
    )]
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
