use pgrx::prelude::*; // par défaut

// use cdshealpix::nested::get; // pour hash + center

// pour best_starting_depth
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

// pour nside
// use cdshealpix::nside_unsafe;
use cdshealpix::is_depth;

::pgrx::pg_module_magic!();

// Fonctions HEALPix

// -------------------------------------------------- hash --------------------------------------------------------------------------
#[pg_extern]
#[inline]
/// Signature d'origine : pub fn hash(depth: u8, lon: f64, lat: f64) -> u64 {
pub fn hpx_hash(depth: f64, lon:f64, lat:f64) -> i64 {
  cdshealpix::nested::hash(depth as u8, lon, lat) as i64
}

// -------------------------------------------------- best_starting_depth --------------------------------------------------------------------------
#[pg_extern]
#[inline]
/// Signature d'origine : pub fn best_starting_depth(d_max_rad: f64) -> u8 {
pub fn hpx_best_starting_depth(d_max_rad: f64) -> f64 {
    cdshealpix::best_starting_depth(d_max_rad) as f64
}

// -------------------------------------------------- nside --------------------------------------------------------------------------
#[pg_extern]
#[inline]
// Signature d'origine : fn check_depth(depth : i8) {}
fn check_depth(depth: i8) {
  assert!(is_depth(depth as u8), "Expected depth in [0, 29]");
}

#[pg_extern]
#[inline]
// Signature d'origine : pub fn hpx_nside(depth: u8) -> u32 {
pub fn hpx_nside(depth: i8) -> f64 {
  cdshealpix::nside(depth as u8) as f64
}

// -------------------------------------------------- center --------------------------------------------------------------------------
#[pg_extern]
#[inline]
// Signature d'origine : pub fn center(depth: u8, hash: u64) -> (f64, f64) {
pub fn hpx_center(depth: i8, hash: i64) -> (f64, f64) {
  cdshealpix::nested::center(depth as u8, hash as u64)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use std::f64::consts::PI;

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
