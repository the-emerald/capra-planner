use crate::db::schema::{tissues};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
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

impl From<Tissue> for capra::deco::tissue::Tissue {
    fn from(value: Tissue) -> Self {
        let n2: [f64; capra::deco::TISSUE_COUNT] = [
            value.n2_1, value.n2_2, value.n2_3, value.n2_4,
            value.n2_5, value.n2_6, value.n2_7, value.n2_8,
            value.n2_9, value.n2_10, value.n2_11, value.n2_12,
            value.n2_13, value.n2_14, value.n2_15, value.n2_16,
        ];

        let he: [f64; capra::deco::TISSUE_COUNT] = [
            value.he_1, value.he_2, value.he_3, value.he_4,
            value.he_5, value.he_6, value.he_7, value.he_8,
            value.he_9, value.he_10, value.he_11, value.he_12,
            value.he_13, value.he_14, value.he_15, value.he_16,
        ];
        
        let mut t: [f64; capra::deco::TISSUE_COUNT] = [0.0; capra::deco::TISSUE_COUNT];
        for (n, h, t) in n2
            .iter()
            .zip(he.iter())
            .zip(t.iter_mut())
            .map(|((a, b), c)| (a, b, c))
        {
            *t = n+h;
        }

        Self::new(
            n2,
            he,
            t
        )
    }
}