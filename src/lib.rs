use std::{env::args, path::Path, process::exit};

use lazy_static::lazy_static;
use quick_xml::{events::Event, Reader};
pub use record_error::RecordError;
pub use record_name::RecordName;
use regex::Regex;
mod record_error;
mod record_name;

pub trait FromXmlEvents: Default + Ord + Clone {
    type FieldType: Copy + TryFrom<String>;
    fn from_xml_text(&mut self, field_type: Self::FieldType, text: &str);

    fn is_last_event(field_type: Self::FieldType) -> bool;

    fn parse_from_xml_file(path: &Path) -> Vec<Self> {
        let mut all_data = Vec::new();
        let mut reader = Box::new(Reader::from_file(path).unwrap());
        reader.trim_text(true);
        let mut buffer = Vec::new();
        let mut current_data = Self::default();
        let mut current_field_type = None;
        loop {
            let event = reader.read_event(&mut buffer);
            match event {
                Ok(Event::Eof) => break,
                Ok(Event::Start(s)) => {
                    current_field_type = String::from_utf8(s.name().to_vec())
                        .unwrap()
                        .try_into()
                        .ok()
                }
                Ok(Event::Text(t)) => {
                    if let Some(field_type) = current_field_type {
                        current_data
                            .from_xml_text(field_type, &t.unescape_and_decode(&reader).unwrap());
                        if Self::is_last_event(field_type) && current_data != Self::default() {
                            all_data.push(current_data.clone());
                            current_data = Self::default();
                        }
                    }
                }
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }
        all_data.sort_unstable();
        all_data
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    pub name: Option<RecordName>,
    pub error: RecordError,
}

impl Output {
    pub fn new(name: Option<RecordName>, error: RecordError) -> Self {
        Self { name, error }
    }
}

pub struct InputPaths {
    pub error_file: String,
    pub name_file: String,
    pub output_file: String,
}

impl InputPaths {
    pub fn get_or_exit() -> InputPaths {
        let mut argv = args();
        let error_message = "
Missing required arguments
Usage: <command> <error-file-path> <name-file-path> <output-file-path>
        ";
        let early_exit = || {
            println!("{}", error_message);
            exit(1);
        };
        InputPaths {
            error_file: argv.nth(1).unwrap_or_else(early_exit),
            name_file: argv.next().unwrap_or_else(early_exit),
            output_file: argv.next().unwrap_or_else(early_exit),
        }
    }
}

pub fn write_output_file(output: Vec<Output>, output_path: &str) {
    let mut writer = csv::Writer::from_path(output_path).unwrap();
    writer
        .write_record(["ID", "Error", "First Name", "Last Name"])
        .unwrap();
    for row in output {
        if let Some(name) = row.name {
            writer
                .write_record([
                    name.record_id.to_string(),
                    row.error.error_text,
                    name.first_name,
                    name.last_name,
                ])
                .unwrap();
        } else {
            writer
                .write_record([row.error.record_id.to_string(), row.error.error_text])
                .unwrap();
        }
    }
}

pub fn remove_excess_whitespace(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\s+"#).unwrap();
    }
    RE.replace_all(s, " ").to_string()
}

pub fn match_error_to_name(
    record_name_data: Vec<RecordName>,
    error_data: Vec<RecordError>,
) -> Vec<Output> {
    error_data
        .into_iter()
        .map(|error| {
            let name = record_name_data
                .binary_search_by_key(&error.record_id, |name| name.record_id)
                .ok()
                .and_then(|index| record_name_data.get(index))
                .map(|name| name.clone());
            Output { name, error }
        })
        .collect()
}
