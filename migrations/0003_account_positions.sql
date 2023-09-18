CREATE TABLE IF NOT EXISTS Account_Positions_Overview
(
    id                      INT GENERATED ALWAYS AS IDENTITY,
    Account_Number          TEXT,
    Account_Name            TEXT,
    symbol                  TEXT,
    Description             TEXT,
    quantity                DECIMAL(10, 2),
    Last_Price              DECIMAL(10, 2),
    Last_Price_Change       DECIMAL(10, 2),
    Current_Value           DECIMAL(10, 2),
    Today_Gain_Loss_Dollar  DECIMAL(10, 2),
    Today_Gain_Loss_Percent DECIMAL(10, 2),
    Total_Gain_Loss_Dollar  DECIMAL(10, 2),
    Total_Gain_Loss_Percent DECIMAL(10, 2),
    Percent_Of_Account      DECIMAL(10, 2),
    Cost_Basis_Total        DECIMAL(10, 2),
    Average_Cost_Basis      DECIMAL(10, 2),
    Type                    TEXT,
    Hash                    TEXT
);

CREATE TABLE IF NOT EXISTS Account_Positions_Dividends
(
    id                 INT GENERATED ALWAYS AS IDENTITY,
    Account_Number     TEXT,
    Account_Name       TEXT,
    symbol             TEXT,
    Description        TEXT,
    quantity           DECIMAL(10, 2),
    Last_Price         DECIMAL(10, 2),
    Last_Price_Change  DECIMAL(10, 2),
    Current_Value      DECIMAL(10, 2),
    Percent_Of_Account DECIMAL(10, 2),
    Ex_Date            DATE,
    Amount_Per_Share   DECIMAL(10, 2),
    Pay_Date           DATE,
    Yield              DECIMAL(10, 2),
    Est_Annual_Income  DECIMAL(10, 2),
    Type               TEXT,
    Hash               TEXT
);

