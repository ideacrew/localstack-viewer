use std::env;

use rocket::{Build, Rocket, State, get, http::ContentType, launch, response::Responder, routes};
use rocket_dyn_templates::{Template, context};
use tera::Tera;

use crate::{
    services::localstack::{
        LocalstackConfiguration, ServiceInvocationError, get_sms_message_list,
        sms_message_list_for_template,
    },
    templates::{Templater, init_template_provider},
};

mod services;

mod templates;

#[get("/sms_messages")]
async fn sms_messages(
    a_config: &State<AppConfig>,
    templater: &State<Templater>,
) -> Result<(ContentType, String), ServiceInvocationError> {
    let data = get_sms_message_list(&a_config.localstack_config).await?;
    Ok(templater.render("sms_messages/index", data))
}

#[get("/")]
fn homepage(templater: &State<Templater>) -> (ContentType, String) {
    templater.render("homepage", tera::Context::new())
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

    let templater = init_template_provider();

    let _ = rocket::build()
        .mount("/", routes![homepage, sms_messages])
        .manage(a_config)
        .manage(templater)
        .ignite()
        .await?
        .launch()
        .await?;
    Ok(())
}
