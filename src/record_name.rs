use crate::FromXmlEvents;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RecordName {
    record_id: usize,
    first_name: String,
    last_name: String,
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

    fn from_xml_text(&mut self, field_type: RecordNameFieldType, text: &str) {
        match field_type {
            RecordNameFieldType::RecordId => self.record_id = text.parse().unwrap(),
            RecordNameFieldType::FirstName => self.first_name = text.to_string(),
            RecordNameFieldType::LastName => self.last_name = text.to_string(),
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

impl TryFrom<String> for RecordNameFieldType {
    fn try_from(start: String) -> Result<Self, Self::Error> {
        match start.as_str() {
            "RecordId" => Ok(RecordNameFieldType::RecordId),
            "PersonFirstNm" => Ok(RecordNameFieldType::FirstName),
            "PersonLastNm" => Ok(RecordNameFieldType::LastName),
            _ => Err(()),
        }
    }

    type Error = ();
}
