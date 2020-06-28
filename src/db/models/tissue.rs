use crate::db::schema::{tissues};

#[derive(Queryable, Identifiable)]
#[table_name = "tissues"]
pub struct Tissue {
    pub id: i32,

    // Nitrogen loadings
    pub n2_1: f64,
    pub n2_2: f64,
    pub n2_3: f64,
    pub n2_4: f64,
    pub n2_5: f64,
    pub n2_6: f64,
    pub n2_7: f64,
    pub n2_8: f64,
    pub n2_9: f64,
    pub n2_10: f64,
    pub n2_11: f64,
    pub n2_12: f64,
    pub n2_13: f64,
    pub n2_14: f64,
    pub n2_15: f64,
    pub n2_16: f64,

    // Helium loadings
    pub he_1: f64,
    pub he_2: f64,
    pub he_3: f64,
    pub he_4: f64,
    pub he_5: f64,
    pub he_6: f64,
    pub he_7: f64,
    pub he_8: f64,
    pub he_9: f64,
    pub he_10: f64,
    pub he_11: f64,
    pub he_12: f64,
    pub he_13: f64,
    pub he_14: f64,
    pub he_15: f64,
    pub he_16: f64,
}