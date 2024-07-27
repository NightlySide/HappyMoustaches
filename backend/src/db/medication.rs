use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum MedicationKind {
    #[sea_orm(string_value = "vaccine")]
    Vaccine,
    #[sea_orm(string_value = "anti_flea")]
    AntiFlea,
    #[sea_orm(string_value = "deworming")]
    Deworming,
    #[sea_orm(string_value = "other")]
    Other,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "medication")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pet_id: i32,
    pub name: f32,
    pub kind: MedicationKind,
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
