//! CSV wrapper for implementing CSV operations
//!
//!
//!

use crate::Csv;

use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;

impl Csv {
    /// ## to_dataframe - Convert a CSV file to a DataFrame  
    pub fn to_dataframe(&self, dataframe: &mut DataFrame) -> Result<DataFrame, PolarsError> {
        let filepath = self.path.clone();

        // open the file
        let file: File = File::open(filepath).expect("CSV File not found");

        // build the reader
        let reader: BufReader<File> = BufReader::new(file);

        // read the CSV file
        CsvReader::new(reader)
            .infer_schema(None)
            .has_header(true)
            .finish()
            .expect("Error reading CSV file");

        Ok(dataframe.clone())
    }
}
