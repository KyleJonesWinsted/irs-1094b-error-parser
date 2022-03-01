use std::{fs::File, io::BufReader};

use irs_1094b_error_parser::FromXmlEvents;
use quick_xml::{
    events::{BytesStart, BytesText},
    Reader,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordName {
    record_id: usize,
    first_name: String,
    last_name: String,
}

impl Default for RecordName {
    fn default() -> Self {
        Self {
            record_id: Default::default(),
            first_name: Default::default(),
            last_name: Default::default(),
        }
    }
}

impl PartialOrd for RecordName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RecordName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.record_id.cmp(&other.record_id)
    }
}

impl FromXmlEvents for RecordName {
    type FieldType = RecordNameFieldType;

    fn from_xml_events(
        &mut self,
        field_type: RecordNameFieldType,
        text: BytesText<'_>,
        reader: &Reader<BufReader<File>>,
    ) {
        let text = text.unescape_and_decode(&reader).unwrap();
        match field_type {
            RecordNameFieldType::RecordId => self.record_id = text.parse().unwrap(),
            RecordNameFieldType::FirstName => self.first_name = text,
            RecordNameFieldType::LastName => self.last_name = text,
        };
    }

    fn is_last_event(field_type: Self::FieldType) -> bool {
        field_type == RecordNameFieldType::LastName
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordNameFieldType {
    RecordId,
    FirstName,
    LastName,
}

impl TryFrom<BytesStart<'_>> for RecordNameFieldType {
    fn try_from(start: BytesStart) -> Result<Self, Self::Error> {
        match start.name() {
            b"RecordId" => Ok(RecordNameFieldType::RecordId),
            b"PersonFirstNm" => Ok(RecordNameFieldType::FirstName),
            b"PersonLastNm" => Ok(RecordNameFieldType::LastName),
            _ => Err(()),
        }
    }

    type Error = ();
}
