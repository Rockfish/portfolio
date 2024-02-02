--
-- Normalized join of Account_Positions_Overview and Account_Positions_Dividends report tables
--
create or replace view Account_Positions_Normalized as
select overview_id,
       dividend_id,
       account_number,
       CASE
           WHEN account_number = '652607171' THEN 'BrokerageLink Microsoft'
           WHEN account_number = '652770183' THEN 'BrokerageLink Oracle'
           WHEN account_number = '35004' THEN 'Oracle 401k'
           ELSE account_name
       END as account_name,
       symbol,
       description,
       quantity,
       last_price,
       last_price_change,
       current_value,
       percent_of_account,
       today_gain_loss_dollar,
       today_gain_loss_percent,
       total_gain_loss_dollar,
       total_gain_loss_percent,
       cost_basis_total,
       average_cost_basis,
       ex_date,
       amount_per_share,
       pay_date,
       yield,
       est_annual_income,
       type,
       as_of_date
from (select overview.id                                                           as overview_id,
             dividends.id                                                          as dividend_id,
             (coalesce(overview.account_number, dividends.account_number))         as account_number,
             (coalesce(overview.account_name, dividends.Account_name))             as Account_name,
             (coalesce(overview.symbol, dividends.symbol))                         as symbol,
             (coalesce(overview.description, dividends.description))               as description,
             (coalesce(overview.quantity, dividends.quantity))                     as quantity,
             (coalesce(overview.last_price, dividends.last_price))                 as last_price,
             (coalesce(overview.last_price_change, dividends.last_price_change))   as last_price_change,
             (coalesce(overview.current_value, dividends.current_value))           as current_value,
             (coalesce(overview.percent_of_account, dividends.percent_of_account)) as percent_of_account,
             today_gain_loss_dollar,
             today_gain_loss_percent,
             total_gain_loss_dollar,
             total_gain_loss_percent,
             cost_basis_total,
             average_cost_basis,
             ex_date,
             amount_per_share,
             pay_date,
             yield,
             est_annual_income,
             (coalesce(overview.type, dividends.type))                             as type,
             overview.as_of_date
      from (select * from Account_Positions_Overview where description <> 'BROKERAGELINK') overview
               full outer join (select * from Account_Positions_Dividends where description <> 'BROKERAGELINK') dividends
                   on overview.symbol = dividends.symbol
                      and overview.account_number = dividends.account_number
                      and COALESCE(overview.quantity, overview.current_value) = COALESCE(dividends.quantity, overview.current_value)
                      and overview.as_of_date = dividends.as_of_date
            ) data;

--
-- Aggregate values of assets
--
create view account_positions_normalized_aggregate as
select symbol                 as symbol,
       max(description)       as description,
       sum(quantity)          as quantity,
       max(last_price)        as last_price,
       sum(current_value)     as current_value,
       max(amount_per_share)  as amount_per_share,
       max(yield)             as yield,
       sum(est_annual_income) as est_annual_income
from Account_Positions_Normalized
-- where amount_per_share is not null
group by symbol
order by symbol;

