//! # Polars_plus - Extending `polars` with more functionality

pub mod csv;
pub mod dataframe;
pub mod errors;
pub mod json;
pub mod lazyframe;
pub mod series;
pub mod success;

use polars::prelude::*;

/// A struct that holds the path and name of a CSV file
pub struct Csv {
    pub path: String,
    pub name: String,
}

/// A struct that represents a lazyframe
pub struct LazyFramePlus {
    pub lazyframe: LazyFrame,
}
