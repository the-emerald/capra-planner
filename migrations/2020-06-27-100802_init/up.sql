-- Your SQL goes here
CREATE TABLE Users (
    id integer PRIMARY KEY AUTOINCREMENT,
    name text,
    tissue_state text,
    current_zhl_setting_id integer,
    current_vpm_setting_id integer,
    current_gas_plan_setting_id integer
);

CREATE TABLE ZHLSettings (
    id integer PRIMARY KEY AUTOINCREMENT,
    gfl integer,
    gfh integer,
    ascent_rate integer,
    descent_rate integer
);

CREATE TABLE GasPlanSettings (
    id integer PRIMARY KEY AUTOINCREMENT,
    sac_bottom integer,
    sac_deco integer
);

CREATE TABLE Dives (
    id integer PRIMARY KEY AUTOINCREMENT,
    user_id integer,
    deco_gases text,
    timestamp datetime,
    zhl_settings_id integer,
    vpm_settings_id integer,
    gas_plan_setting_id integer
);

CREATE TABLE VPMSettings (
    id integer PRIMARY KEY AUTOINCREMENT,
    magic text
);

CREATE TABLE Segments (
    id integer PRIMARY KEY AUTOINCREMENT,
    dive_id integer,
    T text,
    gas_id integer
);

CREATE TABLE Gases (
    id integer PRIMARY KEY AUTOINCREMENT,
    o2 integer,
    he integer,
    dive_id integer
);

CREATE TABLE Tissues (
    user_id integer,
    n2_1 integer,
    n2_2 integer,
    n2_3 integer,
    n2_4 integer,
    n2_5 integer,
    n2_6 integer,
    n2_7 integer,
    n2_8 integer,
    n2_9 integer,
    n2_10 integer,
    n2_11 integer,
    n2_12 integer,
    n2_13 integer,
    n2_14 integer,
    n2_15 integer,
    n2_16 integer,
    he_1 integer,
    he_2 integer,
    he_3 integer,
    he_4 integer,
    he_5 integer,
    he_6 integer,
    he_7 integer,
    he_8 integer,
    he_9 integer,
    he_10 integer,
    he_11 integer,
    he_12 integer,
    he_13 integer,
    he_14 integer,
    he_15 integer,
    he_16 integer
);