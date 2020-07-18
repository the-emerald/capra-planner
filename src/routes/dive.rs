use actix_web::{post, web, HttpResponse};
use crate::{DBPool, json_repr};
use crate::simplified::SimplifiedDive;
use crate::db::actions::dive::{DiveType, add_dive};
use crate::db::actions::dive::DiveType::{EXECUTE};
use serde::{Serialize, Deserialize};
use std::convert::TryInto;
use crate::result::ServerDivePlanningError;
use crate::db::actions::tissue::{get_tissue_of_user, update_tissue_of_user};
use crate::db::actions::user::get_user_by_id;
use crate::db::models::user::User;
use capra::deco::zhl16::util::{ZHL16B_N2_A, ZHL16B_N2_B, ZHL16B_N2_HALFLIFE, ZHL16B_HE_A, ZHL16B_HE_B, ZHL16B_HE_HALFLIFE};
use capra::deco::deco_algorithm::DecoAlgorithm;
use crate::db::actions::settings::{get_zhl_settings_for_user, get_general_settings_for_user};
use capra::planning::modes::open_circuit::OpenCircuit;
use capra::planning::DivePlan;
use capra::common::DENSITY_SALTWATER;

// TODO: Consider moving this to capra crate
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(crate) enum Algorithm {
    ZHL16,
    VPM
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct DiveRouteOutput {
    segments: Vec<(json_repr::dive_segment::DiveSegment, json_repr::gas::Gas)>,
    // gas_used: HashMap<json_repr::gas::Gas, usize>,
    gas_used: Vec<(json_repr::gas::Gas, usize)>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct DiveRouteInput {
    id: i32,
    dive_type: DiveType,
    algorithm: Algorithm,
    parameters: SimplifiedDive
}

#[post("/dive/")]
pub(crate) async fn dive_route(
    pool: web::Data<DBPool>,
    form: web::Json<DiveRouteInput>
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    // Get user
    let id = form.id;
    let user = web::block(move || -> Result<User, diesel::result::Error>{
        get_user_by_id(id, &conn)?
            .ok_or(diesel::result::Error::NotFound)
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    // Re-acquire connection
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    // Get user tissue
    let tu = user.clone();
    let tissue = web::block(move || {
        get_tissue_of_user(&tu, &conn)
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match form.algorithm {
        Algorithm::ZHL16 => {
            // Get ZHL16 settings
            let conn = pool
                .get()
                .map_err(|_| HttpResponse::InternalServerError().finish())?;

            let zsu = user.clone();
            let zs = web::block(move || {
                get_zhl_settings_for_user(&zsu, &conn)
            })
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

            let zhl = capra::deco::zhl16::ZHL16::new(
                tissue.into(),
                ZHL16B_N2_A,
                ZHL16B_N2_B,
                ZHL16B_N2_HALFLIFE,
                ZHL16B_HE_A,
                ZHL16B_HE_B,
                ZHL16B_HE_HALFLIFE,
                zs.gfl as usize, zs.gfh as usize);

            dive(pool, form, user, zhl).await
        },
        Algorithm::VPM => {
            Ok(HttpResponse::NotImplemented().finish())
        },
    }
}

async fn dive<T: DecoAlgorithm>(
    pool: web::Data<DBPool>,
    form: web::Json<DiveRouteInput>,
    user: User,
    deco: T
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    // Conversions
    let c_segs = form.parameters.segments
        .iter()
        .cloned()
        .map(|x|
            Ok(
                (x.0.try_into()?,
                 x.1.try_into()?)
            )
        )
        .collect::<Result<Vec<(capra::common::dive_segment::DiveSegment, capra::common::gas::Gas)>, ServerDivePlanningError>>()?;

    let c_dg = form.parameters.deco_gases
        .iter()
        .cloned()
        .map(|x|
            Ok(
                (x.try_into()?,
                x.max_op_depth)
            )
        )
        .collect::<Result<Vec<(capra::common::gas::Gas, Option<usize>)>, ServerDivePlanningError>>()?;

    let gsu = user.clone();
    let general_settings = web::block(move || {
        get_general_settings_for_user(&gsu, &conn)
    })
        .await
        .map_err(|e| {
            eprintln!("gs {}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let dive = OpenCircuit::new(
        deco,
        &c_dg,
        &c_segs,
        general_settings.ascent_rate as isize,
        general_settings.descent_rate as isize,
        DENSITY_SALTWATER,
        general_settings.sac_bottom as usize,
        general_settings.sac_deco as usize
    );

    let res = dive.plan();

    let out = DiveRouteOutput {
        segments: {
            res.total_segments()
                .into_iter()
                .map(|x| (x.0.into(), x.1.into()))
                .collect()
        },
        gas_used: {
            res.gas_used()
                .into_iter()
                .map(|(k, v)| (json_repr::gas::Gas::from(*k), *v))
                .collect()
        }
    };

    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let adu = user.clone();
    let sd = form.clone();
    web::block(move || {
        add_dive(
            &adu,
            &sd.parameters,
            &sd.dive_type,
            &conn
        )
    })
        .await
        .map_err(|e| {
            eprintln!("dv {}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if form.dive_type == EXECUTE {
        // Update tissue as well
        let conn = pool
            .get()
            .map_err(|_| HttpResponse::InternalServerError().finish())?;

        let tissue = res.deco_algorithm().get_tissue();
        web::block(move || {
            update_tissue_of_user(&user, tissue.into(), &conn)
        })
            .await
            .map_err(|e| {
                eprintln!("ts {}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    }

    Ok(HttpResponse::Ok().json(out))
}