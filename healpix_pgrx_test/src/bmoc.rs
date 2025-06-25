use pgrx::prelude::*; // default

use serde::{Serialize, Deserialize};

// For the operations with the BMOCs
use cdshealpix::nested::bmoc::BMOC;

// For contains implementations
// use cdshealpix::sph_geom::coo3d::{UnitVec3, Vec3};
// 
// extern crate cdshealpix;
// 
// use cdshealpix::sph_geom::{cone, elliptical_cone, zone};
// use cdshealpix::xy_geom::ellipse;

#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BMOCpsql {
    depth_max: i32,
    pub entries: Vec<i64>,
}

impl From<BMOC> for BMOCpsql {
  fn from(item: BMOC) -> BMOCpsql {
    let entries_vec_u64 = item.entries.clone().into_vec();
    let entries_vec_i64 = unsafe {std::mem::transmute::<Vec<u64>, Vec<i64>>(entries_vec_u64)};
    BMOCpsql {depth_max:item.get_depth_max() as i32, entries: entries_vec_i64}
  }
}

// ------------------------------------------- Contains -------------------------------------------

// Cone 
// pub fn hpx_cone_contains<T: Vec3 + UnitVec3>(cone_obj: cone::Cone, point: &T) -> bool {
//     cone.cdshealpix::sph_geom::contains(point)
// }
// 
// // EllipticalCone
// pub fn hpx_elliptical_cone_contains(elliptical_cone_obj: elliptical_cone::EllipticalCone, lon:f64, lat:f64) -> bool {
//     elliptical_cone.cdshealpix::sph_geom::contains(lon, lat)
// }
// 
// // Zone
// pub fn hpx_zone_contains(zone_obj: zone::Zone, lon:f64, lat:f64) -> bool {
//     zone.cdshealpix::sph_geom::contains(lon, lat)
// }
// 
// // Ellipse
// pub fn hpx_ellipse_contains(ellipse_obj: ellipse::Ellipse, x:f64, y:f64) -> bool {
//     ellipse.cdshealpix::xy_geom::contains(x,y)
// }

// ------------------------------------------- Operations -----------------------------------------

// Not
pub fn hpx_not(bmoc: BMOC) -> BMOCpsql {
    bmoc.not().into()
}

// And
pub fn hpx_and(bmoc: BMOC, other: &BMOC) -> BMOCpsql {
    bmoc.and(&other).into()
}

// Or
pub fn hpx_or(bmoc: BMOC, other: &BMOC) -> BMOCpsql {
    bmoc.or(&other).into()
}

// Xor
pub fn hpx_xor(bmoc: BMOC, other: &BMOC) -> BMOCpsql {
    bmoc.xor(&other).into()
}

