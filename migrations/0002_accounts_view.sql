
create or replace view Accounts_Activity as
select id,
       Run_Date             AS Run_Date,
       Account              AS Account,
       (case
            when (Action like 'YOU BOUGHT%') then 'BOUGHT'
            when (Action like 'YOU SOLD%') then 'SOLD'
            when (Action like 'DIVIDEND RECEIVED%') then 'DIVIDEND'
            when (Action like 'INTEREST EARNED%') then 'EARNED INTEREST'
            when (Action like 'REINVESTMENT%') then 'REINVESTMENT'
            when (Action like 'TRANSFERRED FROM%') then 'TRANSFERRED IN'
            when (Action like 'TRANSFERRED TO%') then 'TRANSFERRED OUT'
            when (Action like 'Electronic Funds Transfer Paid%') then 'PAID OUT'
            else '' end)    AS Activity,
       Action               AS Action,
       Symbol               AS Symbol,
       Security_Description AS Security_Description,
       Security_Type        AS Security_Type,
       Exchange_Quantity    AS Exchange_Quantity,
       Exchange_Currency    AS Exchange_Currency,
       Quantity             AS Quantity,
       Currency             AS Currency,
       Price                AS Price,
       Exchange_Rate        AS Exchange_Rate,
       Commission           AS Commission,
       Fees                 AS Fees,
       Accrued_Interest     AS Accrued_Interest,
       Amount               AS Amount,
       Settlement_Date      AS Settlement_Date
from (select id                                     AS id,
             Run_Date                               AS Run_Date,
             Account                                AS Account,
             Action                                 AS Action,
             Symbol                                 AS Symbol,
             Security_Description                   AS Security_Description,
             Security_Type                          AS Security_Type,
             Exchange_Quantity                      AS Exchange_Quantity,
             Exchange_Currency                      AS Exchange_Currency,
             Quantity                               AS Quantity,
             Currency                               AS Currency,
             Price                                  AS Price,
             Exchange_Rate                          AS Exchange_Rate,
             Commission                             AS Commission,
             Fees                                   AS Fees,
             Accrued_Interest                       AS Accrued_Interest,
             Amount                                 AS Amount,
             Settlement_Date                        AS Settlement_Date,
             row_number() OVER (PARTITION BY Hash ) AS row_num
      from portfolio.accounts_history) data
where (row_num = 1);