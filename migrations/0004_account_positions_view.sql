create or replace view Account_Positions as
select overview_id,
       dividend_id,
       account_number,
       account_name,
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
--        calculated_yield,
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
--              round(((COALESCE(dividends.amount_per_share, 0 ) * (case when dividends.symbol = 'RIO' then 2 else 4 end)
--                  ) / dividends.last_price) * 100, 2) as calculated_yield,
             est_annual_income,
             (coalesce(overview.type, dividends.type))                             as type,
             overview.as_of_date
      from (select * from Account_Positions_Overview where symbol <> '') overview
               full outer join (select * from Account_Positions_Dividends where symbol <> '') dividends
                               on overview.symbol = dividends.symbol and overview.quantity = dividends.quantity) data;


create view account_dividends_aggregate as
select symbol                 as symbol,
       max(description)       as description,
       sum(quantity)          as quantity,
       max(last_price)        as last_price,
       max(amount_per_share)  as amount_per_share,
       max(yield)             as yield,
       sum(est_annual_income) as est_annual_income
from account_positions
where amount_per_share is not null
group by symbol
order by symbol;

