table! {
    dives (id) {
        id -> Integer,
        user_id -> Integer,
        tissue_before_id -> Integer,
        timestamp -> Timestamp,
        zhl_settings_id -> Integer,
        vpm_settings_id -> Integer,
        gas_plan_settings_id -> Integer,
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
        dive_id -> Nullable<Integer>,
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
        n2_1 -> Double,
        n2_2 -> Double,
        n2_3 -> Double,
        n2_4 -> Double,
        n2_5 -> Double,
        n2_6 -> Double,
        n2_7 -> Double,
        n2_8 -> Double,
        n2_9 -> Double,
        n2_10 -> Double,
        n2_11 -> Double,
        n2_12 -> Double,
        n2_13 -> Double,
        n2_14 -> Double,
        n2_15 -> Double,
        n2_16 -> Double,
        he_1 -> Double,
        he_2 -> Double,
        he_3 -> Double,
        he_4 -> Double,
        he_5 -> Double,
        he_6 -> Double,
        he_7 -> Double,
        he_8 -> Double,
        he_9 -> Double,
        he_10 -> Double,
        he_11 -> Double,
        he_12 -> Double,
        he_13 -> Double,
        he_14 -> Double,
        he_15 -> Double,
        he_16 -> Double,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        current_tissue_id -> Integer,
        current_zhl_settings_id -> Integer,
        current_vpm_settings_id -> Integer,
        current_gas_plan_settings_id -> Integer,
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

joinable!(dives -> gas_plan_settings (gas_plan_settings_id));
joinable!(dives -> tissues (tissue_before_id));
joinable!(dives -> users (user_id));
joinable!(dives -> vpm_settings (vpm_settings_id));
joinable!(dives -> zhl_settings (zhl_settings_id));
joinable!(gases -> dives (dive_id));
joinable!(segments -> dives (dive_id));
joinable!(segments -> gases (gas_id));
joinable!(users -> gas_plan_settings (current_gas_plan_settings_id));
joinable!(users -> tissues (current_tissue_id));
joinable!(users -> vpm_settings (current_vpm_settings_id));
joinable!(users -> zhl_settings (current_zhl_settings_id));

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
