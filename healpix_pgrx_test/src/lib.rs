use pgrx::prelude::*; // default

// for best_starting_depth
static SMALLER_EDGE2OPEDGE_DIST: [f64; 30] = [
  8.410686705679302e-1,  // depth = 0
  3.7723631722170065e-1, // depth = 1
  1.8203364957037313e-1, // depth = 2
  8.91145416330163e-2,   // depth = 3
  4.3989734509169175e-2, // depth = 4
  2.1817362566054977e-2, // depth = 5
  1.0854009694242892e-2, // depth = 6
  5.409888140793663e-3,  // depth = 7
  2.6995833266547898e-3, // depth = 8
  1.3481074874673246e-3, // depth = 9
  6.735240905806414e-4,  // depth = 10
  3.365953703015157e-4,  // depth = 11
  1.682452196838741e-4,  // depth = 12
  8.410609042173736e-5,  // depth = 13
  4.204784317861652e-5,  // depth = 14
  2.1022283297961136e-5, // depth = 15
  1.0510625670060442e-5, // depth = 16
  5.255150320257332e-6,  // depth = 17
  2.6275239729465538e-6, // depth = 18
  1.3137458638808036e-6, // depth = 19
  6.568678535571394e-7,  // depth = 20
  3.284323270983175e-7,  // depth = 21
  1.642156595517884e-7,  // depth = 22
  8.21076709163242e-8,   // depth = 23
  4.105378528139296e-8,  // depth = 24
  2.0526876713226626e-8, // depth = 25
  1.0263433216329513e-8, // depth = 26
  5.131714858175969e-9,  // depth = 27
  2.5658567623093986e-9, // depth = 28
  1.2829280665188905e-9, // depth = 29
];

// for nside
use cdshealpix::is_depth;

// for nested::center
use serde::{Serialize, Deserialize};

// for nested::siblings
use pgrx::datum::Range;

::pgrx::pg_module_magic!();

// HEALPix functions

// -------------------------------------------------- hash --------------------------------------------------------------------------
#[pg_extern]
#[inline]
/// Original signature : pub fn hash(depth: u8, lon: f64, lat: f64) -> u64 {
pub fn hpx_hash(depth: f64, lon:f64, lat:f64) -> i64 {
  cdshealpix::nested::hash(depth as u8, lon, lat) as i64
}

// -------------------------------------------------- best_starting_depth -----------------------------------------------------------
#[pg_extern]
#[inline]
/// Original signature : pub fn best_starting_depth(d_max_rad: f64) -> u8 {
pub fn hpx_best_starting_depth(d_max_rad: f64) -> f64 {
    cdshealpix::best_starting_depth(d_max_rad) as f64
}

// -------------------------------------------------- nside --------------------------------------------------------------------------
#[pg_extern]
#[inline]
// Original signature : fn check_depth(depth : i8) {}
fn check_depth(depth: i8) {
  assert!(is_depth(depth as u8), "Expected depth in [0, 29]");
}

#[pg_extern]
#[inline]
// Original signature : pub fn hpx_nside(depth: u8) -> u32 {
pub fn hpx_nside(depth: i8) -> f64 {
  cdshealpix::nside(depth as u8) as f64
}

// -------------------------------------------------- nested::center -----------------------------------------------------------------
// Creation of a ZocLayer type to replace Rust's tuple type because Postgres doesn't deal with tuples
#[derive(PostgresType, Serialize, Deserialize)]
pub struct ZocLayer {
    pub depth: f64,
    pub zoc: f64,
}

#[pg_extern]
#[inline]
// Original signature : pub fn center(depth: u8, hash: u64) -> (f64, f64) {
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_center(depth: i32, hash: i64) -> ZocLayer {
  let (new_depth,new_zoc) = cdshealpix::nested::center(depth as u8, hash as u64);
  ZocLayer{depth:new_depth, zoc:new_zoc}
}

// -------------------------------------------------- nested::parent -----------------------------------------------------------------
#[pg_extern]
// Original signature : pub const fn parent(hash: u64, delta_depth: u8) -> u64
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub const fn hpx_parent(hash: i64, delta_depth: i32) -> i64 {
  cdshealpix::nested::parent(hash as u64, delta_depth as u8) as i64
}

// -------------------------------------------------- nested::siblings ---------------------------------------------------------------
#[pg_extern]
// Original signature : pub const fn siblings(depth: u8, hash: u64) -> RangeInclusive<u64>
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_siblings(depth: i64, hash: i64) -> Range<i64> {
  if depth == 0 {
    Range::<i64>::new(0,11)
  } else {
    // floor-round to a multiple of 4
    let hash = hash & 0xFFFFFFFFFFFFFFFCu64 as i64; // <=> & 0b1111..111100
    Range::<i64>::new(hash,hash | 3)
  }
}

// -------------------------------------------------- nested::children ---------------------------------------------------------------
#[pg_extern]
// Original signature : pub const fn children(hash: u64, delta_depth: u8) -> Range<u64>
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_children(hash: i64, delta_depth: i64) -> Range<i64> {
  let twice_dd = delta_depth << 1;
  Range::<i64>::new(hash << twice_dd,RangeBound::Exclusive((hash + 1) << twice_dd))
}

// -------------------------------------------------- nested::to_uniq ----------------------------------------------------------------
#[pg_extern]
#[inline]
// Original signature : pub fn to_uniq(depth: u8, hash: u64) -> u64
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_to_uniq(depth: i64, hash: i64) -> i64 {
  cdshealpix::nested::to_uniq(depth as u8, hash as u64) as i64
}

// -------------------------------------------------- nested::to_zuniq ----------------------------------------------------------------
#[pg_extern]
// Original signature : pub fn to_zuniq(depth: u8, hash: u64) -> u64
// Remark : With (depth : i8) it didn't work because the result couldn't be displayed in the console so I switched its type to i32
pub fn hpx_to_zuniq(depth: i64, hash: i64) -> i64 {
  cdshealpix::nested::to_zuniq(depth as u8, hash as u64) as i64
}



#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use std::f64::consts::PI;
    use pgrx::datum::Range;

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
    fn test_hpx_to_uniq() {
      assert_eq!(crate::hpx_to_uniq(0, 0) , 16);
    }

    #[pg_test]
    fn test_siblings() {


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
