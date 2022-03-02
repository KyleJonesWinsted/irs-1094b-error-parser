use crate::{remove_excess_whitespace, FromXmlEvents};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Default, Clone)]
pub struct RecordError {
    record_id: usize,
    error_text: String,
}

impl PartialEq for RecordError {
    fn eq(&self, other: &Self) -> bool {
        self.record_id == other.record_id
    }
}

impl Eq for RecordError {}

impl Ord for RecordError {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.record_id.cmp(&other.record_id)
    }
}

impl PartialOrd for RecordError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromXmlEvents for RecordError {
    type FieldType = RecordErrorFieldType;

    fn from_xml_text(&mut self, field_type: Self::FieldType, text: &str) {
        let text = &remove_excess_whitespace(text);
        match field_type {
            RecordErrorFieldType::RecordId => self.record_id = parse_record_id(text),
            RecordErrorFieldType::ErrorText => self.error_text = text.to_string(),
        }
    }

    fn is_last_event(field_type: Self::FieldType) -> bool {
        field_type == RecordErrorFieldType::ErrorText
    }
}

fn parse_record_id(s: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\|.+\|\s*?(\d+)").unwrap();
    }
    RE.captures_iter(s)
        .next()
        .and_then(|c| c.get(1))
        .and_then(|c| c.as_str().parse().ok())
        .unwrap_or_default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordErrorFieldType {
    RecordId,
    ErrorText,
}

impl TryFrom<String> for RecordErrorFieldType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "UniqueRecordId" => Ok(RecordErrorFieldType::RecordId),
            "ns2:ErrorMessageTxt" => Ok(RecordErrorFieldType::ErrorText),
            _ => Err(()),
        }
    }
}
