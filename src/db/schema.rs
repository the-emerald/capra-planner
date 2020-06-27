table! {
    Dives (id) {
        id -> Integer,
        user_id -> Integer,
        deco_gases -> Text,
        tissue_before_id -> Integer,
        timestamp -> Timestamp,
        zhl_settings_id -> Integer,
        vpm_settings_id -> Integer,
        gas_plan_setting_id -> Integer,
    }
}

table! {
    GasPlanSettings (id) {
        id -> Integer,
        sac_bottom -> Integer,
        sac_deco -> Integer,
    }
}

table! {
    Gases (id) {
        id -> Integer,
        dive_id -> Integer,
        o2 -> Integer,
        he -> Integer,
    }
}

table! {
    Segments (id) {
        id -> Integer,
        dive_id -> Integer,
        start_depth -> Integer,
        end_depth -> Integer,
        time -> Integer,
        gas_id -> Integer,
    }
}

table! {
    Tissues (id) {
        id -> Integer,
        n2_1 -> Float,
        n2_2 -> Float,
        n2_3 -> Float,
        n2_4 -> Float,
        n2_5 -> Float,
        n2_6 -> Float,
        n2_7 -> Float,
        n2_8 -> Float,
        n2_9 -> Float,
        n2_10 -> Float,
        n2_11 -> Float,
        n2_12 -> Float,
        n2_13 -> Float,
        n2_14 -> Float,
        n2_15 -> Float,
        n2_16 -> Float,
        he_1 -> Float,
        he_2 -> Float,
        he_3 -> Float,
        he_4 -> Float,
        he_5 -> Float,
        he_6 -> Float,
        he_7 -> Float,
        he_8 -> Float,
        he_9 -> Float,
        he_10 -> Float,
        he_11 -> Float,
        he_12 -> Float,
        he_13 -> Float,
        he_14 -> Float,
        he_15 -> Float,
        he_16 -> Float,
    }
}

table! {
    Users (id) {
        id -> Integer,
        name -> Text,
        current_tissue_id -> Integer,
        current_zhl_setting_id -> Integer,
        current_vpm_setting_id -> Integer,
        current_gas_plan_setting_id -> Integer,
    }
}

table! {
    VPMSettings (id) {
        id -> Integer,
    }
}

table! {
    ZHLSettings (id) {
        id -> Integer,
        gfl -> Integer,
        gfh -> Integer,
        ascent_rate -> Integer,
        descent_rate -> Integer,
    }
}

joinable!(Dives -> GasPlanSettings (gas_plan_setting_id));
joinable!(Dives -> Tissues (tissue_before_id));
joinable!(Dives -> Users (user_id));
joinable!(Dives -> VPMSettings (vpm_settings_id));
joinable!(Dives -> ZHLSettings (zhl_settings_id));
joinable!(Gases -> Dives (dive_id));
joinable!(Segments -> Dives (dive_id));
joinable!(Segments -> Gases (gas_id));
joinable!(Users -> GasPlanSettings (current_gas_plan_setting_id));
joinable!(Users -> Tissues (current_tissue_id));
joinable!(Users -> VPMSettings (current_vpm_setting_id));
joinable!(Users -> ZHLSettings (current_zhl_setting_id));

allow_tables_to_appear_in_same_query!(
    Dives,
    GasPlanSettings,
    Gases,
    Segments,
    Tissues,
    Users,
    VPMSettings,
    ZHLSettings,
);
