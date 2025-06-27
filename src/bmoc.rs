use pgrx::prelude::*; // default

use serde::{Serialize, Deserialize};

// For contains
use cdshealpix::nested::bmoc::Status;

// For the operations with the BMOCs
use cdshealpix::nested::bmoc::BMOC;

// For the ranges representation
use std::ops::RangeInclusive;

// use cdshealpix::sph_geom::coo3d::{UnitVec3, Vec3};
// 
// extern crate cdshealpix;
// 
// use cdshealpix::sph_geom::{cone, elliptical_cone, zone};
// use cdshealpix::xy_geom::ellipse;

#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BMOCpsql {
    pub depth_max: i32,
    pub entries: Vec<i64>,
}

// BMOC -> BMOCpsql
impl From<BMOC> for BMOCpsql {
  fn from(item: BMOC) -> Self {
    let entries_vec_i64 = unsafe {
        let entries_vec_u64 = item.entries.to_vec();
        std::mem::transmute::<Vec<u64>, Vec<i64>>(entries_vec_u64)
    };
    BMOCpsql {depth_max:item.get_depth_max() as i32, entries: entries_vec_i64}
  }
}

// BMOCpsql -> BMOC
impl From<BMOCpsql> for BMOC {
  fn from(item: BMOCpsql) -> Self {
    let entries_vec_i64 = item.entries.to_vec();
    let entries_vec_u64: Vec<u64> = unsafe {
        std::mem::transmute::<Vec<i64>, Vec<u64>>(entries_vec_i64)
    };
    
    BMOC::create_unsafe(item.depth_max as u8, entries_vec_u64.into_boxed_slice())
  }
}

//  ------------------------------------------------ Contains -----------------------------------------------

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

#[derive(PostgresEnum, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Statuspsql {
    IN,
    OUT,
    UNKNOWN,
}

impl From<Status> for Statuspsql {
  fn from(item: Status) -> Self {
    match item {
        Status::IN => Statuspsql::IN,
        Status::OUT => Statuspsql::OUT,
        Status::UNKNOWN => Statuspsql::UNKNOWN,
    }
  }
}

#[pg_extern(immutable, parallel_safe)]
pub fn hpx_contains(bmoc: BMOCpsql, lon: f64, lat:f64) -> Statuspsql {
    BMOC::from(bmoc).test_coo(lon, lat).into()
}

// ------------------------------------------------ Operations -----------------------------------------------

// Not
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_not(bmoc: BMOCpsql) -> BMOCpsql {
    BMOC::from(bmoc).not().into()
}

// And
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_and(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    BMOC::from(bmoc).and(&BMOC::from(other)).into()
}

// Or
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_or(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    BMOC::from(bmoc).or(&BMOC::from(other)).into()
}

// Xor
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_xor(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    BMOC::from(bmoc).xor(&BMOC::from(other)).into()
}

// ------------------------------------------- Ranges representation -----------------------------------------

#[derive(PostgresType, Debug, Serialize, Deserialize)]
pub struct VecRangei64 {
  pub vec_field: Vec<RangeInclusive<i64>>
}

impl From<Box<[std::ops::Range<u64>]>> for VecRangei64 {
  fn from(item: Box<[std::ops::Range<u64>]>) -> Self {
    let vec_range_u64 = item.into_vec();
    let mut vec_range_i64: VecRangei64 = VecRangei64{ vec_field: Vec::new()};
    for range_u64 in vec_range_u64 {
        let lower_bound = range_u64.start;
        let upper_bound = range_u64.end;
        let range_i64 = RangeInclusive::new(lower_bound as i64,upper_bound as i64);
        vec_range_i64.vec_field.push(range_i64);
    }
    vec_range_i64
  }
}

// Returns a vector of ranges at the maximal depth (=29)
#[pg_extern]
pub fn hpx_to_ranges(bmoc: BMOCpsql) -> VecRangei64 {
    BMOC::from(bmoc).to_ranges().into()
}