create or replace view total_gain_percentage as
select total_dividend_and_interest,
             total_stocks_gain_loss,
             total_dividend_and_interest + total_stocks_gain_loss                                               as total_gain,
             current_value_jan_31_2023,
             round((total_dividend_and_interest + total_stocks_gain_loss) / (current_value_jan_31_2023 - total_dividend_and_interest) * 100, 2) as percentage,
             (select sum(amount) from accounts_history_normalized where extract(year from run_date) > 2022 and activity = 'Electronic funds paid out') as paid_out
      from (select sum(current_value)                                              as current_value_jan_31_2023,
                   sum(total_gain_loss_dollar)                                     as total_stocks_gain_loss,
                   (select sum(amount) as total_dividend_and_interest
                    from accounts_history_normalized
                    where extract(year from run_date) > 2022
                      and (activity = 'Dividend' or activity = 'Interest Earned')) as total_dividend_and_interest
            from account_positions_normalized
            where as_of_date = '2024-01-31') as ap;



CREATE OR REPLACE FUNCTION format_number(numeric)
    RETURNS text AS
$$
BEGIN
    RETURN to_char($1, 'FM999,999,999,990.00');
END;
$$ LANGUAGE plpgsql;

create or replace view total_gain_percentage_rows as
SELECT v.column_name as item, v.value
FROM (select * from total_gain_percentage) t,
     LATERAL (
         VALUES ('total dividend and interest since 01-01-2022', format_number(t.total_dividend_and_interest)),
                ('total stocks gain loss', format_number(t.total_stocks_gain_loss)),
                ('total gain', format_number(t.total_gain)),
                ('current value as of 01-31-2023', format_number(t.current_value_jan_31_2023)),
                ('percentage gain', format_number(t.percentage)),
                ('paid out', format_number(t.paid_out))
         ) AS v(column_name, value);
