use crate::queue::{Queue, SerdeSerializedSend};
use chrono::Local;
use serde::{Deserialize, Serialize};

const DATE_FORMAT: &str = "%Y.%m.%d %H:%M:%S";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Origin {
    Upload,
    Uuid,
    IR,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum State {
    Started,
    InProgress,
    Ok,
    Fail,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Attributes {
    pub process_id: String,
    pub transaction_id: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Status {
    pub origin: Origin,
    pub state: State,
    pub created_at: String,
    pub message: Option<String>,
    pub attributes: Attributes,
}

impl Status {
    pub fn new<S: Into<String>>(origin: Origin, state: State, process_id: S) -> Self {
        use chrono::offset::TimeZone;
        use chrono_tz::Europe::Berlin;

        let berlin = Berlin
            .from_local_datetime(&Local::now().naive_local())
            .single()
            .expect("Unexpected Date");

        Self {
            origin,
            state,
            attributes: Attributes {
                process_id: process_id.into(),
                transaction_id: Some(crate::request::transaction::read_id()),
                uuid: None,
            },
            created_at: berlin.format(DATE_FORMAT).to_string(),
            message: None,
        }
    }

    pub fn from_upload_lambda<S: Into<String>>(state: State, process_id: S) -> Self {
        Self::new(Origin::Upload, state, process_id)
    }

    pub fn from_uuid_lambda<S: Into<String>>(state: State, process_id: S) -> Self {
        Self::new(Origin::Uuid, state, process_id)
    }

    pub fn from_ir_lambda<S: Into<String>>(state: State, process_id: S) -> Self {
        Self::new(Origin::IR, state, process_id)
    }

    pub fn with_message<S: Into<String>>(&mut self, message: S) -> &mut Self {
        self.message = Some(message.into());

        self
    }

    pub fn with_uuid<S: Into<String>>(&mut self, uuid: S) -> &mut Self {
        self.attributes.uuid = Some(uuid.into());

        self
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).expect("Could not serialize Status")
    }

    pub fn send_to<S: Into<String>>(&self, url: S, queue: &Queue) -> Result<(), String> {
        let json = self.as_json();
        queue.send_serialized(&json, url)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_serialize() {
        use serde_json;

        let status = super::Status::from_ir_lambda(super::State::InProgress, "abcdef#42");
        let json = status.as_json();
        assert_eq!(
            status,
            serde_json::from_str(&json).expect("Could not deserialize")
        );
    }
}
