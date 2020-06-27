table! {
    dives (id) {
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
    gas_plan_settings (id) {
        id -> Integer,
        sac_bottom -> Integer,
        sac_deco -> Integer,
    }
}

table! {
    gases (id) {
        id -> Integer,
        dive_id -> Integer,
        o2 -> Integer,
        he -> Integer,
    }
}

table! {
    segments (id) {
        id -> Integer,
        dive_id -> Integer,
        start_depth -> Integer,
        end_depth -> Integer,
        time -> Integer,
        gas_id -> Integer,
    }
}

table! {
    tissues (id) {
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
    users (id) {
        id -> Integer,
        name -> Text,
        current_tissue_id -> Integer,
        current_zhl_setting_id -> Integer,
        current_vpm_setting_id -> Integer,
        current_gas_plan_setting_id -> Integer,
    }
}

table! {
    vpm_settings (id) {
        id -> Integer,
    }
}

table! {
    zhl_settings (id) {
        id -> Integer,
        gfl -> Integer,
        gfh -> Integer,
        ascent_rate -> Integer,
        descent_rate -> Integer,
    }
}

joinable!(dives -> gas_plan_settings (gas_plan_setting_id));
joinable!(dives -> tissues (tissue_before_id));
joinable!(dives -> users (user_id));
joinable!(dives -> vpm_settings (vpm_settings_id));
joinable!(dives -> zhl_settings (zhl_settings_id));
joinable!(gases -> dives (dive_id));
joinable!(segments -> dives (dive_id));
joinable!(segments -> gases (gas_id));
joinable!(users -> gas_plan_settings (current_gas_plan_setting_id));
joinable!(users -> tissues (current_tissue_id));
joinable!(users -> vpm_settings (current_vpm_setting_id));
joinable!(users -> zhl_settings (current_zhl_setting_id));

allow_tables_to_appear_in_same_query!(
    dives,
    gas_plan_settings,
    gases,
    segments,
    tissues,
    users,
    vpm_settings,
    zhl_settings,
);
