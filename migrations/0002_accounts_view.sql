
create or replace view Accounts_Activity as
select id,
       Run_Date             AS Run_Date,
       Account              AS Account,
       (case
            when (Action like 'YOU BOUGHT%') then 'Bought'
            when (Action like 'YOU SOLD%') then 'Sold'
            when (Action like 'DIVIDEND RECEIVED%') then 'Dividend'
            when (Action like 'INTEREST EARNED%') then 'Earned interest'
            when (Action like 'REINVESTMENT%') then 'Reinvestment'
            when (Action like 'TRANSFERRED FROM%') then 'Transferred in'
            when (Action like 'TRANSFERRED TO%') then 'Transferred out'
            when (Action like 'Electronic Funds Transfer Paid%') then 'Electronic funds paid out'
            when (Action like 'Electronic Funds Transfer Received (Cash)%') then 'Electronic funds received'
            when (Action like 'Contributions%') then 'Contribution'
            when (Action like 'WIRE TRANSFER TO BANK (Cash)%') then 'Wire to bank'
            when (Action like 'WIRE TRANSFER FROM BANK (Cash)%') then 'Wire from bank'
            when (Action like 'FED TAX W/H FEDERAL TAX WITHHELD (Cash)%') then 'Fed Tax withheld'
            when (Action like 'FEE CHARGED%') then 'Fee charged'
            when (Action like 'FOREIGN TAX PAID%') then 'Foreign tax paid'
            when (Action like 'CASH CONTRIBUTION CURRENT YEAR (Cash)%') then 'Cash contribution'
            when (Action like 'Exchange In') then 'Exchange In'
            when (Action like 'Exchange Out') then 'Exchange Out'
            when (Action like 'Change on Market Value') then 'Change of Market Value'
            when (Action like 'Change In Market Value') then 'Change of Market Value'
            when (Action like 'TRANSFER OF ASSETS%') then 'Transfer of assets'
            when (Action like 'PARTIAL DISTRIBUTION%') then 'Partial distribution'
            when (Action like 'JOURNALED SPP PURCHASE CREDIT (Cash)') then 'ESPP purchase credit'
            when (Action like 'IN LIEU OF FRX SHARE%') then 'Cash in lieu of fractional shares'
            when (Action like 'ADJ FOREIGN TAX PAID TAX%') then 'Foreign tax paid'
            when (Action like 'ADJUST FEE CHARGED%') then 'Adjusted fee'
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