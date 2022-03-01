use std::{fs::File, io::BufReader, path::Path};

use lazy_static::lazy_static;
use quick_xml::{
    events::{BytesStart, BytesText, Event},
    Reader,
};
use regex::Regex;

pub trait FromXmlEvents {
    type FieldType;
    fn from_xml_events(
        &mut self,
        field_type: Self::FieldType,
        text: BytesText<'_>,
        reader: &Reader<BufReader<File>>,
    );

    fn is_last_event(field_type: Self::FieldType) -> bool;
}

pub fn parse_data_file<'a, T>(path: &Path) -> Vec<T>
where
    T: FromXmlEvents + Default + Ord + Clone,
    T::FieldType: Copy + TryFrom<BytesStart<'static>>,
{
    let mut all_data = Vec::new();
    let mut reader = Box::new(Reader::from_file(path).unwrap());
    reader.trim_text(true);
    let mut buffer = Vec::new();
    let mut current_data = T::default();
    let mut current_field_type = None;
    loop {
        let event = reader.read_event(&mut buffer);
        match event {
            Ok(Event::Eof) => break,
            Ok(Event::Start(s)) => current_field_type = s.to_owned().try_into().ok(),
            Ok(Event::Text(t)) => {
                if let Some(field_type) = current_field_type {
                    current_data.from_xml_events(field_type, t, &reader);
                    if T::is_last_event(field_type) && current_data != T::default() {
                        all_data.push(current_data.clone());
                        current_data = T::default();
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

pub fn remove_excess_whitespace(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\s+"#).unwrap();
    }
    RE.replace_all(s, " ").to_string()
}
