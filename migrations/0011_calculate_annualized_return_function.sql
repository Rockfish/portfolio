CREATE OR REPLACE FUNCTION calculate_annualized_return_with_dividends(
    buy_price NUMERIC,
    sell_price NUMERIC,
    number_of_shares NUMERIC,
    dividends_received NUMERIC,
    buy_date DATE,
    sell_date DATE
)
    RETURNS NUMERIC AS
$$
DECLARE
    -- Declare variables
    days_held                        INTEGER;
    years_held                       NUMERIC;
    total_return_with_dividends      NUMERIC;
    annualized_return_with_dividends NUMERIC;
BEGIN
    -- Calculate the number of days the investment was held
    days_held := sell_date - buy_date;

    -- Convert days held into years
    years_held := days_held / 365.0;

    -- Adjust the sell price to include dividends received
    sell_price := sell_price + (dividends_received / number_of_shares);

    -- Calculate the total return including dividends
    total_return_with_dividends := sell_price / buy_price;

    -- Calculate the annualized rate of return including dividends
    IF years_held = 0 THEN
        -- Avoid division by zero if the investment was bought and sold on the same day
        RETURN 0;
    ELSE
        annualized_return_with_dividends := POWER(total_return_with_dividends, (1 / years_held)) - 1;
        RETURN annualized_return_with_dividends;
    END IF;
END;
$$ LANGUAGE plpgsql;


drop type annualized_return_row_type cascade;
CREATE TYPE annualized_return_row_type AS
(
    symbol            TEXT,
    buy_date          date,
    sale_date         date,
    num_days          numeric,
    buy_price         numeric,
    sale_price        numeric,
    buy_amount        numeric,
    sale_amount       numeric,
    num_shares       numeric,
    sale_gain       numeric,
    dividends_received  numeric,
    total_gain      numeric,
    percentage_increase numeric,
    annualized_rate numeric
);


CREATE OR REPLACE FUNCTION get_annualized_return_table(
    symbol_name TEXT
)
    RETURNS SETOF annualized_return_row_type AS
$$
DECLARE
    sell_price         NUMERIC;
    sold_amount        NUMERIC;
    sold_profit        NUMERIC;
    number_of_shares   NUMERIC;
    dividends_received NUMERIC;
    dividend_per_share NUMERIC;
    sell_date          DATE;
    total_profit       NUMERIC;
    annualized_return  NUMERIC;
    buy_record         RECORD;
    return_record      annualized_return_row_type;
BEGIN

    -- Dividend
    select sum(amount)
    into dividends_received
    from account_history_normalized
    where symbol = symbol_name
      and activity in ('Dividend');

    -- Sold
    select max(run_date), -1 * sum(quantity), sum(amount), -1 * sum(amount) / sum(quantity)
    into sell_date, number_of_shares, sold_amount, sell_price
    from account_history_normalized h
    where h.symbol = symbol_name
      and activity in ('Sold')
    group by h.symbol, activity;

    dividend_per_share := dividends_received / number_of_shares;

    -- Per Buy
    FOR buy_record IN select activity,
                             run_date                         as buy_date,
                             sum(quantity)                    as quantity,
                             -1 * sum(amount)                 as amount,
                             -1 * sum(amount) / sum(quantity) as buy_price -- works with group by
                      from account_history_normalized
                      where symbol = symbol_name
                        and activity in ('Bought', 'Reinvestment')
                      group by activity, run_date
        LOOP

            sold_profit := (sell_price - buy_record.buy_price) * buy_record.quantity;

            annualized_return := round(calculate_annualized_return_with_dividends(
                                               buy_record.buy_price,
                                               sell_price,
                                               buy_record.quantity,
                                               dividend_per_share * buy_record.quantity,
                                               buy_record.buy_date,
                                               sell_date) * 100, 2);

            total_profit := round((sell_price - buy_record.buy_price) * buy_record.quantity +
                                  dividend_per_share * buy_record.quantity, 2);

            return_record.symbol := symbol_name;
            return_record.buy_date := buy_record.buy_date;
            return_record.sale_date := sell_date;
            return_record.num_days := sell_date - buy_record.buy_date;
            return_record.buy_price := round(buy_record.buy_price, 2);
            return_record.sale_price := round(sell_price, 2);
            return_record.buy_amount := buy_record.amount;
            return_record.sale_amount := round(sell_price * buy_record.quantity, 2);
            return_record.num_shares := buy_record.quantity;
            return_record.sale_gain := round(sold_profit, 2);
            return_record.dividends_received := round(dividend_per_share * buy_record.quantity, 2);
            return_record.total_gain := total_profit;
            return_record.percentage_increase := round(total_profit / buy_record.amount * 100, 2);
            return_record.annualized_rate := annualized_return;

            return next return_record;

        end loop;
END;
$$ LANGUAGE plpgsql;



CREATE OR REPLACE FUNCTION get_annualized_return_table_rows(
    symbol_name TEXT
)
    RETURNS TABLE
            (
                Item  TEXT,
                Value TEXT
            )
AS
$$
BEGIN
    RETURN QUERY SELECT v.column_name as item, v.value
                 from (select symbol_name                                                  as symbol,
                              (171113.13 - 148019.36 + 15040.23)                           as profit,
                              round(calculate_annualized_return_with_dividends(
                                            56.84,
                                            66.07,
                                            2589,
                                            15040.23,
                                            '2022-07-05'::date,
                                            TO_DATE('2023-09-15', 'YYYY-MM-DD')) * 100,
                                    2)                                                     as annualized_return_percentage) t,
                      LATERAL (
                          VALUES ('Symbol', symbol),
                                 ('Total Profit', format_number(t.profit)),
                                 ('Annualized Return %', format_number(t.annualized_return_percentage))
                          ) AS v(column_name, value);
END;
$$ LANGUAGE plpgsql;