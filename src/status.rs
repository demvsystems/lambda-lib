use chrono::Local;
use serde::{Deserialize, Serialize};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

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
    pub transaction_id: String,
    pub process_id: Option<String>,
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
    pub fn new(origin: Origin, state: State) -> Self {
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
                transaction_id: crate::request::transaction::read_id(),
                process_id: None,
                uuid: None,
            },
            created_at: berlin.format(DATE_FORMAT).to_string(),
            message: None,
        }
    }

    pub fn from_upload_lambda(state: State) -> Self {
        Self::new(Origin::Upload, state)
    }

    pub fn from_uuid_lambda(state: State) -> Self {
        Self::new(Origin::Uuid, state)
    }

    pub fn from_ir_lambda(state: State) -> Self {
        Self::new(Origin::IR, state)
    }

    pub fn with_message<S: Into<String>>(&mut self, message: S) -> &mut Self {
        self.message = Some(message.into());

        self
    }

    pub fn with_process_id<S: Into<String>>(&mut self, process_id: S) -> &mut Self {
        self.attributes.process_id = Some(process_id.into());

        self
    }

    pub fn with_uuid<S: Into<String>>(&mut self, uuid: S) -> &mut Self {
        self.attributes.uuid = Some(uuid.into());

        self
    }

    pub fn with_transaction_id(&mut self, transaction_id: Option<String>) -> &mut Self {
        self.attributes.transaction_id =
            transaction_id.unwrap_or_else(|| crate::request::transaction::read_id());

        self
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).expect("Could not serialize Status")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_serialize() {
        use serde_json;

        let mut status = super::Status::from_ir_lambda(super::State::InProgress);
        status.with_process_id("abcdef#42");
        assert_eq!(Some("abcdef#42"), status.attributes.process_id.as_deref());
        let json = status.as_json();
        let result: super::Status = serde_json::from_str(&json).expect("Could not deserialize");
        assert_eq!(status, result);
    }
}
