
create view stock_activity_summary as
select symbol,
       activity,
       TO_char(sum(amount), '999,999D99')                                                               sum,
       max(run_date)                                                                                    max_date,
       min(run_date)                                                                                    min_date,
       (case when symbol in (select distinct symbol from account_positions) THEN 'Own' ELSE 'Sold' END) status
from accounts_activity
where activity not in ('Transferred in', 'Transferred out', 'Transfer of assets')
  and symbol not in ('', '00162Q866', '315994103', '850578TU9')
--   and symbol not in (select distinct symbol from account_positions)
group by symbol, activity
order by symbol;
