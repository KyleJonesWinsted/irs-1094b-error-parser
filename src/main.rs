#![warn(clippy::all, clippy::pedantic)]
use std::{path::Path, time::Instant};

use irs_1094b_error_parser::{
    match_error_to_name, write_output_file, FromXmlEvents, InputPaths, RecordError, RecordName,
};

fn main() {
    let start = Instant::now();
    let paths = InputPaths::get_or_exit();
    let error_file_path = Path::new(&paths.error_file);
    let record_name_file_path = Path::new(&paths.name_file);
    let record_name_data = RecordName::parse_from_xml_file(record_name_file_path);
    let error_data = RecordError::parse_from_xml_file(error_file_path);
    let output_len = error_data.len();
    let output = match_error_to_name(record_name_data, error_data);
    write_output_file(output, &paths.output_file);
    println!(
        "Done! Matched {} errors in {:?}",
        output_len,
        start.elapsed()
    );
}
