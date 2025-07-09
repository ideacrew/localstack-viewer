use rocket::{
    Response,
    http::{ContentType, Status},
    response::Responder,
};
use serde::Deserialize;
use serde_json::{Map, value::RawValue};
use std::{collections::HashMap, io::Cursor};

pub(crate) struct LocalstackConfiguration {
    base_url: String,
}

static SMS_LIST_URI: &str = "/_aws/sns/sms-messages";

#[derive(Deserialize)]
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

impl Into<tera::Context> for SmsMessageList {
    fn into(self) -> tera::Context {
        let mut ctxt = tera::Context::new();
        ctxt.insert("region", &self.region);

        let mut message_groups = Vec::new();

        for (k, v) in self.sms_messages.into_iter() {
            let mut mg_attrs = Map::new();
            let mut mg_messages = Vec::new();
            let _ = mg_attrs.insert(
                "phone_number".to_owned(),
                serde_json::Value::String(k.to_owned()),
            );
            for m in v {
                mg_messages.push(serde_json::Value::String(m.as_ref().get().to_owned()));
            }
            let _ = mg_attrs.insert("messages".to_owned(), serde_json::Value::Array(mg_messages));
            message_groups.push(serde_json::Value::Object(mg_attrs));
        }
        let message_group_array = serde_json::Value::Array(message_groups);
        ctxt.insert("message_groups", &message_group_array);
        ctxt
    }
}

pub(crate) fn sms_message_list_for_template(message_list: SmsMessageList) -> serde_json::Value {
    let mut attrs = Map::new();
    let _ = attrs.insert(
        "region".to_owned(),
        serde_json::Value::String(message_list.region.to_owned()),
    );
    let mut message_groups = Vec::new();

    for (k, v) in message_list.sms_messages.into_iter() {
        let mut mg_attrs = Map::new();
        let mut mg_messages = Vec::new();
        let _ = mg_attrs.insert(
            "phone_number".to_owned(),
            serde_json::Value::String(k.to_owned()),
        );
        for m in v {
            mg_messages.push(serde_json::Value::String(m.as_ref().get().to_owned()));
        }
        let _ = mg_attrs.insert("messages".to_owned(), serde_json::Value::Array(mg_messages));
        message_groups.push(serde_json::Value::Object(mg_attrs));
    }
    let message_group_array = serde_json::Value::Array(message_groups);
    let _ = attrs.insert("message_groups".to_owned(), message_group_array);
    serde_json::Value::Object(attrs)
}
