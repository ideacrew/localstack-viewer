use std::env;

use lapin::Connection;
use rocket::{State, get, http::ContentType, post, routes};

use localstack_viewer_data::SmsMessageList;

use crate::services::{
    ServiceInvocationError,
    contact_gateway::{self, publish_update_trigger_message},
    localstack::{
        LocalstackConfiguration, get_sms_message_list, list_blocked_numbers, purge_sms_message_list,
    },
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

#[post("/sms/update_blocklist")]
async fn update_sms_blocklist(a_config: &State<AppConfig>) -> Result<(), ServiceInvocationError> where
{
    publish_update_trigger_message(&a_config.event_source_connection).await
}

struct AppConfig {
    localstack_config: LocalstackConfiguration,
    event_source_connection: Connection,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let localstack_url = env::var("LOCALSTACK_URL").unwrap();
    let amqp_url = env::var("EVENT_SOURCE_AMQP_URL").unwrap();

    let conn = contact_gateway::establish_connection(&amqp_url)
        .await
        .unwrap();

    let a_config = AppConfig {
        localstack_config: LocalstackConfiguration::new(localstack_url.as_str()),
        event_source_connection: conn,
    };

    let _r = rocket::build()
        .mount(
            "/api",
            routes![
                list_sms_messages,
                purge_sms_messages,
                get_blocked_numbers,
                update_sms_blocklist
            ],
        )
        .manage(a_config)
        .ignite()
        .await?
        .launch()
        .await?;
    Ok(())
}
