use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pet_weights")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pet_id: i32,
    pub value: f32,
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
}

impl Related<super::pet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
