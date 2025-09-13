use std::env;

use rocket::{State, get, post, routes};

use crate::services::localstack::{
    LocalstackConfiguration, ServiceInvocationError, SmsMessageList, get_sms_message_list,
    purge_sms_message_list,
};

mod services;

#[get("/sms-messages")]
async fn list_sms_messages(
    a_config: &State<AppConfig>,
) -> Result<SmsMessageList, ServiceInvocationError> where
{
    get_sms_message_list(&a_config.localstack_config).await
}

#[post("/sms-messages/purge")]
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
        .mount("/api", routes![list_sms_messages, purge_sms_messages])
        .manage(a_config)
        .ignite()
        .await?
        .launch()
        .await?;
    Ok(())
}
