
CREATE TABLE IF NOT EXISTS Account_History_Source_File
(
    id         INT GENERATED ALWAYS AS IDENTITY UNIQUE,
    filename   TEXT,
    start_date DATE,
    end_date   DATE
);

CREATE TABLE IF NOT EXISTS Account_History
(
    id                   INT GENERATED ALWAYS AS IDENTITY UNIQUE,
    Run_Date             DATE,
    Account              TEXT,
    Action               TEXT,
    symbol               TEXT,
    security_description TEXT,
    security_type        TEXT,
    exchange_quantity    DECIMAL(10, 2),
    exchange_currency    TEXT,
    quantity             DECIMAL(10, 3),
    currency             TEXT,
    price                DECIMAL(10, 2),
    exchange_rate        TEXT,
    commission           DECIMAL(10, 2),
    fees                 DECIMAL(10, 2),
    accrued_interest     DECIMAL(10, 2),
    amount               DECIMAL(10, 2),
    settlement_date      DATE,
    Source_File_id       INT REFERENCES Account_History_Source_File (id)
);
