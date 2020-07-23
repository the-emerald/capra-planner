use actix_web::{post, web, HttpResponse};
use crate::{DBPool, json_repr};
use crate::simplified::SimplifiedDive;
use crate::db::actions::dive::{PlanType, add_dive};
use crate::db::actions::dive::PlanType::{EXECUTE};
use serde::{Serialize, Deserialize};
use std::convert::TryInto;
use crate::result::ServerDivePlanningError;
use crate::db::actions::tissue::{get_tissue_of_user, update_tissue_of_user};
use crate::db::actions::user::get_user_by_id;
use crate::db::models::user::User;
use capra::deco::deco_algorithm::DecoAlgorithm;
use crate::db::actions::settings::{get_zhl_settings_for_user, get_general_settings_for_user};
use capra::planning::modes::open_circuit::OpenCircuit;
use capra::planning::DivePlan;
use capra::common::DENSITY_SALTWATER;
use capra::common::dive_segment::SegmentType::DiveSegment;
use time::Duration;
use std::cmp::Ordering;
use crate::db::models::settings::ZHLSettings;

// TODO: Consider moving this to capra crate
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(crate) enum Algorithm {
    ZHL16,
    VPM
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct DiveRouteOutput {
    segments: Vec<(json_repr::dive_segment::DiveSegment, json_repr::gas::Gas)>,
    gas_used: Vec<(json_repr::gas::Gas, usize)>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct DiveRouteInput {
    id: i32,
    surface_interval: u64, // Milliseconds
    plan_type: PlanType,
    algorithm: Algorithm,
    parameters: SimplifiedDive
}

#[post("/dive/")]
pub(crate) async fn dive_route(
    pool: web::Data<DBPool>,
    form: web::Json<DiveRouteInput>
) -> actix_web::Result<HttpResponse> {
    // Get user
    let user = get_user_by_id_blocking(pool.clone(), form.id).await?;

    // Get user tissue
    let tissue = get_tissue_of_user_blocking(pool.clone(), user.clone()).await?.into();

    match form.algorithm {
        Algorithm::ZHL16 => {
            // Get ZHL16 settings
            let zs = get_zhl_setting_of_user_blocking(pool.clone(), user.clone()).await?;

            let zhl = match zs.variant.as_str() {
                "B" => {
                    capra::deco::zhl16::ZHL16::new_by_variant(
                        tissue,
                        zs.gfl as usize, zs.gfh as usize,
                        capra::deco::zhl16::variant::Variant::B,
                    )
                },
                "C" => {
                    capra::deco::zhl16::ZHL16::new_by_variant(
                        tissue,
                        zs.gfl as usize, zs.gfh as usize,
                        capra::deco::zhl16::variant::Variant::C,
                    )
                },
                _ => {
                    return Ok(HttpResponse::InternalServerError().finish());
                }
            };

            // let si = form.surface_interval;
            let zhl = match form.surface_interval.cmp(&(0 as u64)) {
                Ordering::Equal => {
                    zhl
                },
                _ => {
                    let new_t = add_surface_interval(
                        zhl,
                        Duration::milliseconds(form.surface_interval as i64)
                    )?;
                    match zs.variant.as_str() {
                        "B" => {
                            capra::deco::zhl16::ZHL16::new_by_variant(
                                new_t,
                                zs.gfl as usize, zs.gfh as usize,
                                capra::deco::zhl16::variant::Variant::B,
                            )
                        },
                        "C" => {
                            capra::deco::zhl16::ZHL16::new_by_variant(
                                new_t,
                                zs.gfl as usize, zs.gfh as usize,
                                capra::deco::zhl16::variant::Variant::C,
                            )
                        },
                        _ => {
                            return Ok(HttpResponse::InternalServerError().finish());
                        }
                    }
                },
            };

            dive(pool, form, user, zhl).await
        },
        Algorithm::VPM => {
            Ok(HttpResponse::NotImplemented().finish())
        },
    }
}

async fn get_zhl_setting_of_user_blocking(pool: web::Data<DBPool>, user: User) -> Result<ZHLSettings, HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    web::block(move || {
        get_zhl_settings_for_user(&user, &conn)
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
}

async fn get_tissue_of_user_blocking(pool: web::Data<DBPool>, user: User)
    -> Result<crate::db::models::tissue::Tissue, HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    web::block(move || {
        get_tissue_of_user(&user, &conn)
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
}

async fn get_user_by_id_blocking(pool: web::Data<DBPool>, id: i32) -> Result<User, HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    web::block(move || -> Result<User, diesel::result::Error>{
        get_user_by_id(id, &conn)?
            .ok_or(diesel::result::Error::NotFound)
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
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
            eprintln!("{}", e);
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
            &sd.plan_type,
            &conn
        )
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if form.plan_type == EXECUTE {
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
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
    }

    Ok(HttpResponse::Ok().json(out))
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SurfaceIntervalInput {
    pub(crate) id: i32,
    pub(crate) algorithm: Algorithm,
    pub(crate) duration: u64 // Milliseconds
}

#[post("/dive/si")]
pub(crate) async fn surface_interval(
    pool: web::Data<DBPool>,
    form: web::Json<SurfaceIntervalInput>,
) -> actix_web::Result<HttpResponse> {
    let user = get_user_by_id_blocking(pool.clone(), form.id).await?;
    let tissue = get_tissue_of_user_blocking(pool.clone(), user.clone()).await?.into();

    let after_si_tissue = match form.algorithm {
        Algorithm::ZHL16 => {
            let zs = get_zhl_setting_of_user_blocking(pool.clone(), user.clone()).await?;

            let z = match zs.variant.as_str() {
                "B" => {
                    capra::deco::zhl16::ZHL16::new_by_variant(
                        tissue,
                        100, 100,
                        capra::deco::zhl16::variant::Variant::B,
                    )
                },
                "C" => {
                    capra::deco::zhl16::ZHL16::new_by_variant(
                        tissue,
                        100, 100,
                        capra::deco::zhl16::variant::Variant::C,
                    )
                },
                _ => { return Ok(HttpResponse::InternalServerError().finish()) }
            };

            add_surface_interval(z, Duration::milliseconds(form.duration as i64))?
        },
        Algorithm::VPM => {
            return Ok(HttpResponse::NotImplemented().finish())
        },
    };

    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    web::block(move || {
        update_tissue_of_user(&user, after_si_tissue.into(), &conn)
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())

}

fn add_surface_interval<T: DecoAlgorithm>(mut deco: T, duration: Duration) -> actix_web::Result<capra::deco::tissue::Tissue> {
    let air = capra::common::gas::Gas::new(21, 0, 79).unwrap();

    let surface_interval_segment = capra::common::dive_segment::DiveSegment::new(
        DiveSegment,
        0,
        0,
        duration,
        1, 1
    )
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    deco.add_dive_segment(&surface_interval_segment, &air, 10000.0);

    Ok(deco.get_tissue())
}