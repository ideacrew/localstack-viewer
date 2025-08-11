use std::env;

use rocket::{State, get, routes};

use crate::services::localstack::{
    LocalstackConfiguration, ServiceInvocationError, SmsMessageList, get_sms_message_list,
};

mod services;

#[get("/sms-messages")]
async fn sms_messages(
    a_config: &State<AppConfig>,
) -> Result<SmsMessageList, ServiceInvocationError> where
{
    get_sms_message_list(&a_config.localstack_config).await
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
        .mount("/api", routes![sms_messages])
        .manage(a_config)
        .ignite()
        .await?
        .launch()
        .await?;
    Ok(())
}
