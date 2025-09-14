use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct SmsMessageList {
    pub sms_messages: HashMap<String, Vec<HashMap<String, Box<RawValue>>>>,
    pub region: String,
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
                let err = format!("{:?}", e);
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
