-- Your SQL goes here
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    current_tissue_id INTEGER NOT NULL DEFAULT 1,
    current_zhl_settings_id INTEGER NOT NULL DEFAULT 1,
    current_vpm_settings_id INTEGER NOT NULL DEFAULT 1,
    current_general_settings_id INTEGER NOT NULL DEFAULT 1,

    FOREIGN KEY (current_tissue_id) REFERENCES tissues(id),
    FOREIGN KEY (current_zhl_settings_id) REFERENCES zhl_settings(id),
    FOREIGN KEY (current_vpm_settings_id) REFERENCES vpm_settings(id),
    FOREIGN KEY (current_general_settings_id) REFERENCES general_settings (id)
);

CREATE TABLE zhl_settings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    subtype TEXT NOT NULL,
    gfl INTEGER NOT NULL,
    gfh INTEGER NOT NULL
);

CREATE TABLE vpm_settings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE general_settings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    sac_bottom INTEGER NOT NULL,
    sac_deco INTEGER NOT NULL,
    ascent_rate INTEGER NOT NULL,
    descent_rate INTEGER NOT NULL
);

CREATE TABLE dives (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    tissue_before_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    executed INTEGER NOT NULL, -- Boolean
    zhl_settings_id INTEGER NOT NULL,
    vpm_settings_id INTEGER NOT NULL,
    general_settings_id INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (tissue_before_id) REFERENCES tissues(id),
    FOREIGN KEY (zhl_settings_id) REFERENCES zhl_settings(id),
    FOREIGN KEY (vpm_settings_id) REFERENCES vpm_settings(id),
    FOREIGN KEY (general_settings_id) REFERENCES general_settings (id)
);


CREATE TABLE segments (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    dive_id INTEGER NOT NULL,
    start_depth INTEGER NOT NULL,
    end_depth INTEGER NOT NULL,
    time INTEGER NOT NULL,
    ascent_rate INTEGER NOT NULL,
    descent_rate INTEGER NOT NULL,
    gas_id INTEGER NOT NULL,

    FOREIGN KEY (dive_id) REFERENCES dives(id),
    FOREIGN KEY (gas_id) REFERENCES gases(id)
);

CREATE TABLE gases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    dive_id INTEGER,
    o2 INTEGER NOT NULL,
    he INTEGER NOT NULL,
    max_operating_depth INTEGER,

    FOREIGN KEY (dive_id) REFERENCES dives(id)
);

CREATE TABLE tissues (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    n2_1 DOUBLE NOT NULL,
    n2_2 DOUBLE NOT NULL,
    n2_3 DOUBLE NOT NULL,
    n2_4 DOUBLE NOT NULL,
    n2_5 DOUBLE NOT NULL,
    n2_6 DOUBLE NOT NULL,
    n2_7 DOUBLE NOT NULL,
    n2_8 DOUBLE NOT NULL,
    n2_9 DOUBLE NOT NULL,
    n2_10 DOUBLE NOT NULL,
    n2_11 DOUBLE NOT NULL,
    n2_12 DOUBLE NOT NULL,
    n2_13 DOUBLE NOT NULL,
    n2_14 DOUBLE NOT NULL,
    n2_15 DOUBLE NOT NULL,
    n2_16 DOUBLE NOT NULL,
    he_1 DOUBLE NOT NULL,
    he_2 DOUBLE NOT NULL,
    he_3 DOUBLE NOT NULL,
    he_4 DOUBLE NOT NULL,
    he_5 DOUBLE NOT NULL,
    he_6 DOUBLE NOT NULL,
    he_7 DOUBLE NOT NULL,
    he_8 DOUBLE NOT NULL,
    he_9 DOUBLE NOT NULL,
    he_10 DOUBLE NOT NULL,
    he_11 DOUBLE NOT NULL,
    he_12 DOUBLE NOT NULL,
    he_13 DOUBLE NOT NULL,
    he_14 DOUBLE NOT NULL,
    he_15 DOUBLE NOT NULL,
    he_16 DOUBLE NOT NULL
);

-- Default values
-- ZHL-16
INSERT INTO zhl_settings VALUES (1, 'B', 50, 70);

-- VPM
INSERT INTO vpm_settings VALUES (1);

-- Gas Planning
INSERT INTO general_settings VALUES (1, 20, 15, -10, 20);

-- Tissues
INSERT INTO tissues VALUES (1, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79, 0.79,
                            0.79, 0.79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
