pub(crate) mod contact_gateway;
pub(crate) mod localstack;

use std::io::Cursor;

use aws_sdk_sns::{
    error::SdkError, operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
};
use aws_smithy_types::body::SdkBody;
use rocket::{
    Response,
    http::{ContentType, Status},
    response::Responder,
};

use crate::services::contact_gateway::ContactGatewayMessagePublishError;

#[derive(Debug)]
pub(crate) enum ServiceInvocationError {
    #[allow(dead_code)]
    ReqwestError(reqwest::Error),
    #[allow(dead_code)]
    SerializationError(serde_json::Error, String),
    #[allow(dead_code)]
    AwsSdkError(
        SdkError<ListPhoneNumbersOptedOutError, aws_smithy_runtime_api::http::Response<SdkBody>>,
    ),
    #[allow(dead_code)]
    ContactGatewayMessageError(ContactGatewayMessagePublishError),
}

impl From<reqwest::Error> for ServiceInvocationError {
    fn from(value: reqwest::Error) -> Self {
        ServiceInvocationError::ReqwestError(value)
    }
}

impl From<serde_json::Error> for ServiceInvocationError {
    fn from(value: serde_json::Error) -> Self {
        ServiceInvocationError::SerializationError(value, "".to_owned())
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

impl From<ContactGatewayMessagePublishError> for ServiceInvocationError {
    fn from(value: ContactGatewayMessagePublishError) -> Self {
        ServiceInvocationError::ContactGatewayMessageError(value)
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
