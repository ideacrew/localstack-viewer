use aws_config::BehaviorVersion;
use aws_sdk_sns::{
    Client, Config,
    config::{IdentityCache, Region},
    error::SdkError,
    operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
};
use aws_smithy_runtime_api::client::auth::AuthSchemePreference;
use aws_smithy_types::body::SdkBody;
use rocket::{
    Response,
    http::{ContentType, Status},
    response::{Builder, Responder},
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
    AwsSdkError(
        SdkError<ListPhoneNumbersOptedOutError, aws_smithy_runtime_api::http::Response<SdkBody>>,
    ),
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

impl From<SdkError<ListPhoneNumbersOptedOutError, aws_smithy_runtime_api::http::Response<SdkBody>>>
    for ServiceInvocationError
{
    fn from(
        value: SdkError<
            ListPhoneNumbersOptedOutError,
            aws_smithy_runtime_api::http::Response<SdkBody>,
        >,
    ) -> Self {
        ServiceInvocationError::AwsSdkError(value)
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

fn blocked_number_response<'a>(numbers: Vec<String>) -> (ContentType, String) {
    let json = serde_json::to_string(&numbers.clone());
    match json {
        Err(e) => {
            let err = format!("{:?}", e);
            (ContentType::JSON, err)
        }
        Ok(a) => (ContentType::JSON, a),
    }
}

pub(crate) async fn list_blocked_numbers(
    lc: &LocalstackConfiguration,
) -> Result<(ContentType, String), ServiceInvocationError> {
    let creds = aws_sdk_sns::config::Credentials::new(
        "ANOTREAL",
        "notrealrnrELgWzOk3IfjzDKtFBhDby",
        None,
        None,
        "test",
    );
    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(creds)
        .endpoint_url(&lc.base_url)
        .region("us-east-1")
        .load()
        .await;
    let client = Client::new(&config);
    client
        .list_phone_numbers_opted_out()
        .into_paginator()
        .items()
        .send()
        .try_collect()
        .await
        .map_err(|e| e.into())
        .map(|o| blocked_number_response(o))
}

pub(crate) async fn purge_sms_message_list(
    lc: &LocalstackConfiguration,
) -> Result<(), ServiceInvocationError> {
    let client = reqwest::Client::new();
    let _result = client
        .delete(lc.base_url.clone() + SMS_LIST_URI)
        .send()
        .await;
    Ok(())
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
