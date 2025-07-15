use pgrx::prelude::*; // default

// TESTS 

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use std::f64::consts::PI;
    use pgrx::datum::Range;
    use cdshealpix::nested::n_hash;
    use crate::bmoc::BMOCpsql;
    use crate::moc::*;
    use moc::elemset::range::MocRanges;
    use moc::qty::Hpx;
    use moc::moc::range::RangeMOC;
    use moc::moc::RangeMOCIntoIterator;

    // Adaptation of HEALPix's Rust tests for PGRX

    #[pg_test]
    fn test_hpx_hash() {
        assert_eq!(19456, crate::hpx_hash(6,0.0,0.0));
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

    #[pg_test]
    // Bad test : no cell treatment before the operation
    // See the full test on https://github.com/cds-astro/cds-healpix-rust/tree/48a5ee396abf852450e4b685f6e107310f39beec
    fn test_hpx_and() {
      let mut vec_test_1: Vec<i64> = Vec::new();
      vec_test_1.push(1099);
      vec_test_1.push(1121);
      vec_test_1.push(4395);
      vec_test_1.push(4410);
      vec_test_1.push(4481);
      

      let mut vec_test_2: Vec<i64> = Vec::new();
      vec_test_2.push(1124);
      vec_test_2.push(4395);
      vec_test_2.push(1126);
      vec_test_2.push(4493);
      vec_test_2.push(4502);

      let mut vec_test_res: Vec<i64> = Vec::new();
      vec_test_res.push(1126);
      vec_test_res.push(4395);
      vec_test_res.push(4495);
      vec_test_res.push(4502);

      let bmoc_1: BMOCpsql = BMOCpsql{ depth_max: 5, entries: vec_test_1 };
      let bmoc_2: BMOCpsql = BMOCpsql{ depth_max: 5, entries: vec_test_2 };
      let bmoc_res: BMOCpsql = BMOCpsql{ depth_max: 5, entries: vec_test_res };

      assert_eq!(crate::bmoc::hpx_and(bmoc_1, bmoc_2), bmoc_res);
    }

    #[pg_test]
    // Bad test : no cell treatment before the operation
    // See the full test on https://github.com/cds-astro/cds-healpix-rust/tree/48a5ee396abf852450e4b685f6e107310f39beec
    fn test_hpx_or() {
      let mut vec_test_1: Vec<i64> = Vec::new();
      vec_test_1.push(1099);
      vec_test_1.push(1121);
      vec_test_1.push(4395);
      vec_test_1.push(4410);
      vec_test_1.push(4481);
      

      let mut vec_test_2: Vec<i64> = Vec::new();
      vec_test_2.push(1124);
      vec_test_2.push(1126);
      vec_test_2.push(4493);
      vec_test_2.push(4502);
      

      let mut vec_test_res: Vec<i64> = Vec::new();
      vec_test_res.push(1099);
      vec_test_res.push(1121);
      vec_test_res.push(4395);
      vec_test_res.push(4410);
      vec_test_res.push(4481);

      let bmoc_1: BMOCpsql = BMOCpsql{ depth_max: 5, entries: vec_test_1 };
      let bmoc_2: BMOCpsql = BMOCpsql{ depth_max: 5, entries: vec_test_2 };
      let bmoc_res: BMOCpsql = BMOCpsql{ depth_max: 5, entries: vec_test_res };

      assert_eq!(crate::bmoc::hpx_or(bmoc_1, bmoc_2), bmoc_res);

      fn test_from_ascii_ivoa() {
        let smoc_ascii = "3/3 10 4/16-18 22 5/19-20 17/222 28/123456789 29/";
        let smoc = crate::moc::moc_from_ascii_ivoa(&smoc_ascii).unwrap();
        let mut rit = smoc.into_range_iter();
        assert_eq!(rit.depth_max, 29);
        assert_eq!(rit.next(), Some(493827156..493827160));
        assert_eq!(rit.next(), Some(3724541952..3741319168));
        assert_eq!(rit.next(), Some(5348024557502464..5910974510923776));
        assert_eq!(rit.next(), Some(13510798882111488..21392098230009856));
        assert_eq!(rit.next(), Some(24769797950537728..25895697857380352));
        assert_eq!(rit.next(), Some(45035996273704960..49539595901075456));
        assert_eq!(rit.next(), None);
      }
    }
}   
