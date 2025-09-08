use pgrx::prelude::*; // default

use serde::{Serialize, Deserialize};

// For the BMOC creations
use cdshealpix::nested::bmoc::BMOCBuilderUnsafe;

// For the operations with the BMOCs
use cdshealpix::nested::bmoc::BMOC;

// For contains
use cdshealpix::nested::bmoc::Status;

// For the redefinition of the operators' behavior
use std::ops::Not;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;

use std::ops::Range as StdRange;
use pgrx::datum::Range as PgRange;

use skyregion::{
  regions::{
    cone::Cone,
    ellipse::EllipticalCone,
    zone::Zone,
    polygon::Polygon,
    ring::Ring,
  },
  SkyRegion
};

// Creation of a StdRange type that is in the current crate to satisfy the orphan rule 
pub struct StdRangeCrate(pub std::ops::Range<u64>);

// PgRange<i64> -> StdRangeCrate<u64>
impl From<PgRange<i64>> for StdRangeCrate {
    fn from(item: PgRange<i64>) -> StdRangeCrate {
        let start: u64 = match item.lower() {
            Some(&RangeBound::Exclusive(lower_bound)) => lower_bound as u64 + 1,
            Some(&RangeBound::Inclusive(lower_bound)) => lower_bound as u64,
            Some(RangeBound::Infinite) => panic!("Infinite RangeBound"),
            None => panic!("No RangeBound"),
        };
        let end: u64 = match item.upper() {
            Some(&RangeBound::Exclusive(upper_bound)) => upper_bound as u64,
            Some(&RangeBound::Inclusive(upper_bound)) => upper_bound as u64 + 1,
            Some(RangeBound::Infinite) => panic!("Infinite RangeBound"),
            None => panic!("No RangeBound"),
        };
        StdRangeCrate( StdRange {start, end})
    }
}

// StdRangeCrate<i64> -> PgRange<i64>
impl From<StdRangeCrate> for PgRange<i64> {
    fn from(item: StdRangeCrate) -> PgRange<i64> {
        PgRange::<i64>::new(item.0.start as i64, item.0.end as i64 - 1)
    }
}

// BMOC type that is PSQL compatible
#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BMOCpsql {
    pub depth_max: i32,
    pub entries: Vec<i64>,
}

// Creation of a BMOC
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_create_bmoc_psql(depth_max: i32, entries: Vec<i64>) -> BMOCpsql {
    BMOCpsql { depth_max, entries }
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

// UNNECESSARY : We use mgx_hash_range
//
// // Provides the part of the query that turns the ranges into betweens
// // Form : element BETWEEN ... AND ... OR element BETWEEN ... AND ... 
// #[pg_extern(immutable, parallel_safe)]
// pub fn bmoc_to_between(element: String, bmoc: BMOCpsql) -> String {
//     let mut res = String::new();
//     let hpx_bmoc: BMOC = bmoc.into();
//     let flagged_ranges: Vec<(StdRange<u64>, bool)> = hpx_bmoc.to_flagged_ranges();
//     let len = flagged_ranges.len();
// 
//     for (i, (r, _)) in flagged_ranges.iter().enumerate() {
//         res += &format!("{} BETWEEN {} AND {}", element, r.start, r.end);
//         if i < len - 1 {
//             res += " OR ";
//         }
//     }
//     res
// }
// 
// // Provides the complete query that returns the bmocs that contain the element in at least one of their ranges
// // Form : SELECT * FROM table WHERE element BETWEEN ... AND ... OR element BETWEEN ... AND ...;
// #[pg_extern(immutable, parallel_safe)]
// pub fn bmoc_contains_element_query(table: String, element: String, bmoc: BMOCpsql) -> String {
//     format!("SELECT * FROM {} WHERE {};", table, bmoc_to_between(element, bmoc))
// }

//  ----------------------- Creation of a BMOC from different coverage types -------------------------------

// Cone 
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_cone_coverage_approx(depth: i32, cone_lon: f64, cone_lat:f64, cone_radius: f64) -> BMOCpsql {
  cdshealpix::nested::cone_coverage_approx(depth as u8, cone_lon.to_radians(), cone_lat.to_radians(), cone_radius.to_radians()).into()
}
 
// EllipticalCone
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_elliptical_cone_coverage(depth: i32, lon: f64, lat: f64, a: f64, b: f64, pa: f64) -> BMOCpsql {
  cdshealpix::nested::elliptical_cone_coverage(depth as u8, lon.to_radians(), lat.to_radians(), a.to_radians(), b.to_radians(), pa.to_radians()).into()
}

// Zone
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_zone_coverage(depth: i32, lon_min: f64, lat_min: f64, lon_max: f64, lat_max: f64) -> BMOCpsql {
  cdshealpix::nested::zone_coverage(depth as u8, lon_min.to_radians(), lat_min.to_radians(), lon_max.to_radians(), lat_max.to_radians()).into()
}

// Type created to adapt the Rust vertex tuple (f64, f64) to PSQL for polygon_coverage
#[derive(PostgresType, Serialize, Deserialize)]
pub struct VertexPSQL(f64, f64);

impl From<VertexPSQL> for (f64,f64) {
  fn from(item: VertexPSQL) -> Self {
    (item.0, item.1)
  }
}

// Creation of a vertex (useful in Postgres)
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_create_vertexpsql(lon: f64, lat: f64) -> VertexPSQL {
  VertexPSQL(lon.to_radians(), lat.to_radians())
}

// Polygon
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_polygon_coverage(depth: i32, vertices: Vec<VertexPSQL>, exact_solution: bool) -> BMOCpsql {
  let mut vertices_tuple: Vec<(f64,f64)> = Vec::new();
  for vertex in vertices {
    vertices_tuple.push(vertex.into());
  }
  let vertices_as_array = vertices_tuple.as_slice();
  cdshealpix::nested::polygon_coverage(depth as u8, vertices_as_array, exact_solution).into()
}

// Box
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_box_coverage(depth: i32, lon: f64, lat: f64, a: f64, b: f64, pa: f64) -> BMOCpsql {
  cdshealpix::nested::box_coverage(depth as u8, lon.to_radians(), lat.to_radians(), a.to_radians(), b.to_radians(), pa.to_radians()).into()
}

// Ring
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_ring_coverage_approx(depth: i32, cone_lon: f64, cone_lat: f64, cone_radius_int: f64, cone_radius_ext: f64) -> BMOCpsql {
  cdshealpix::nested::ring_coverage_approx(depth as u8, cone_lon.to_radians(), cone_lat.to_radians(), cone_radius_int.to_radians(), cone_radius_ext.to_radians()).into()
}

// ------------------------------------------------ Contains -----------------------------------------------

// Status type that is PSQL compatible
#[derive(PostgresEnum, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Statuspsql {
    In,
    Out,
    Unknown,
}

// Status -> Statuspsql
impl From<Status> for Statuspsql {
  fn from(item: Status) -> Self {
    match item {
        Status::IN => Statuspsql::In,
        Status::OUT => Statuspsql::Out,
        Status::UNKNOWN => Statuspsql::Unknown,
    }
  }
}

// Contains
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_contains(bmoc: BMOCpsql, lon: f64, lat:f64) -> Statuspsql {
    BMOC::from(bmoc).test_coo(lon.to_radians(), lat.to_radians()).into()
}

// Contains
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_contains_bool(bmoc: BMOCpsql, lon: f64, lat:f64) -> bool {
    let status: Statuspsql = mgx_bmoc_contains(bmoc, lon, lat);
    match status {
      Statuspsql::In => true,
      Statuspsql::Out => false,
      Statuspsql::Unknown => true,
    }
}

// ------------------------------------------------ Operations -----------------------------------------------

// Not
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_not(bmoc: BMOCpsql) -> BMOCpsql {
    BMOC::from(bmoc).not().into()
}

// Redefinition of !'s behavior
impl Not for BMOCpsql {
  type Output = BMOCpsql;

  fn not(self) -> BMOCpsql {
    let bmoc = self;
    mgx_bmoc_not(bmoc)
  }
}

// And
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_and(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    BMOC::from(bmoc).and(&BMOC::from(other)).into()
}

// Redefinition of &'s behavior
impl BitAnd for BMOCpsql {
  type Output = BMOCpsql;

  fn bitand(self, other: BMOCpsql) -> BMOCpsql {
    let bmoc = self;
    mgx_bmoc_and(bmoc, other)
  }
}

// Redefinition of &'s behavior for Postgres utilisations
#[pg_operator]
#[opname(&)]
fn mgx_bmoc_pg_and(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    bmoc & other
}

// Or
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_or(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    BMOC::from(bmoc).or(&BMOC::from(other)).into()
}

// Redefinition of |'s behavior for Rust utilisations
impl BitOr for BMOCpsql {
  type Output = BMOCpsql;

  fn bitor(self, other: BMOCpsql) -> BMOCpsql {
    let bmoc = self;
    mgx_bmoc_or(bmoc, other)
  }
}

// Redefinition of |'s behavior for Postgres utilisations
#[pg_operator]
#[opname(|)]
fn mgx_pg_bmoc_or(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    bmoc | other
}

// Xor
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_xor(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    BMOC::from(bmoc).xor(&BMOC::from(other)).into()
}

// Redefinition of ^'s behavior for Rust utilisations
impl BitXor for BMOCpsql {
  type Output = BMOCpsql;

  fn bitxor(self, other: BMOCpsql) -> BMOCpsql {
    let bmoc = self;
    mgx_bmoc_xor(bmoc, other)
  }
}

// Redefinition of ^'s behavior for Postgres utilisations
#[pg_operator]
#[opname(^)]
fn mgx_pg_bmoc_xor(bmoc: BMOCpsql, other: BMOCpsql) -> BMOCpsql {
    bmoc ^ other
}

// ------------------------------------------- Ranges representation -----------------------------------------

// cdshealpix::nested::is_partial
pub fn mgx_is_partial(raw_value: &i64) -> bool {
  (*raw_value & 1_i64) == 0_i64
}

// Returns a vector of the ranges with flag=0 only
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_flag_zero(bmoc: BMOCpsql) -> Vec<PgRange<i64>> {
  let entries = bmoc.entries;
  let (flag0, _flag1): (Vec<i64>, Vec<i64>) = 
    entries.into_iter().partition(|cell| mgx_is_partial(cell));
  let bmoc_res = BMOCpsql{ depth_max: bmoc.depth_max, entries: flag0 };
  mgx_bmoc_to_ranges(bmoc_res)
}

// Returns a vector of the ranges with flag=1 only
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_flag_one(bmoc: BMOCpsql) -> Vec<PgRange<i64>> {
  let entries = bmoc.entries;
  let (_flag0, flag1): (Vec<i64>, Vec<i64>) = 
    entries.into_iter().partition(|cell| mgx_is_partial(cell));
  let bmoc_res = BMOCpsql{ depth_max: bmoc.depth_max, entries: flag1 };
  mgx_bmoc_to_ranges(bmoc_res)
}

// Returns a vector of ranges
// Warning : the ranges are not at the MOC depth, not a the depth 29
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_bmoc_to_ranges(bmoc: BMOCpsql) -> Vec<PgRange<i64>> {
    let depth_max = bmoc.depth_max;
    let std_bmoc: BMOC = bmoc.into();

    let vec_range_u64: Vec<StdRange<u64>> = std_bmoc.to_ranges().into_vec();
    let mut vec_range_i64: Vec<PgRange<i64>> = Vec::new();

    let shift = (29 - depth_max) << 1;
    for r in vec_range_u64 {
        vec_range_i64.push(StdRangeCrate((r.start << shift)..(r.end << shift)).into());
    }

    vec_range_i64
}

// ----------------------------------------------------- Skyregion::contains -----------------------------------------------------------

// Cone
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_skyregion_cone_contains(
    lon_deg: f64,
    lat_deg: f64,
    radius_deg: f64,
    test_lon_deg: f64,
    test_lat_deg: f64,
) -> bool {
    match Cone::from_deg(lon_deg, lat_deg, radius_deg) {
        Ok(cone) => {
            let test_lon = test_lon_deg.to_radians();
            let test_lat = test_lat_deg.to_radians();
            cone.contains(test_lon, test_lat)
        }
        Err(_) => false,
    }
}

// Elliptical cone
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_skyregion_elliptical_cone_contains(
    lon_deg: f64,
    lat_deg: f64,
    a_deg: f64,
    b_deg: f64,
    pa_deg: f64,
    test_lon_deg: f64,
    test_lat_deg: f64,
) -> bool {
    match EllipticalCone::from_deg(lon_deg, lat_deg, a_deg, b_deg, pa_deg) {
        Ok(elliptical_cone) => {
            let test_lon = test_lon_deg.to_radians();
            let test_lat = test_lat_deg.to_radians();

            elliptical_cone.contains(test_lon, test_lat)
        }
        Err(_) => false,
    }
}

// Zone
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_skyregion_zone_contains(
    lon_min_deg: f64,
    lat_min_deg: f64,
    lon_max_deg: f64,
    lat_max_deg: f64,
    test_lon_deg: f64,
    test_lat_deg: f64,
) -> bool {
    match Zone::from_deg(lon_min_deg, lat_min_deg, lon_max_deg, lat_max_deg) {
        Ok(zone) => {
            let test_lon = test_lon_deg.to_radians();
            let test_lat = test_lat_deg.to_radians();

            zone.contains(test_lon, test_lat)
        }
        Err(_) => false,
    }
}

// Like mgx_best_starting_depth but from the skyregion repository, needed for in_polygon
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_polygon_characteristic_depth(
    vertices_deg: Vec<VertexPSQL>,
    complement: bool,
) -> i32 {
    let std_vertices_deg: Vec<(f64, f64)> = unsafe {std::mem::transmute::<Vec<VertexPSQL>, Vec<(f64, f64)>>(vertices_deg)};
    match Polygon::from_deg(std_vertices_deg, complement) {
        Ok(polygon) => {
            polygon.characteristic_depth() as i32
        }
        Err(e) => error!("Failed to find the polygon's characteristic depth : {}", e),
    }
}

// Polygon
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_skyregion_polygon_contains(
    vertices_deg: Vec<VertexPSQL>,
    complement: bool,
    test_lon_deg: f64,
    test_lat_deg: f64,
) -> bool {
    let std_vertices_deg: Vec<(f64, f64)> = unsafe {std::mem::transmute::<Vec<VertexPSQL>, Vec<(f64, f64)>>(vertices_deg)};
    match Polygon::from_deg(std_vertices_deg, complement) {
        Ok(polygon) => {
            let test_lon = test_lon_deg.to_radians();
            let test_lat = test_lat_deg.to_radians();

            polygon.contains(test_lon, test_lat)
        }
        Err(_) => false,
    }
}

// Box
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_skyregion_box_contains(
    lon_deg: f64,
    lat_deg: f64,
    a_deg: f64,
    b_deg: f64,
    pa_deg: f64,
    test_lon_deg: f64,
    test_lat_deg: f64,
) -> bool {
    match Polygon::from_box_deg(lon_deg, lat_deg, a_deg, b_deg, pa_deg) {
        Ok(my_box) => {
            let test_lon = test_lon_deg.to_radians();
            let test_lat = test_lat_deg.to_radians();

            my_box.contains(test_lon, test_lat)
        }
        Err(_) => false,
    }
}

// Ring
#[pg_extern(immutable, parallel_safe)]
pub fn mgx_skyregion_ring_contains(
    lon_deg: f64,
    lat_deg: f64,
    r_min_deg: f64,
    r_max_deg: f64,
    test_lon_deg: f64,
    test_lat_deg: f64,
) -> bool {
    match Ring::from_deg(lon_deg, lat_deg, r_min_deg, r_max_deg) {
        Ok(ring) => {
            let test_lon = test_lon_deg.to_radians();
            let test_lat = test_lat_deg.to_radians();

            ring.contains(test_lon, test_lat)
        }
        Err(_) => false,
    }
}