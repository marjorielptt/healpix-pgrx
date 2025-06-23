use pgrx::prelude::*; // default

use serde::{Serialize, Deserialize};

#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BMOCpsql {
    depth_max: i32,
    pub entries: Vec<i64>,
}