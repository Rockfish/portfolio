CREATE TABLE IF NOT EXISTS Account_Positions_Overview
(
    id                      INT GENERATED ALWAYS AS IDENTITY,
    account_number          TEXT,
    account_name            TEXT,
    symbol                  TEXT,
    description             TEXT,
    quantity                DECIMAL(10, 2),
    last_price              DECIMAL(10, 2),
    last_price_change       DECIMAL(10, 2),
    current_value           DECIMAL(10, 2),
    today_gain_loss_dollar  DECIMAL(10, 2),
    today_gain_loss_percent DECIMAL(10, 2),
    total_gain_loss_dollar  DECIMAL(10, 2),
    total_gain_loss_percent DECIMAL(10, 2),
    percent_of_account      DECIMAL(10, 2),
    cost_basis_total        DECIMAL(10, 2),
    average_cost_basis      DECIMAL(10, 2),
    type                    TEXT,
    as_of_date              DATE,
    Hash                    TEXT
);

CREATE TABLE IF NOT EXISTS Account_Positions_Dividends
(
    id                 INT GENERATED ALWAYS AS IDENTITY,
    account_number     TEXT,
    account_name       TEXT,
    symbol             TEXT,
    description        TEXT,
    quantity           DECIMAL(10, 2),
    last_price         DECIMAL(10, 2),
    last_price_change  DECIMAL(10, 2),
    current_value      DECIMAL(10, 2),
    percent_of_account DECIMAL(10, 2),
    ex_date            DATE,
    amount_per_share   DECIMAL(10, 2),
    pay_date           DATE,
    yield              DECIMAL(10, 2),
    est_annual_income  DECIMAL(10, 2),
    type               TEXT,
    as_of_date         DATE,
    Hash               TEXT
);

