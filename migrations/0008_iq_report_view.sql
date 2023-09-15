create view iq_report_latest as
select id,
       stock,
       div_growth,
       value_rating,
       price,
       dividend,
       yield,
       points_down,
       percent_down,
       undervalue_lo_price,
       undervalue_hi_yield,
       points_up,
       percent_up,
       overvalue_hi_price,
       overvalue_lo_yield,
       sp_rating,
       lo_52_wk,
       hi_52_wk,
       book_value,
       earnings_12_mo,
       price_to_earnings,
       pay_out,
       div_in_dgr,
       long_term_debt,
       bluechip_criteria,
       symbol,
       sector,
       industry,
       sub_sector,
       div_growth_3_year,
       div_growth_5_year,
       div_growth_10_year,
       report_date
from (select *,
             row_number() over (partition by symbol order by report_date desc) rnum
      from iq_report) iq
where iq.rnum = 1;


create view portfolio_report as
select ad.symbol                                           as symbol,
       ad.description                                      as description,
       stock                                               as stock,
       ad.last_price                                       as portfolio_price,
       ad.yield                                            as portfolio_yield,
       ir.price                                            as report_price,
       ir.yield                                            as report_yield,
       overvalue_hi_price,
       round((price / overvalue_hi_price) * 100, 0) || '%' as "%_of_overvalue",
       report_date
from account_dividends_aggregate ad
         left join iq_report_latest ir on ad.symbol = ir.symbol
order by "%_of_overvalue" desc;
