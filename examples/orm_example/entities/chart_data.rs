//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "chart_data")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub symbol: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub chart: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub update_date: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub quality_rank: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub shares_outstanding: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub institution_own: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub div_paid_since: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub profit_margin: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub ttm_earnings: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub pe_ratio: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub book_value: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub div_payout: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub current_price: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub current_yield: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub overvalue_price: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub overvalue_pts_up: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub overvalue_yield: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub overvalue_percent_up: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub undervalue_price: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub undervalue_pts_dn: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub undervalue_yield: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub undervalue_percent_dn: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
