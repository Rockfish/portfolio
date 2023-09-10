CREATE TABLE IF NOT EXISTS Account_History (
    id                   INT GENERATED ALWAYS AS IDENTITY,
    Run_Date             DATE,
    Account              TEXT,
    Action               TEXT,
    Symbol               TEXT,
    Security_Description TEXT,
    Security_Type        TEXT,
    Exchange_Quantity    DECIMAL(10, 2),
    Exchange_Currency    TEXT,
    Quantity             DECIMAL(10, 3),
    Currency             TEXT,
    Price                DECIMAL(10, 2),
    Exchange_Rate        TEXT,
    Commission           DECIMAL(10, 2),
    Fees                 DECIMAL(10, 2),
    Accrued_Interest     DECIMAL(10, 2),
    Amount               DECIMAL(10, 2),
    Settlement_Date      DATE,
    Hash                 TEXT
);

CREATE TABLE IF NOT EXISTS Account_History_Data_Files
(
    id              INT GENERATED ALWAYS AS IDENTITY,
    file_name       TEXT,
    file_date       DATE,
    data_start_date DATE,
    data_end_date   DATE
);

create or replace procedure account_history_cleanup()
    LANGUAGE SQL
BEGIN ATOMIC
delete
from Account_History
where id in (select id
             from (select id, row_number() over (partition by Hash) as row_num from Account_History) as data
             where data.row_num > 1);
END;
