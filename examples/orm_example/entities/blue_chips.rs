//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "blue_chips")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub symbol: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub sector: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub chart_link: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
