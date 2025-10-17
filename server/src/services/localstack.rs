use aws_config::BehaviorVersion;
use aws_sdk_sns::Client;
use rocket::http::ContentType;

pub(crate) struct LocalstackConfiguration {
    base_url: String,
}

static SMS_LIST_URI: &str = "/_aws/sns/sms-messages";

use localstack_viewer_data::SmsMessageList;

use crate::services::ServiceInvocationError;

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
    match data {
        Ok(d) => Ok(d),
        Err(e) => Err(ServiceInvocationError::SerializationError(e, text)),
    }
}

impl LocalstackConfiguration {
    pub(crate) fn new(url: &str) -> Self {
        LocalstackConfiguration {
            base_url: url.to_owned(),
        }
    }
}
