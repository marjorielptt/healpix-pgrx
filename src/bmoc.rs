use pgrx::prelude::*; // default

use serde::{Serialize, Deserialize};

// For the operations with the BMOCs
use cdshealpix::nested::bmoc::BMOC;

// For contains
use cdshealpix::nested::bmoc::Status;

// BMOC type that is PSQL compatible
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

//  ----------------------------------- Coverage types conversions to BMOC ------------------------------------

// Cone 
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_cone_coverage_approx(depth: i32, cone_lon: f64, cone_lat:f64, cone_radius: f64) -> BMOCpsql {
  cdshealpix::nested::cone_coverage_approx(depth as u8, cone_lon, cone_lat, cone_radius).into()
}
 
// EllipticalCone
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_elliptical_cone_coverage(depth: i32, lon: f64, lat: f64, a: f64, b: f64, pa: f64) -> BMOCpsql {
  cdshealpix::nested::elliptical_cone_coverage(depth as u8, lon, lat, a, b, pa).into()
}

// Zone
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_zone_coverage(depth: i32, lon_min: f64, lat_min: f64, lon_max: f64, lat_max: f64) -> BMOCpsql {
  cdshealpix::nested::zone_coverage(depth as u8, lon_min, lat_min, lon_max, lat_max).into()
}

// Type created to adapt the Rust vertex tuple (f64, f64) to PSQL for polygon_coverage
#[derive(PostgresType, Serialize, Deserialize)]
pub struct Vertexpsql(f64, f64);

impl From<Vertexpsql> for (f64,f64) {
  fn from(item: Vertexpsql) -> Self {
    (item.0, item.1)
  }
}

// Polygon
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_polygon_coverage(depth: i32, vertices: Vec<Vertexpsql>, exact_solution: bool) -> BMOCpsql {
  let mut vertices_tuple: Vec<(f64,f64)> = Vec::new();
  for vertex in vertices {
    vertices_tuple.push(vertex.into());
  }
  let vertices_as_array = vertices_tuple.as_slice();
  cdshealpix::nested::polygon_coverage(depth as u8, vertices_as_array, exact_solution).into()
}

// Box
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_box_coverage(depth: i32, lon: f64, lat: f64, a: f64, b: f64, pa: f64) -> BMOCpsql {
  cdshealpix::nested::box_coverage(depth as u8, lon, lat, a, b, pa).into()
}

// Ring
#[pg_extern(immutable, parallel_safe)]
pub fn hpx_ring_coverage_approx(depth: i32, cone_lon: f64, cone_lat: f64, cone_radius_int: f64, cone_radius_ext: f64) -> BMOCpsql {
  cdshealpix::nested::ring_coverage_approx(depth as u8, cone_lon, cone_lat, cone_radius_int, cone_radius_ext).into()
}

// ------------------------------------------------ Contains -----------------------------------------------

// Status type that is PSQL compatible
#[derive(PostgresEnum, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Statuspsql {
    IN,
    OUT,
    UNKNOWN,
}

// Status -> Statuspsql
impl From<Status> for Statuspsql {
  fn from(item: Status) -> Self {
    match item {
        Status::IN => Statuspsql::IN,
        Status::OUT => Statuspsql::OUT,
        Status::UNKNOWN => Statuspsql::UNKNOWN,
    }
  }
}

// Contains
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

// #[derive(PostgresType, Debug, Serialize, Deserialize)]
// pub struct VecRangei64 {
//   pub vec_field: Vec<RangeInclusive<i64>>
// }
// 
// impl From<Box<[std::ops::Range<u64>]>> for VecRangei64 {
//   fn from(item: Box<[std::ops::Range<u64>]>) -> Self {
//     let vec_range_u64 = item.into_vec();
//     let mut vec_range_i64: VecRangei64 = VecRangei64{ vec_field: Vec::new()};
//     for range_u64 in vec_range_u64 {
//         let lower_bound = range_u64.start;
//         let upper_bound = range_u64.end;
//         let range_i64 = RangeInclusive::new(lower_bound as i64,upper_bound as i64);
//         vec_range_i64.vec_field.push(range_i64);
//     }
//     vec_range_i64
//   }
// }

// Returns a vector of ranges at the maximal depth (=29)
#[pg_extern]
pub fn hpx_to_ranges(bmoc: BMOCpsql) -> Vec<pgrx::datum::Range<i64>> {
    let vec_range_u64: Vec<std::ops::Range<u64>> = BMOC::from(bmoc).to_ranges().into_vec();
    let mut vec_range_i64: Vec<pgrx::datum::Range<i64>> = Vec::new();
    for range_u64 in vec_range_u64 {
      let lower_bound = range_u64.start;
      let upper_bound = range_u64.end;
      let range_i64 = pgrx::datum::Range::new(lower_bound as i64, upper_bound as i64);
      vec_range_i64.push(range_i64);
    }
    vec_range_i64
}

// Returns a vector of the ranges with flag=0 only
