//! Wrapper for lazyframe module
//!

use crate::LazyFramePlus;

use polars::prelude::*;

impl LazyFramePlus {
    /// ## new - Create a new LazyFramePlus instance
    pub fn new(lazyframe: LazyFrame) -> LazyFramePlus {
        LazyFramePlus { lazyframe }
    }

    /// ## numerise_nan_column - Numerise NaN values in a column
    ///
    pub fn numerise_nan_column(&self, column_name: &str) -> Result<LazyFrame, PolarsError> {
        let lazyframe = self.lazyframe.clone();

        let lazyframe_numberised: LazyFrame =
            lazyframe.with_columns(vec![when(col(column_name).is_null())
                .then(lit(0.0))
                .otherwise(col(column_name))
                .alias(column_name)]);

        Ok(lazyframe_numberised)
    }

    /// ## rename_column - Rename a column in a LazyFrame
    ///
    pub fn rename_lazyframe_column(
        lazyframe: LazyFrame,
        old_name: &str,
        new_name: &str,
    ) -> Result<LazyFrame, PolarsError> {
        let lazyframe_pivots: LazyFrame = lazyframe
            .with_column(col(old_name).alias(new_name))
            .drop_columns(vec![old_name]);

        Ok(lazyframe_pivots)
    }
}
