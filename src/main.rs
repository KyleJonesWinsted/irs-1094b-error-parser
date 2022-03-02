#![warn(clippy::all, clippy::pedantic)]
use std::path::Path;

use irs_1094b_error_parser::{parse_data_file, RecordError, RecordName};

fn main() {
    let error_file_path = Path::new("../Test AIR Errors.xml");
    let record_name_file_path = Path::new("../1094B_Request_BB3FJ_20220228T104623000Z.xml");
    let record_name_data: Vec<RecordName> = parse_data_file(record_name_file_path);
    let error_data: Vec<RecordError> = parse_data_file(error_file_path);
    for datum in record_name_data {
        println!("{:?}", datum);
    }
    for datum in error_data {
        println!("{:?}", datum);
    }
}
