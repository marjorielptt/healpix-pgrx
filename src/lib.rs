use pgrx::prelude::*; // default

// for nested::center + nested::from_uniq + nested::from_zuniq
use serde::{Serialize, Deserialize};

// for nested::siblings + nested::children
use pgrx::datum::Range;
use std::ops::RangeInclusive;

::pgrx::pg_module_magic!();

mod bmoc;
mod tests;
mod moc;

// HEALPix functions

// -------------------------------------------------- hash --------------------------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
#[inline]
/// Original signature : pub fn hash(depth: u8, lon: f64, lat: f64) -> u64
pub fn hpx_hash(depth: i32, lon:f64, lat:f64) -> i64 {
  cdshealpix::nested::hash(depth as u8, lon.to_radians(), lat.to_radians()) as i64
}

#[pg_extern(immutable, parallel_safe)]
#[inline]
/// Test
pub fn hpx_hash_range(depth: i32, lon:f64, lat:f64) -> pgrx::datum::Range<i64> {
  let hash_value: i64 = cdshealpix::nested::hash(depth as u8, lon.to_radians(), lat.to_radians()) as i64;
  pgrx::datum::Range::<i64>::new(hash_value, hash_value + 1 )
}

// -------------------------------------------------- best_starting_depth -----------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
#[inline]
/// Original signature : pub fn best_starting_depth(d_max_rad: f64) -> u8
pub fn hpx_best_starting_depth(d_max_rad: f64) -> f64 {
    cdshealpix::best_starting_depth(d_max_rad) as f64
}

// -------------------------------------------------- nside --------------------------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
#[inline]
// Original signature : pub fn hpx_nside(depth: u8) -> u32
pub fn hpx_nside(depth: i8) -> f64 {
  cdshealpix::nside(depth as u8) as f64
}

// -------------------------------------------------- nested::center -----------------------------------------------------------------
// Creation of a Coo type to replace Rust's tuple type because Postgres doesn't deal with tuples
#[derive(PostgresType, Serialize, Deserialize)]
pub struct Coo {
    pub lon_rad: f64,
    pub lat_rad: f64,
}

impl From<(f64, f64)> for Coo {
  fn from(item: (f64, f64)) -> Coo {
    Coo {lon_rad:item.0, lat_rad:item.1}
  }
}

#[pg_extern(immutable, parallel_safe)]
#[inline]
// Original signature : pub fn center(depth: u8, hash: u64) -> (f64, f64)
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_center(depth: i32, hash: i64) -> Coo {
  cdshealpix::nested::center(depth as u8, hash as u64).into()
}

// -------------------------------------------------- nested::parent -----------------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
// Original signature : pub const fn parent(hash: u64, delta_depth: u8) -> u64
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub const fn hpx_parent(hash: i64, delta_depth: i32) -> i64 {
  cdshealpix::nested::parent(hash as u64, delta_depth as u8) as i64
}

// -------------------------------------------------- nested::siblings ---------------------------------------------------------------
// std::ops::RangeInclusive<u64> and pgrx::datum::Range<i64> are both foreign traits so the From implementation doesn't work

// Solution : Creation of a RangeInclusiveCurrentCrate struct in the current crate to be able to implement From with
// the known type RangeInclusiveCurrentCrate and the foreign trait Range
pub struct RangeInclusiveCurrentCrate(pub RangeInclusive<u64>);

impl From<RangeInclusiveCurrentCrate> for Range<i64> {
  fn from(item: RangeInclusiveCurrentCrate) -> Range<i64> {
    let lower_bound = item.0.start();
    let upper_bound = item.0.end();
    Range::<i64>::new(*lower_bound as i64, *upper_bound as i64)
  }
}

#[pg_extern(immutable, parallel_safe)]
// Original signature : pub const fn siblings(depth: u8, hash: u64) -> RangeInclusive<u64>
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_siblings(depth: i32, hash: i64) -> Range<i64> {
  RangeInclusiveCurrentCrate(cdshealpix::nested::siblings(depth as u8, hash as u64)).into()
}

// -------------------------------------------------- nested::children ---------------------------------------------------------------
// std::ops::Range<u64> and pgrx::datum::Range<i64> are both foreign traits so the From implementation doesn't work

// Solution : Creation of a RangeCurrentCrate struct in the current crate to be able to implement From with
// the known type RangeCurrentCrate and the foreign trait pgrx::datum::Range
pub struct RangeCurrentCrate(pub std::ops::Range<u64>);

impl From<RangeCurrentCrate> for pgrx::datum::Range<i64> {
  fn from(item: RangeCurrentCrate) -> pgrx::datum::Range<i64> {
    let lower_bound = item.0.start;
    let upper_bound = item.0.end;
    pgrx::datum::Range::<i64>::new(lower_bound as i64, RangeBound::Exclusive(upper_bound as i64))
  }
}

#[pg_extern(immutable, parallel_safe)]
// Original signature : pub const fn children(hash: u64, delta_depth: u8) -> RangeInclusive<u64>
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_children(hash: i64, delta_depth: i32) -> pgrx::datum::Range<i64> {
  RangeCurrentCrate(cdshealpix::nested::children(hash as u64, delta_depth as u8)).into()
}

// -------------------------------------------------- nested::to_uniq ----------------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
#[inline]
// Original signature : pub fn to_uniq(depth: u8, hash: u64) -> u64
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_to_uniq(depth: i32, hash: i64) -> i64 {
  cdshealpix::nested::to_uniq(depth as u8, hash as u64) as i64
}

// -------------------------------------------------- nested::to_zuniq ----------------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
// Original signature : pub fn to_zuniq(depth: u8, hash: u64) -> u64
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_to_zuniq(depth: i32, hash: i64) -> i64 {
  cdshealpix::nested::to_zuniq(depth as u8, hash as u64) as i64
}

// -------------------------------------------------- nested::from_uniq ----------------------------------------------------------------
// Creation of a UniqTuple type to replace Rust's tuple type because Postgres doesn't deal with tuples
#[derive(PostgresType, Serialize, Deserialize)]
pub struct UniqTuple {
    pub depth: i32,
    pub hash: i64,
}

impl From<(u8, u64)> for UniqTuple {
  fn from(item: (u8, u64)) -> UniqTuple {
    UniqTuple {depth:item.0 as i32, hash:item.1 as i64}
  }
}

#[pg_extern(immutable, parallel_safe)]
// Original signature : pub const fn from_uniq(uniq_hash: u64) -> (u8, u64)
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_from_uniq(uniq_hash: i64) -> UniqTuple {
  cdshealpix::nested::from_uniq(uniq_hash as u64).into()
}

// -------------------------------------------------- nested::from_zuniq ---------------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
// Original signature : pub const fn from_zuniq(zuniq: u64) -> (u8, u64)
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_from_zuniq(zuniq: i64) -> UniqTuple {
  cdshealpix::nested::from_zuniq(zuniq as u64).into()
}

// -------------------------------------------------- nested::external_edge -------------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
// Original signature : pub fn external_edge(depth: u8, hash: u64, delta_depth: u8) -> Box<[u64]> 
pub fn hpx_external_edge(depth: i32, hash: i64, delta_depth: i32) -> Vec<i64> {
  let vec_u64: Vec<u64> = cdshealpix::nested::external_edge(depth as u8, hash as u64, delta_depth as u8).into_vec();
  unsafe { std::mem::transmute::<Vec<u64>, Vec<i64>>(vec_u64) }
}

// -------------------------------------------------- nested::internal_edge --------------------------------------------------------------
#[pg_extern(immutable, parallel_safe)]
// Original signature : pub fn external_edge(depth: u8, hash: u64, delta_depth: u8) -> Box<[u64]> 
pub fn hpx_internal_edge(depth: i32, hash: i64, delta_depth: i32) -> Vec<i64> {
  let vec_u64: Vec<u64> = cdshealpix::nested::internal_edge(depth as u8, hash as u64, delta_depth as u8).into_vec();
  unsafe { std::mem::transmute::<Vec<u64>, Vec<i64>>(vec_u64) }
}

// -------------------------------------------------- nested::neighbours -----------------------------------------------------------------
#[derive(PostgresType, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MainWindMapPSQL {
  array: [Option<i64>; 9],
}

impl From<cdshealpix::compass_point::MainWindMap<u64>> for MainWindMapPSQL {
  fn from(item: cdshealpix::compass_point::MainWindMap<u64>) -> Self {
    let mut array: [Option<i64>; 9] = [None; 9];
    for (mw, val) in item.entries_vec() {
      array[mw as usize] = Some(val as i64);
    }
    MainWindMapPSQL { array }
  }
}

#[pg_extern(immutable, parallel_safe)]
#[inline]
// Original signature : pub fn neighbours(depth: u8, hash: u64, include_center: bool) -> MainWindMap<u64>
pub fn hpx_neighbours(depth: i32, hash: i64, include_center: bool) -> MainWindMapPSQL {
  cdshealpix::nested::neighbours(depth as u8, hash as u64, include_center).into()
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
