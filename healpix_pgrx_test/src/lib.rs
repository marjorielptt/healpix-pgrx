use pgrx::prelude::*; // default

// for nested::center + nested::from_uniq + nested::from_zuniq
use serde::{Serialize, Deserialize};

// for nested::siblings + nested::children
use pgrx::datum::Range;
use std::ops::RangeInclusive;

::pgrx::pg_module_magic!();

// HEALPix functions

// -------------------------------------------------- hash --------------------------------------------------------------------------
#[pg_extern]
#[inline]
/// Original signature : pub fn hash(depth: u8, lon: f64, lat: f64) -> u64
pub fn hpx_hash(depth: f64, lon:f64, lat:f64) -> i64 {
  cdshealpix::nested::hash(depth as u8, lon, lat) as i64
}

// -------------------------------------------------- best_starting_depth -----------------------------------------------------------
#[pg_extern]
#[inline]
/// Original signature : pub fn best_starting_depth(d_max_rad: f64) -> u8
pub fn hpx_best_starting_depth(d_max_rad: f64) -> f64 {
    cdshealpix::best_starting_depth(d_max_rad) as f64
}

// -------------------------------------------------- nside --------------------------------------------------------------------------
#[pg_extern]
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

#[pg_extern]
#[inline]
// Original signature : pub fn center(depth: u8, hash: u64) -> (f64, f64)
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_center(depth: i32, hash: i64) -> Coo {
  cdshealpix::nested::center(depth as u8, hash as u64).into()
}

// -------------------------------------------------- nested::parent -----------------------------------------------------------------
#[pg_extern]
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

#[pg_extern]
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

#[pg_extern]
// Original signature : pub const fn children(hash: u64, delta_depth: u8) -> RangeInclusive<u64>
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_children(hash: i64, delta_depth: i32) -> pgrx::datum::Range<i64> {
  RangeCurrentCrate(cdshealpix::nested::children(hash as u64, delta_depth as u8)).into()
}

// -------------------------------------------------- nested::to_uniq ----------------------------------------------------------------
#[pg_extern]
#[inline]
// Original signature : pub fn to_uniq(depth: u8, hash: u64) -> u64
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_to_uniq(depth: i32, hash: i64) -> i64 {
  cdshealpix::nested::to_uniq(depth as u8, hash as u64) as i64
}

// -------------------------------------------------- nested::to_zuniq ----------------------------------------------------------------
#[pg_extern]
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

#[pg_extern]
// Original signature : pub const fn from_uniq(uniq_hash: u64) -> (u8, u64)
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_from_uniq(uniq_hash: i64) -> UniqTuple {
  cdshealpix::nested::from_uniq(uniq_hash as u64).into()
}

// -------------------------------------------------- nested::from_zuniq ---------------------------------------------------------------
#[pg_extern]
// Original signature : pub const fn from_zuniq(zuniq: u64) -> (u8, u64)
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_from_zuniq(zuniq: i64) -> UniqTuple {
  cdshealpix::nested::from_zuniq(zuniq as u64).into()
}

// -------------------------------------------------- nested::external_edge -------------------------------------------------------------
#[pg_extern]
// Original signature : pub fn external_edge(depth: u8, hash: u64, delta_depth: u8) -> Box<[u64]> 
pub fn hpx_external_edge(depth: i32, hash: i64, delta_depth: i32) -> Vec<i64> {
  let vec_u64: Vec<u64> = cdshealpix::nested::external_edge(depth as u8, hash as u64, delta_depth as u8).into_vec();
  let vec_i64 = unsafe { std::mem::transmute::<Vec<u64>, Vec<i64>>(vec_u64) } ;
  vec_i64
}

// -------------------------------------------------- nested::internal_edge --------------------------------------------------------------
#[pg_extern]
// Original signature : pub fn external_edge(depth: u8, hash: u64, delta_depth: u8) -> Box<[u64]> 
pub fn hpx_internal_edge(depth: i32, hash: i64, delta_depth: i32) -> Vec<i64> {
  let vec_u64: Vec<u64> = cdshealpix::nested::internal_edge(depth as u8, hash as u64, delta_depth as u8).into_vec();
  let vec_i64 = unsafe { std::mem::transmute::<Vec<u64>, Vec<i64>>(vec_u64) } ;
  vec_i64
}

// -------------------------------------------------- nested::neighbours -----------------------------------------------------------------
#[derive(PostgresType, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MainWindMapPSQL {
  array: [Option<i64>; 9],
}

impl From<cdshealpix::compass_point::MainWindMap<u64>> for MainWindMapPSQL {
  fn from(item: cdshealpix::compass_point::MainWindMap<u64>) -> Self {
    let entries_vec = item.entries_vec();
    let mut array: [Option<i64>; 9] = [None; 9];
    for i in 0..10 {
      let (_, val) = &entries_vec[i];
      array[i] = Some(*val as i64);
    }
    MainWindMapPSQL { array }
  }
}

#[pg_extern]
#[inline]
// Original signature : pub fn neighbours(depth: u8, hash: u64, include_center: bool) -> MainWindMap<u64>
pub fn hpx_neighbours(depth: i32, hash: i64, include_center: bool) -> MainWindMapPSQL {
  cdshealpix::nested::neighbours(depth as u8, hash as u64, include_center).into()
}

// -------------------------------------------------------- TESTS ------------------------------------------------------------------------
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use std::f64::consts::PI;
    use pgrx::datum::Range;
    use cdshealpix::nested::n_hash;

    // Adaptation of HEALPix's Rust tests for PGRX

    #[pg_test]
    fn test_hpx_hash() {
        assert_eq!(19456, crate::hpx_hash(6.0,0.0,0.0));
    }

    #[pg_test]
    fn test_hpx_best_starting_depth() {
        assert_eq!(0.0, crate::hpx_best_starting_depth(PI / 4f64)); // 45 deg
        assert_eq!(5.0, crate::hpx_best_starting_depth(0.0174533)); //  1 deg
        assert_eq!(7.0, crate::hpx_best_starting_depth(0.0043632)); // 15 arcmin
        assert_eq!(9.0, crate::hpx_best_starting_depth(0.0013));    // 4.469 arcmin
        assert_eq!(15.0, crate::hpx_best_starting_depth(1.454E-5)); // 3 arcsec
        assert_eq!(20.0, crate::hpx_best_starting_depth(6.5E-7));   // 0.134 arcsec
        assert_eq!(22.0, crate::hpx_best_starting_depth(9.537E-8)); // 20 mas
    }

    #[pg_test]
    fn test_hpx_nside() {
        assert_eq!(1.0, crate::hpx_nside(0));
        assert_eq!(2.0, crate::hpx_nside(1));
        assert_eq!(4.0, crate::hpx_nside(2));
        assert_eq!(8.0, crate::hpx_nside(3));
        assert_eq!(16.0, crate::hpx_nside(4));
        assert_eq!(32.0, crate::hpx_nside(5));
        assert_eq!(64.0, crate::hpx_nside(6));
        assert_eq!(128.0, crate::hpx_nside(7));
        assert_eq!(256.0, crate::hpx_nside(8));
        assert_eq!(512.0, crate::hpx_nside(9));
        assert_eq!(1024.0, crate::hpx_nside(10));
        assert_eq!(2048.0, crate::hpx_nside(11));
        assert_eq!(4096.0, crate::hpx_nside(12));
        assert_eq!(8192.0, crate::hpx_nside(13));
        assert_eq!(16384.0, crate::hpx_nside(14));
        assert_eq!(32768.0, crate::hpx_nside(15));
        assert_eq!(65536.0, crate::hpx_nside(16));
        assert_eq!(131072.0, crate::hpx_nside(17));
        assert_eq!(262144.0, crate::hpx_nside(18));
        assert_eq!(524288.0, crate::hpx_nside(19));
        assert_eq!(1048576.0, crate::hpx_nside(20));
        assert_eq!(2097152.0, crate::hpx_nside(21));
        assert_eq!(4194304.0, crate::hpx_nside(22));
        assert_eq!(8388608.0, crate::hpx_nside(23));
        assert_eq!(16777216.0, crate::hpx_nside(24));
        assert_eq!(33554432.0, crate::hpx_nside(25));
        assert_eq!(67108864.0, crate::hpx_nside(26));
        assert_eq!(134217728.0, crate::hpx_nside(27));
        assert_eq!(268435456.0, crate::hpx_nside(28));
        assert_eq!(536870912.0, crate::hpx_nside(29));
    }

    #[pg_test]
    fn test_hpx_parent() {
        let hash1: i64 = 4;
        let parent1 = crate::hpx_parent(hash1, 1);
        assert_eq!(parent1, 1);
    
        let hash2: i64 = 640;
        let parent2 = crate::hpx_parent(hash2, 1);
        assert_eq!(parent2, 160);
        let grandparent2 = crate::hpx_parent(hash2, 2);
        assert_eq!(grandparent2, 40);
        let base2 = crate::hpx_parent(hash2, 3);
        assert_eq!(base2, 10);
      }

    #[pg_test]
    fn test_hpx_siblings() {
        let hash1: i64 = 3;
        let siblings1: Range<i64> = crate::hpx_siblings(0, hash1);

        // Range stands for RangeInclusive here so the upper bound is inclusive
        // Recovery of the lower and the upper bounds
        let lower_bound1 = match Range::lower(&siblings1) {
          Some(value) => value,
          None => panic!("No bound"),
        };
        let upper_bound1 = match Range::upper(&siblings1) {
          Some(value) => value,
          None => panic!("No bound"),
        };

        // Extraction of the value in the &pgrx::prelude::RangeBound<i64>
        fn extract_value(bound: &RangeBound<i64>) -> i64 {
            match bound {
                RangeBound::Inclusive(val) => *val,
                RangeBound::Exclusive(val) => *val,
                RangeBound::Infinite => panic!("Bound not supported in this test"),
            }
        }
        assert!(lower_bound1.get() <= Some(&hash1) &&  upper_bound1.get() >= Some(&hash1));     // <=> siblings1.contains(&hash1)
        assert_eq!(extract_value(lower_bound1), 0i64);
        assert_eq!(extract_value(upper_bound1), 11i64);
    


        let hash2: i64 = 76;
        let siblings2: Range<i64> = crate::hpx_siblings(2, hash2);

        // Recovery of the lower and upper bounds
        let lower_bound2 = match Range::lower(&siblings2) {
          Some(value) => value,
          None => panic!("No bound"),
        };
        let upper_bound2 = match Range::upper(&siblings2) {
          Some(value) => value,
          None => panic!("No bound"),
        };

        assert!(lower_bound2.get() <= Some(&hash2) && upper_bound2.get() >= Some(&hash2));     // <=> siblings2.contains(&hash2)
        assert_eq!(extract_value(lower_bound2) & 3, 0i64);
        assert_eq!(extract_value(upper_bound2) & 3, 3i64);
      }

    #[pg_test]
    fn test_hpx_children() {
      let hash1: i64 = 0;
      let children1: Range<i64> = crate::hpx_children(hash1, 1);
      // Range is right exclusive
      assert_eq!(Range::lower(&children1), Some(&RangeBound::Inclusive(0i64)));
      assert_eq!(Range::upper(&children1), Some(&RangeBound::Exclusive(4i64)));
      let grandchildren1 = crate::hpx_children(hash1, 2);
      assert_eq!(Range::lower(&grandchildren1), Some(&RangeBound::Inclusive(0i64)));
      assert_eq!(Range::upper(&grandchildren1), Some(&RangeBound::Exclusive(16i64)));

      let hash2: i64 = 31;
      let children2 = crate::hpx_children(hash2, 1);
      assert_eq!(Range::lower(&children2), Some(&RangeBound::Inclusive(124i64)));
      assert_eq!(Range::upper(&children2), Some(&RangeBound::Exclusive(128i64)));
    }

    #[pg_test]
    fn test_hpx_to_uniq() {
      // First test
      assert_eq!(crate::hpx_to_uniq(0, 0) , 16);

      // Second test
      for depth in 0i32..8i32 {
        for idx in 0i64..(n_hash(depth as u8) as i64) {
          assert_eq!(depth, crate::hpx_from_uniq(crate::hpx_to_uniq(depth, idx)).depth);
          assert_eq!(idx, crate::hpx_from_uniq(crate::hpx_to_uniq(depth,idx)).hash);
        }
      }
    }
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
