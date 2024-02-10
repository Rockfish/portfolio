//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "account_history_source_file")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub filename: Option<String>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::account_history::Entity")]
    AccountHistory,
}

impl Related<super::account_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
