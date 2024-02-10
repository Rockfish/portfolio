/*
create view accounts_history_normalized (
id,
run_date,
account,
activity,
action,
symbol,
security_description,
security_type,
exchange_quantity,
exchange_currency,
quantity,
currency,
price,
exchange_rate,
commission,
fees,
accrued_interest,
amount,
settlement_date
)
 */

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "account_history_normalized")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i32,
    pub run_date: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub account: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub activity: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub action: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub symbol: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub security_description: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub security_type: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub exchange_quantity: Option<Decimal>,
    #[sea_orm(column_type = "Text", nullable)]
    pub exchange_currency: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))", nullable)]
    pub quantity: Option<Decimal>,
    #[sea_orm(column_type = "Text", nullable)]
    pub currency: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub price: Option<Decimal>,
    #[sea_orm(column_type = "Text", nullable)]
    pub exchange_rate: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub commission: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub fees: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub accrued_interest: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub amount: Option<Decimal>,
    pub settlement_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
