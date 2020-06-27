-- Your SQL goes here
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    current_tissue_id INTEGER NOT NULL,
    current_zhl_setting_id INTEGER NOT NULL DEFAULT 1,
    current_vpm_setting_id INTEGER NOT NULL DEFAULT 1,
    current_gas_plan_setting_id INTEGER NOT NULL DEFAULT 1,

    FOREIGN KEY (current_tissue_id) REFERENCES tissues(id),
    FOREIGN KEY (current_zhl_setting_id) REFERENCES zhl_settings(id),
    FOREIGN KEY (current_vpm_setting_id) REFERENCES vpm_settings(id),
    FOREIGN KEY (current_gas_plan_setting_id) REFERENCES gas_plan_settings(id)
);

CREATE TABLE zhl_settings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    gfl INTEGER NOT NULL,
    gfh INTEGER NOT NULL,
    ascent_rate INTEGER NOT NULL,
    descent_rate INTEGER NOT NULL
);

CREATE TABLE vpm_settings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE gas_plan_settings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    sac_bottom INTEGER NOT NULL,
    sac_deco INTEGER NOT NULL
);

CREATE TABLE dives (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    deco_gases TEXT NOT NULL,
    tissue_before_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    zhl_settings_id INTEGER NOT NULL,
    vpm_settings_id INTEGER NOT NULL,
    gas_plan_setting_id INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (tissue_before_id) REFERENCES tissues(id),
    FOREIGN KEY (zhl_settings_id) REFERENCES zhl_settings(id),
    FOREIGN KEY (vpm_settings_id) REFERENCES vpm_settings(id),
    FOREIGN KEY (gas_plan_setting_id) REFERENCES gas_plan_settings(id)
);


CREATE TABLE segments (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    dive_id INTEGER NOT NULL,
    start_depth INTEGER NOT NULL,
    end_depth INTEGER NOT NULL,
    time INTEGER NOT NULL,
    gas_id INTEGER NOT NULL,

    FOREIGN KEY (dive_id) REFERENCES dives(id),
    FOREIGN KEY (gas_id) REFERENCES gases(id)
);

CREATE TABLE gases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    dive_id INTEGER NOT NULL,
    o2 INTEGER NOT NULL,
    he INTEGER NOT NULL,

    FOREIGN KEY (dive_id) REFERENCES dives(id)
);

CREATE TABLE tissues (
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
INSERT INTO zhl_settings VALUES (1, 50, 70, 20, -10);

-- VPM
INSERT INTO vpm_settings VALUES (1);

-- Gas Planning
INSERT INTO gas_plan_settings VALUES (1, 20, 15);