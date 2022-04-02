use std::{
    env::args,
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    io::BufReader,
    marker::PhantomData,
    path::Path,
};

use lazy_static::lazy_static;
use quick_xml::{events::Event, Reader};
pub use record_error::RecordError;
pub use record_name::RecordName;
use regex::Regex;
mod record_error;
mod record_name;

pub trait FromXmlEvents: Default + PartialEq {
    type FieldType: PartialEq + Copy + TryFrom<String>;
    fn insert_xml_value(&mut self, field_type: Self::FieldType, text: &str);
}

pub struct XmlEvents<T: FromXmlEvents> {
    reader: Reader<BufReader<File>>,
    buffer: Vec<u8>,
    item: PhantomData<T>,
    current_field_type: Option<T::FieldType>,
    seen_fields: Vec<T::FieldType>,
}

impl<T: FromXmlEvents> XmlEvents<T> {
    pub fn try_from_path(path: &Path) -> Result<XmlEvents<T>, Box<dyn Error>> {
        let mut reader = Reader::from_file(path)?;
        reader.trim_text(true);
        Ok(XmlEvents {
            reader,
            buffer: Vec::new(),
            item: PhantomData,
            current_field_type: None,
            seen_fields: Vec::new(),
        })
    }

    fn is_end_of_current_record(&mut self) -> bool {
        if let Some(current_field_type) = self.current_field_type {
            let is_end = if self.seen_fields.contains(&current_field_type) {
                self.seen_fields.clear();
                true
            } else {
                false
            };
            self.seen_fields.push(current_field_type);
            return is_end;
        }
        false
    }

    fn bytes_to_field(s: quick_xml::events::BytesStart) -> Option<T::FieldType> {
        String::from_utf8(s.name().to_vec())
            .unwrap()
            .try_into()
            .ok()
    }
}

impl<T: FromXmlEvents> Iterator for XmlEvents<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_data = T::default();
        loop {
            let event = self.reader.read_event(&mut self.buffer);
            match event {
                Ok(Event::Eof) => return None,
                Ok(Event::Start(s)) => {
                    self.current_field_type = Self::bytes_to_field(s);
                    if self.is_end_of_current_record() && current_data != T::default() {
                        return Some(current_data);
                    }
                }
                Ok(Event::End(_)) => (),
                Ok(Event::Text(t)) => {
                    if let Some(field_type) = self.current_field_type {
                        current_data.insert_xml_value(
                            field_type,
                            &t.unescape_and_decode(&self.reader).unwrap(),
                        );
                    }
                }
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }
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
    pub fn get() -> Result<InputPaths, InputPathsError> {
        let mut argv = args();
        Ok(InputPaths {
            error_file: argv.nth(1).ok_or(InputPathsError)?,
            name_file: argv.next().ok_or(InputPathsError)?,
            output_file: argv.next().ok_or(InputPathsError)?,
        })
    }
}

pub struct InputPathsError;

impl Display for InputPathsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "
Missing required arguments
Usage: <command> <error-file-path> <name-file-path> <output-file-path>
        "
        )
    }
}

impl Debug for InputPathsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Display::fmt(&self, f)
    }
}

impl Error for InputPathsError {}

pub fn write_output_file(output: impl Iterator<Item = Output>, output_path: &str) -> usize {
    let mut writer = csv::Writer::from_path(output_path).unwrap();
    let mut written_rows = 0;
    writer
        .write_record(["ID", "Error", "First Name", "Last Name"])
        .unwrap();
    for row in output {
        if let Some(name) = &row.name {
            writer
                .write_record([
                    &name.record_id.to_string(),
                    &row.error.error_text,
                    &name.first_name,
                    &name.last_name,
                ])
                .unwrap();
        } else {
            writer
                .write_record([
                    &row.error.record_id.to_string(),
                    &row.error.error_text,
                    "",
                    "",
                ])
                .unwrap();
        }
        written_rows += 1;
    }
    written_rows
}

pub fn remove_excess_whitespace(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\s+"#).unwrap();
    }
    RE.replace_all(s, " ").to_string()
}

pub fn match_error_to_name<'a>(
    record_name_data: &'a [RecordName],
    error_data: impl Iterator<Item = RecordError> + 'a,
) -> impl Iterator<Item = Output> + 'a {
    error_data.map(|error| {
        let name = record_name_data
            .iter()
            .find(|name| name.record_id == error.record_id)
            .cloned();
        Output { name, error }
    })
}
