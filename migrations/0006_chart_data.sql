create table chart_data
(
    id                    INT GENERATED ALWAYS AS IDENTITY,
    Symbol                TEXT,
    Chart                 TEXT,
    Update_date           TEXT,
    Quality_Rank          TEXT,
    Shares_Outstanding    TEXT,
    Institution_Own       TEXT,
    Div_Paid_Since        TEXT,
    Profit_Margin         TEXT,
    TTM_Earnings          TEXT,
    PE_Ratio              TEXT,
    Book_Value            TEXT,
    Div_Payout            TEXT,
    Current_Price         TEXT,
    Current_Yield         TEXT,
    Overvalue_Price       TEXT,
    Overvalue_Pts_Up      TEXT,
    Overvalue_Yield       TEXT,
    Overvalue_Percent_Up  TEXT,
    Undervalue_Price      TEXT,
    Undervalue_Pts_Dn     TEXT,
    Undervalue_Yield      TEXT,
    Undervalue_Percent_Dn TEXT
);