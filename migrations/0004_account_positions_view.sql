create or replace view Account_Positions as
select overview_id,
       dividend_id,
       Account_Number,
       Account_Name,
       symbol,
       Description,
       quantity,
       Last_Price,
       Last_Price_Change,
       Current_Value,
       Percent_Of_Account,
       Today_Gain_Loss_Dollar,
       Today_Gain_Loss_Percent,
       Total_Gain_Loss_Dollar,
       Total_Gain_Loss_Percent,
       Cost_Basis_Total,
       Average_Cost_Basis,
       Ex_Date,
       Amount_Per_Share,
       Pay_Date,
       Yield,
--        calculated_yield,
       Est_Annual_Income,
       Type
from (select overview.id                                                           as overview_id,
             dividends.id                                                          as dividend_id,
             (coalesce(overview.Account_Number, dividends.Account_Number))         as Account_Number,
             (coalesce(overview.Account_Name, dividends.Account_name))             as Account_name,
             (coalesce(overview.symbol, dividends.symbol))                         as symbol,
             (coalesce(overview.Description, dividends.Description))               as Description,
             (coalesce(overview.quantity, dividends.quantity))                     as quantity,
             (coalesce(overview.Last_Price, dividends.Last_Price))                 as Last_Price,
             (coalesce(overview.Last_Price_Change, dividends.Last_Price_Change))   as Last_Price_Change,
             (coalesce(overview.Current_Value, dividends.Current_Value))           as Current_Value,
             (coalesce(overview.Percent_Of_Account, dividends.Percent_Of_Account)) as Percent_Of_Account,
             Today_Gain_Loss_Dollar,
             Today_Gain_Loss_Percent,
             Total_Gain_Loss_Dollar,
             Total_Gain_Loss_Percent,
             Cost_Basis_Total,
             Average_Cost_Basis,
             Ex_Date,
             Amount_Per_Share,
             Pay_Date,
             Yield,
--              round(((COALESCE(dividends.Amount_Per_Share, 0 ) * (case when dividends.symbol = 'RIO' then 2 else 4 end)
--                  ) / dividends.Last_Price) * 100, 2) as calculated_yield,
             Est_Annual_Income,
             (coalesce(overview.Type, dividends.Type))                             as Type
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

