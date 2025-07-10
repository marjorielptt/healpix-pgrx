use pgrx::prelude::*; // default

use moc::moc::range::RangeMOC;
use moc::qty::Hpx;
use moc::elemset::range::MocRanges;

use pgrx::spi::SpiResult;
use serde::{Serialize, Deserialize};
use serde::ser::SerializeStruct;
use serde::de::Deserializer;

use std::ops::Range as StdRange;
use pgrx::datum::Range as PgRange;

// RangeMOC type that is PSQL compatible
#[derive(PostgresType, Debug)]
pub struct RangeMOCPSQL {
    pub depth_max: i32,
    pub ranges: Vec<PgRange<i64>>,
}

// RangeMOC<u64, Hpx<u64>> -> RangeMOCPSQL
impl From<RangeMOC<u64, Hpx<u64>>> for RangeMOCPSQL {
    fn from(item: RangeMOC<u64, Hpx<u64>>) -> Self {
        let depth_max = item.depth_max() as i32;

        let ranges = item
            .into_moc_ranges()  // we catch the ranges of the MOC
            .iter()             // we iterate on each StdRange<u32> in the Box<[StdRange<u32>]>
            .map(|std_range| {
                PgRange::new(std_range.start as i64, std_range.end as i64)     // we convert every StdRange in a PgRange
            })
            .collect();

        RangeMOCPSQL { depth_max, ranges }
    }
}

// RangeMOCPSQL -> RangeMOC<u64, Hpx<u64>>
impl From<RangeMOCPSQL> for RangeMOC<u64, Hpx<u64>> {
    fn from(item: RangeMOCPSQL) -> Self {
        let ranges_u64: Vec<StdRange<u64>> = item
            .ranges         // we catch the Vec<PgRange<i64>>
            .into_iter()    // we iterate on each PgRange<i64> in the Vec<PgRange<i64>>
            .map(|pg_range| {
                let start = match pg_range.lower() {
                    Some(RangeBound::Inclusive(lower_bound)) => *lower_bound as u64,  
                    Some(RangeBound::Exclusive(lower_bound)) => (*lower_bound + 1) as u64, // we work with u64 (integers) so an exclusive lower bound means the bound+1 for StdRange (default: lower bound is inclusive for StdRange)
                                                                                           // MIGHT BE BETTER TO WORK WITH A BOOL (for example let exclusive: bool = true;)
                    Some(RangeBound::Infinite) | None => panic!("Unexpected unbounded lower range"),
                };
                let end = match pg_range.upper() {
                    Some(RangeBound::Inclusive(upper_bound)) => (*upper_bound + 1) as u64, // we work with u64 (integers) so an inclusive upper bound means the bound+1 for StdRange (default: upper bound is exclusive for StdRange)
                    Some(RangeBound::Exclusive(upper_bound)) => *upper_bound as u64,
                    Some(RangeBound::Infinite) | None => panic!("Unexpected unbounded upper range"),
                };
                StdRange { start, end }     // we convert every PgRange in a StdRange
            })
            .collect();

        let moc_ranges = MocRanges::new_unchecked(ranges_u64);      // We create the MocRanges from the StdRange

        RangeMOC::new(item.depth_max as u8, moc_ranges)     
    }
}

// Implementation of Serialize for RangeMOCPSQL
impl Serialize for RangeMOCPSQL {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
    {
        let mut state = serializer.serialize_struct("RangeMOCPSQL", 2)?;    // we serialize the structure RangeMOCPSQL which has 2 fields
        state.serialize_field("depth_max", &self.depth_max)?;       // we serialize the depth_max field

        let std_ranges: Vec<std::ops::Range<i64>> = self.ranges.iter().map(|pg_range| {
            let start = match pg_range.lower() {
                Some(pgrx::datum::RangeBound::Inclusive(lower_bound)) => *lower_bound,
                Some(pgrx::datum::RangeBound::Exclusive(lower_bound)) => *lower_bound + 1,
                Some(RangeBound::Infinite) | None => panic!("Unexpected unbounded lower range"),
            };
        
            let end = match pg_range.upper() {
                Some(pgrx::datum::RangeBound::Inclusive(upper_bound)) => *upper_bound + 1,
                Some(pgrx::datum::RangeBound::Exclusive(upper_bound)) => *upper_bound,
                Some(RangeBound::Infinite) | None => panic!("Unexpected unbounded upper range"),
            };
        
            std::ops::Range { start, end }
        }).collect();

        state.serialize_field("ranges", &std_ranges)?;      // we serialize the StdRange bc it's an equivalent structure of PgRange to JSON
        state.end()
    }
}

// Implementation of Deserialize for RangeMOCPSQL
impl<'de> Deserialize<'de> for RangeMOCPSQL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            depth_max: i32,
            ranges: Vec<std::ops::Range<i64>>,
        }

        let inner = Inner::deserialize(deserializer)?;

        let ranges = inner
            .ranges
            .into_iter()
            .map(|std_range| PgRange::new(std_range.start, std_range.end))
            .collect();

        Ok(RangeMOCPSQL {
            depth_max: inner.depth_max,
            ranges,
        })
    }
}

// MOC -> Ascii
#[pg_extern(immutable, parallel_safe)]
// Original signature : pub fn to_ascii(&self) -> Result<String, AsciiError>
pub fn hpx_to_ascii(moc: RangeMOCPSQL) -> SpiResult<String> {
    let range_moc: RangeMOC<u64, Hpx<u64>> = moc.into();
    match range_moc.to_ascii() {
        Ok(ascii) => Ok(ascii),
        Err(e) => error!("Failed to convert RangeMOC to ASCII: {}", e),
    }
}
