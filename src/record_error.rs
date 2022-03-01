use irs_1094b_error_parser::{remove_excess_whitespace, FromXmlEvents};
use lazy_static::lazy_static;
use quick_xml::events::BytesStart;
use regex::Regex;

#[derive(Debug, Clone)]
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

impl Default for RecordError {
    fn default() -> Self {
        Self {
            record_id: Default::default(),
            error_text: Default::default(),
        }
    }
}

impl FromXmlEvents for RecordError {
    type FieldType = RecordErrorFieldType;

    fn from_xml_events(
        &mut self,
        field_type: Self::FieldType,
        text: quick_xml::events::BytesText<'_>,
        reader: &quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    ) {
        let text = &text.unescape_and_decode(&reader).unwrap();
        match field_type {
            RecordErrorFieldType::RecordId => {
                self.record_id = parse_record_id(&remove_excess_whitespace(text))
            }
            RecordErrorFieldType::ErrorText => self.error_text = remove_excess_whitespace(text),
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

impl TryFrom<BytesStart<'_>> for RecordErrorFieldType {
    type Error = ();

    fn try_from(value: BytesStart<'_>) -> Result<Self, Self::Error> {
        match value.name() {
            b"UniqueRecordId" => Ok(RecordErrorFieldType::RecordId),
            b"ns2:ErrorMessageTxt" => Ok(RecordErrorFieldType::ErrorText),
            _ => Err(()),
        }
    }
}
