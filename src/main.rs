#![warn(clippy::all, clippy::pedantic)]
use std::{error::Error, path::Path, time::Instant};

use irs_1094b_error_parser::{
    match_error_to_name, write_output_file, InputPaths, RecordName, XmlEvents,
};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let paths = InputPaths::get()?;
    let error_file_path = Path::new(&paths.error_file);
    let record_name_file_path = Path::new(&paths.name_file);
    let record_name_data: Vec<RecordName> =
        XmlEvents::try_from_path(record_name_file_path)?.collect();
    let error_data = XmlEvents::try_from_path(error_file_path)?;
    let output = match_error_to_name(&record_name_data, error_data);
    let written_rows = write_output_file(output, &paths.output_file);
    println!(
        "Done! Matched {} errors in {:?}",
        written_rows,
        start.elapsed()
    );
    Ok(())
}
