CREATE SCHEMA IF NOT EXISTS ssa;

CREATE TABLE ssa.baby_names (
    name TEXT,
    year INT,
    sex  CHAR,
    count BIGINT,
    PRIMARY KEY(name, year, sex)
);