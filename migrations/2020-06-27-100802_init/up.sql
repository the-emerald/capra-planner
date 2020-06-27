-- Your SQL goes here
CREATE TABLE Users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    current_tissue INTEGER NOT NULL, -- fk
    current_zhl_setting_id INTEGER NOT NULL, -- fk
    current_vpm_setting_id INTEGER NOT NULL, -- fk
    current_gas_plan_setting_id INTEGER NOT NULL -- fk
);

CREATE TABLE ZHLSettings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    gfl INTEGER NOT NULL,
    gfh INTEGER NOT NULL,
    ascent_rate INTEGER NOT NULL,
    descent_rate INTEGER NOT NULL
);

CREATE TABLE VPMSettings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE GasPlanSettings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    sac_bottom INTEGER NOT NULL,
    sac_deco INTEGER NOT NULL
);

CREATE TABLE Dives (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,    -- fk
    deco_gases TEXT NOT NULL,
    tissue_before_id INTEGER NOT NULL,   -- fk
    timestamp DATETIME NOT NULL,
    zhl_settings_id INTEGER NOT NULL,    -- fk
    vpm_settings_id INTEGER NOT NULL,    -- fk
    gas_plan_setting_id INTEGER NOT NULL -- fk
);


CREATE TABLE Segments (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    dive_id INTEGER NOT NULL,   -- fk
    start_depth INTEGER NOT NULL,
    end_depth INTEGER NOT NULL,
    time INTEGER NOT NULL,
    gas_id INTEGER NOT NULL -- fk
);

CREATE TABLE Gases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    dive_id INTEGER NOT NULL -- fk
    o2 INTEGER NOT NULL,
    he INTEGER NOT NULL,
);

CREATE TABLE Tissues (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    n2_1 REAL NOT NULL,
    n2_2 REAL NOT NULL,
    n2_3 REAL NOT NULL,
    n2_4 REAL NOT NULL,
    n2_5 REAL NOT NULL,
    n2_6 REAL NOT NULL,
    n2_7 REAL NOT NULL,
    n2_8 REAL NOT NULL,
    n2_9 REAL NOT NULL,
    n2_10 REAL NOT NULL,
    n2_11 REAL NOT NULL,
    n2_12 REAL NOT NULL,
    n2_13 REAL NOT NULL,
    n2_14 REAL NOT NULL,
    n2_15 REAL NOT NULL,
    n2_16 REAL NOT NULL,
    he_1 REAL NOT NULL,
    he_2 REAL NOT NULL,
    he_3 REAL NOT NULL,
    he_4 REAL NOT NULL,
    he_5 REAL NOT NULL,
    he_6 REAL NOT NULL,
    he_7 REAL NOT NULL,
    he_8 REAL NOT NULL,
    he_9 REAL NOT NULL,
    he_10 REAL NOT NULL,
    he_11 REAL NOT NULL,
    he_12 REAL NOT NULL,
    he_13 REAL NOT NULL,
    he_14 REAL NOT NULL,
    he_15 REAL NOT NULL,
    he_16 REAL NOT NULL
);


-- Default values for settings
-- ZHL-16
INSERT INTO ZHLSettings VALUES (1, 50, 70, 20, -10)

-- VPM

-- Gas Planning
INSERT INTO GasPlanSettings VALUES (1, 20, 15)
