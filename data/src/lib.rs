use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct SmsSubmittedAt {
    pub DataType: String,
    pub StringValue: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SmsMessageAttributes {
    pub submitted_at: Option<SmsSubmittedAt>,
    #[serde(flatten)]
    pub extra_attributes: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct SmsMessage {
    pub PhoneNumber: String,
    pub MessageId: String,
    pub Message: String,
    pub MessageAttributes: Option<SmsMessageAttributes>,
    #[serde(flatten)]
    pub extra_attributes: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SmsMessageList {
    pub sms_messages: HashMap<String, Vec<SmsMessage>>,
    pub region: String,
}

impl SmsMessageList {
    pub fn message_count(&self) -> usize {
        self.sms_messages.iter().fold(0, |c, (_k, v)| c + v.len())
    }

    pub fn number_count(&self) -> usize {
        self.sms_messages.keys().len()
    }
}

impl SmsMessage {
    pub fn submitted_at(&self) -> Option<String> {
        self.MessageAttributes
            .as_ref()
            .and_then(|ma| ma.submitted_at.clone())
            .map(|sa| sa.StringValue)
    }
}

#[cfg(feature = "server")]
use std::io::Cursor;

#[cfg(feature = "server")]
use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Response,
};

#[cfg(feature = "server")]
impl<'r, 'o: 'r> Responder<'r, 'o> for SmsMessageList {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let json = serde_json::to_string(&self);
        match json {
            Err(e) => {
                let err = format!("{:?}:\n{:?}", e, self);
                Ok(Response::build()
                    .status(Status::Ok)
                    .header(ContentType::JSON)
                    .sized_body(err.len(), Cursor::new(err))
                    .finalize())
            }
            Ok(a) => Ok(Response::build()
                .status(Status::Ok)
                .header(ContentType::JSON)
                .sized_body(a.len(), Cursor::new(a))
                .finalize()),
        }
    }
}
