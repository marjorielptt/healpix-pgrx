use pgrx::prelude::*;
use cdshealpix::nested::get;

::pgrx::pg_module_magic!();

#[pg_extern]
#[inline]
// pub fn hash(depth: u8, lon: f64, lat: f64) -> u64 {
pub fn hash(depth: f64, lon:f64, lat:f64) -> f64 {
  get(depth as u8).hash(lon, lat) as f64
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hash() {
        assert_eq!(19456.0, crate::hash(6.0,0.0,0.0));
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
