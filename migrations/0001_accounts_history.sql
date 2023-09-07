CREATE TABLE IF NOT EXISTS Accounts_History (
    id                   BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
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

CREATE TABLE IF NOT EXISTS Accounts_History_Data_Files
(
    id              BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    file_name       TEXT,
    file_date       DATE,
    data_start_date DATE,
    data_end_date   DATE
);
