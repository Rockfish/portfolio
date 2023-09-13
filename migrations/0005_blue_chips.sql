create table blue_chips
(
    id         INT GENERATED ALWAYS AS IDENTITY,
    symbol     text,
    name       text,
    sector     text,
    chart_link text
)