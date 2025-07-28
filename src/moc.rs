use pgrx::prelude::*;   // default

// Library imports
use serde::{Deserialize, Serialize};
use pgrx::{
    spi::SpiResult,
    datum::Range as PgRange,
};
use std::ops::{
    Range as StdRange,
    Not, BitAnd, BitOr, BitXor, Sub
};
use moc::{
    moc::{
        range::RangeMOC,
        range::CellSelection,
        cellcellrange::CellOrCellRangeMOC,
        HasMaxDepth,
    },
    elemset::range::MocRanges,
    qty::Hpx,
    deser::ascii::from_ascii_ivoa,
    elem::cellcellrange::CellOrCellRange,
};

use crate::bmoc::*;

// ----------------------------- Postgres compatible types declarations & types conversions ------------------------------

// Creation of a PSQL compatible type of RangeMOC
#[derive(PostgresType, Debug, Serialize, Deserialize)]
pub struct RangeMOCPSQL {
    pub depth_max: i32,
    pub ranges: Vec<StdRange<i64>>,
}

// Creation of a StdRange type that is in the current crate to satisfy the orphan rule 
pub struct StdRangeCrate(pub std::ops::Range<i64>);

// PgRange<i64> -> StdRangeCrate<i64>
impl From<PgRange<i64>> for StdRangeCrate {
    fn from(item: PgRange<i64>) -> StdRangeCrate {
        let start: i64 = match item.lower() {
            Some(&RangeBound::Exclusive(lower_bound)) => lower_bound+1,
            Some(&RangeBound::Inclusive(lower_bound)) => lower_bound,
            Some(RangeBound::Infinite) => panic!("Infinite RangeBound"),
            None => panic!("No RangeBound"),
        };
        let end: i64 = match item.upper() {
            Some(&RangeBound::Exclusive(upper_bound)) => upper_bound,
            Some(&RangeBound::Inclusive(upper_bound)) => upper_bound+1,
            Some(RangeBound::Infinite) => panic!("Infinite RangeBound"),
            None => panic!("No RangeBound"),
        };
        StdRangeCrate( StdRange {start, end})
    }
}

// StdRangeCrate<i64> -> PgRange<i64>
impl From<StdRangeCrate> for PgRange<i64> {
    fn from(item: StdRangeCrate) -> PgRange<i64> {
        PgRange::<i64>::new(item.0.start, item.0.end)
    }
}

// RangeMOCPSQL -> RangeMOC
impl From<RangeMOCPSQL> for RangeMOC<u64, Hpx::<u64>> {
    fn from(item: RangeMOCPSQL) -> Self {
        let mut ranges_u64 = Vec::new();

        for r in item.ranges {
            ranges_u64.push(StdRange{start : r.start as u64, end: r.end as u64});
        }

        RangeMOC::new(item.depth_max as u8, MocRanges::new_unchecked(ranges_u64))
    }
}

// RangeMOC -> RangeMOCPSQL
impl From<RangeMOC<u64, Hpx::<u64>>> for RangeMOCPSQL {
    fn from(item: RangeMOC<u64, Hpx::<u64>>) -> Self {
        let ranges_u64 = item
            .clone()
            .into_moc_ranges()
            .ranges()
            .0
            .clone()
            .into_vec();
        let mut ranges_i64 = Vec::new();

        for r in ranges_u64 {
            ranges_i64.push(unsafe {std::mem::transmute::<StdRange<u64>, StdRange<i64>>(r)});
        }

        RangeMOCPSQL {depth_max: item.depth_max() as i32, ranges: ranges_i64}
    }
}

// --------------------------------------------------- MOC creation ------------------------------------------------------

// Creation of a MOC
#[pg_extern(immutable, parallel_safe)]
pub fn create_range_moc_psql(depth_max: i32, ranges: Vec<PgRange<i64>>) -> RangeMOCPSQL {
    let mut std_ranges: Vec<StdRange<i64>> = Vec::new();

    for r in ranges {
        let new_range: StdRangeCrate = r.into();
        std_ranges.push(new_range.0);
    }
    
    RangeMOCPSQL { depth_max, ranges:std_ranges }
}

// Returns the vec of ranges of the moc
#[pg_extern(immutable, parallel_safe)]
pub fn moc_to_ranges(moc: RangeMOCPSQL) -> Vec<PgRange<i64>> {
    let mut res: Vec<PgRange<i64>> = Vec::new();
    for r in moc.ranges {
        res.push(r.into());
    }
    res
}

// ------------------------------------------------ deser::to_ascii_ivoa -------------------------------------------------

// RangeMOCPSQL -> Ascii
#[pg_extern(immutable, parallel_safe)]
pub fn moc_to_ascii(moc: RangeMOCPSQL) -> SpiResult<String> {
    let range_moc: RangeMOC<u64, Hpx::<u64>> = moc.into();

    match range_moc.to_ascii() {
        Ok(ascii) => Ok(ascii),
        Err(e) => error!("Failed to convert RangeMOC to ASCII: {}", e),
    }
}

// ----------------------------------------------- deser::from_ascii_ivoa ------------------------------------------------

// Creation of a PSQL compatible type of CellOrCellRangeMOC
#[derive(PostgresType, Debug, Serialize, Deserialize)]
pub struct CellOrCellRangeMOCPSQL {
    pub depth_max: i32,
    pub ranges: Vec<CellOrCellRangePSQL>,
}

// Creation of a PSQL compatible type of CellOrCellRange
#[derive(PostgresEnum, Debug, Serialize, Deserialize)]
pub enum CellOrCellRangePSQL {CellPSQL, CellRangePSQL}

// Creation of a PSQL compatible type of Cell
#[derive(PostgresType, Debug, Serialize, Deserialize)]
pub struct CellPSQL {
    pub depth_max: i32,
    pub idx: i32,
}

// Creation of a PSQL compatible type of Cell
#[derive(PostgresType, Debug, Serialize, Deserialize)]
pub struct CellRangePSQL {
    pub depth_max: i32,
    pub range: StdRange<i32>,
}

// CellOrCellRangeMOC -> RangeMOCPSQL
impl From<CellOrCellRangeMOC<u32, Hpx::<u32>>> for RangeMOCPSQL {
    fn from(item: CellOrCellRangeMOC<u32, Hpx::<u32>>) -> Self {
        let depth_max = item.depth_max() as i32;
        let vec_moc = item.moc_elems().0.0.into_vec();
        let mut vec_u32 = Vec::new();
        for elem in vec_moc {
            match elem {
                CellOrCellRange::Cell(cell) => vec_u32.push(StdRange {start: cell.idx, end: cell.idx}),
                CellOrCellRange::CellRange(cell_range) => vec_u32.push(cell_range.range),
            }
        }
        let vec_i64 = unsafe {std::mem::transmute::<Vec<StdRange<u32>>, Vec<StdRange<i64>>>(vec_u32)};
        RangeMOCPSQL{depth_max, ranges: vec_i64}
    }
}

// Ascii -> RangeMOCPSQL
#[pg_extern(immutable, parallel_safe)]
pub fn moc_from_ascii_ivoa(input: &str) -> SpiResult<RangeMOCPSQL> {
    match from_ascii_ivoa(input) {
        Ok(range_moc) => Ok(range_moc.into()),
        Err(e) => error!("Failed to convert RangeMOC to ASCII: {}", e),
    }
}

// ----------------------------------------------- moc::range::degrade -------------------------------------------------

// Degrade the input MOC (= MOC complement)
#[pg_extern(immutable, parallel_safe)]
pub fn moc_degrade(moc: RangeMOCPSQL, new_depth: i32) -> RangeMOCPSQL {
    let std_moc: RangeMOC<u64, Hpx::<u64>> = moc.into();
    let std_res = std_moc.degraded(new_depth as u8);
    std_res.into()
}

// ----------------------------------------------- moc::range::expanded -------------------------------------------------

// Add the MOC external border of depth `self.depth_max`.
#[pg_extern(immutable, parallel_safe)]
pub fn moc_expanded(moc: RangeMOCPSQL) -> RangeMOCPSQL {
    let std_moc: RangeMOC<u64, Hpx::<u64>> = moc.into();
    let std_res = std_moc.expanded();
    std_res.into()
}

// --------------------------------------------------- Contains ----------------------------------------------------------

// Tests if the cell is in the MOC 
#[pg_extern(immutable, parallel_safe)]
pub fn moc_is_in(moc: RangeMOCPSQL, lon: f64, lat: f64) -> bool {
    let range_moc: RangeMOC<u64, Hpx::<u64>> = moc.into();
    range_moc.is_in(lon.to_radians(), lat.to_radians())
}

//  ------------------------------- Creation of a MOC from different coverage types --------------------------------------

// Creation of a PSQL compatible type of CellSelection
#[derive(PostgresEnum, Debug, Serialize, Deserialize)]
pub enum CellSelectionPSQL {
    All,
    Inside,
    Border,
}

// CellSelectionPSQL -> CellSelection
impl From<CellSelectionPSQL> for CellSelection {
    fn from(item: CellSelectionPSQL) -> Self {
        match item {
            CellSelectionPSQL::All => CellSelection::All,
            CellSelectionPSQL::Inside => CellSelection::Inside,
            CellSelectionPSQL::Border => CellSelection::Border,
        }
    }
}

// Creation of a MOC from a Cone
#[pg_extern(immutable, parallel_safe)]
pub fn moc_from_cone(
    lon: f64,
    lat: f64,
    radius: f64,
    depth: i32,
    delta_depth: i32,
    selection: CellSelectionPSQL
) -> RangeMOCPSQL
{
    let range_moc: RangeMOC<u64, Hpx::<u64>> = RangeMOC::from_cone(lon.to_radians(), lat.to_radians(), radius.to_radians(), depth as u8, delta_depth as u8, selection.into());
    range_moc.into()
}

// Creation of a MOC from an EllipticalCone
#[pg_extern(immutable, parallel_safe)]
pub fn moc_from_elliptical_cone(
    lon: f64,
    lat: f64,
    a: f64,
    b: f64,
    pa: f64,
    depth: i32,
    delta_depth: i32,
    selection: CellSelectionPSQL
) -> RangeMOCPSQL
{
    let range_moc: RangeMOC<u64, Hpx::<u64>> = RangeMOC::from_elliptical_cone(lon.to_radians(), lat.to_radians(), a.to_radians(), b.to_radians(), pa.to_radians(), depth as u8, delta_depth as u8, selection.into());
    range_moc.into()
}

// Creation of a MOC from a Polygon
#[pg_extern(immutable, parallel_safe)]
pub fn moc_from_polygon(
    vertices: Vec<VertexPSQL>,
    complement: bool,
    depth: i32,
    selection: CellSelectionPSQL
) -> RangeMOCPSQL
{
    let mut vertices_tuple: Vec<(f64,f64)> = Vec::new();
    for vertex in vertices {
      vertices_tuple.push(vertex.into());
    }
    let vertices_as_array = vertices_tuple.as_slice();
    let range_moc: RangeMOC<u64, Hpx::<u64>> = RangeMOC::from_polygon(vertices_as_array, complement, depth as u8, selection.into());
    range_moc.into()
}

// Creation of a MOC from a Box
#[pg_extern(immutable, parallel_safe)]
pub fn moc_from_box(
    lon: f64,
    lat: f64,
    a: f64,
    b: f64,
    pa: f64,
    depth: i32,
    selection: CellSelectionPSQL
) -> RangeMOCPSQL
{
    let range_moc: RangeMOC<u64, Hpx::<u64>> = RangeMOC::from_box(lon.to_radians(), lat.to_radians(), a.to_radians(), b.to_radians(), pa.to_radians(), depth as u8, selection.into());
    range_moc.into()
}

// Creation of a MOC from a Ring
#[pg_extern(immutable, parallel_safe)]
pub fn moc_from_ring(
    lon: f64,
    lat: f64,
    radius_int: f64,
    radius_ext: f64,
    depth: i32,
    delta_depth: i32,
    selection: CellSelectionPSQL
) -> RangeMOCPSQL
{
    let range_moc: RangeMOC<u64, Hpx::<u64>> = RangeMOC::from_ring(lon.to_radians(), lat.to_radians(), radius_int.to_radians(), radius_ext.to_radians(), depth as u8, delta_depth as u8, selection.into());
    range_moc.into()
}

// -------------------------------------------------------- Operations -----------------------------------------------------------

// ----------------------------------------------------------- Not ---------------------------------------------------------------

// Not
#[pg_extern(immutable, parallel_safe)]
pub fn moc_not(moc: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc_std: RangeMOC<u64, Hpx::<u64>> = moc.into();
    moc_std.not().into()
}

// Complement
#[pg_extern(immutable, parallel_safe)]
pub fn moc_complement(moc: RangeMOCPSQL) -> RangeMOCPSQL {
    moc_not(moc)
}

// Redefinition of !'s behavior for Rust utilisations
impl Not for RangeMOCPSQL {
  type Output = RangeMOCPSQL;

  fn not(self) -> RangeMOCPSQL {
    let moc = self;
    moc_not(moc)
  }
}

// ----------------------------------------------------------- And ---------------------------------------------------------------

// And
#[pg_extern(immutable, parallel_safe)]
pub fn moc_and(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc_std: RangeMOC<u64, Hpx::<u64>> = moc.into();
    let other_std: RangeMOC<u64, Hpx::<u64>> = other.into();
    moc_std.and(&other_std).into()
}

// Intersection
#[pg_extern(immutable, parallel_safe)]
pub fn moc_intersection(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    moc_and(moc, other)
}

// Redefinition of &'s behavior for Rust utilisations
impl BitAnd for RangeMOCPSQL {
  type Output = RangeMOCPSQL;

  fn bitand(self, other: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc = self;
    moc_and(moc, other)
  }
}

// Redefinition of &'s behavior for Postgres utilisations
#[pg_operator]
#[opname(&)]
fn my_and(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    moc & other
}

// ----------------------------------------------------------- Or ----------------------------------------------------------------

// Or
#[pg_extern(immutable, parallel_safe)]
pub fn moc_or(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc_std: RangeMOC<u64, Hpx::<u64>> = moc.into();
    let other_std: RangeMOC<u64, Hpx::<u64>> = other.into();
    moc_std.or(&other_std).into()
}

// Union
#[pg_extern(immutable, parallel_safe)]
pub fn moc_union(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    moc_or(moc, other)
}

// Redefinition of |'s behavior for Rust utilisations
impl BitOr for RangeMOCPSQL {
  type Output = RangeMOCPSQL;

  fn bitor(self, other: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc = self;
    moc_or(moc, other)
  }
}

// Redefinition of |'s behavior for Postgres utilisations
#[pg_operator]
#[opname(|)]
fn my_or(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    moc | other
}

// ----------------------------------------------------------- Xor ---------------------------------------------------------------

// Xor
#[pg_extern(immutable, parallel_safe)]
pub fn moc_xor(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc_std: RangeMOC<u64, Hpx::<u64>> = moc.into();
    let other_std: RangeMOC<u64, Hpx::<u64>> = other.into();
    moc_std.xor(&other_std).into()
}

// Redefinition of ^'s behavior for Rust utilisations
impl BitXor for RangeMOCPSQL {
  type Output = RangeMOCPSQL;

  fn bitxor(self, other: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc = self;
    moc_xor(moc, other)
  }
}

// Redefinition of ^'s behavior for Postgres utilisations
#[pg_operator]
#[opname(^)]
fn my_xor(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    moc ^ other
}

// ----------------------------------------------------------- Minus -------------------------------------------------------------

// Minus
#[pg_extern(immutable, parallel_safe)]
pub fn moc_minus(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc_std: RangeMOC<u64, Hpx::<u64>> = moc.into();
    let other_std: RangeMOC<u64, Hpx::<u64>> = other.into();
    moc_std.minus(&other_std).into()
}

// Redefinition of -'s behavior for Rust utilisations
impl Sub for RangeMOCPSQL {
  type Output = RangeMOCPSQL;

  fn sub(self, other: RangeMOCPSQL) -> RangeMOCPSQL {
    let moc = self;
    moc_minus(moc, other)
  }
}

// Redefinition of -'s behavior for Postgres utilisations
#[pg_operator]
#[opname(-)]
fn my_minus(moc: RangeMOCPSQL, other: RangeMOCPSQL) -> RangeMOCPSQL {
    moc - other
}
