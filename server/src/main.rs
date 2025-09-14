use std::env;

use rocket::{State, get, http::ContentType, post, routes};

use localstack_viewer_data::SmsMessageList;

use crate::services::localstack::{
    LocalstackConfiguration, ServiceInvocationError, get_sms_message_list, list_blocked_numbers,
    purge_sms_message_list,
};

mod services;

#[get("/sms/blocked-numbers")]
async fn get_blocked_numbers(
    a_config: &State<AppConfig>,
) -> Result<(ContentType, String), ServiceInvocationError> {
    list_blocked_numbers(&a_config.localstack_config).await
}

#[get("/sms/messages")]
async fn list_sms_messages(
    a_config: &State<AppConfig>,
) -> Result<SmsMessageList, ServiceInvocationError> where
{
    get_sms_message_list(&a_config.localstack_config).await
}

#[post("/sms/purge-messages")]
async fn purge_sms_messages(a_config: &State<AppConfig>) -> Result<(), ServiceInvocationError> where
{
    purge_sms_message_list(&a_config.localstack_config).await
}

struct AppConfig {
    localstack_config: LocalstackConfiguration,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let localstack_url = env::var("LOCALSTACK_URL").unwrap();

    let a_config = AppConfig {
        localstack_config: LocalstackConfiguration::new(localstack_url.as_str()),
    };

    let _ = rocket::build()
        .mount(
            "/api",
            routes![list_sms_messages, purge_sms_messages, get_blocked_numbers],
        )
        .manage(a_config)
        .ignite()
        .await?
        .launch()
        .await?;
    Ok(())
}
