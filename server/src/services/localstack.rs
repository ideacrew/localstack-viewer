use rocket::{
    Response,
    http::{ContentType, Status},
    response::Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::{collections::HashMap, io::Cursor};

pub(crate) struct LocalstackConfiguration {
    base_url: String,
}

static SMS_LIST_URI: &str = "/_aws/sns/sms-messages";

#[derive(Deserialize, Serialize)]
pub(crate) struct SmsMessageList {
    pub sms_messages: HashMap<String, Vec<Box<RawValue>>>,
    pub region: String,
}

#[derive(Debug)]
pub(crate) enum ServiceInvocationError {
    ReqwestError(reqwest::Error),
    SerializationError(serde_json::Error),
}

impl From<reqwest::Error> for ServiceInvocationError {
    fn from(value: reqwest::Error) -> Self {
        ServiceInvocationError::ReqwestError(value)
    }
}

impl From<serde_json::Error> for ServiceInvocationError {
    fn from(value: serde_json::Error) -> Self {
        ServiceInvocationError::SerializationError(value)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ServiceInvocationError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let err_str = format!("{:?}:", self);
        Ok(Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::Plain)
            .sized_body(err_str.len(), Cursor::new(err_str))
            .finalize())
    }
}

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

pub(crate) async fn get_sms_message_list(
    lc: &LocalstackConfiguration,
) -> Result<SmsMessageList, ServiceInvocationError> {
    let result = reqwest::get(lc.base_url.clone() + SMS_LIST_URI).await;
    let text = result?.text().await?;
    let data: serde_json::Result<SmsMessageList> = serde_json::from_str(&text);
    Ok(data?)
}

impl LocalstackConfiguration {
    pub(crate) fn new(url: &str) -> Self {
        LocalstackConfiguration {
            base_url: url.to_owned(),
        }
    }
}
