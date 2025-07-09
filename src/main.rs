use std::env;

use rocket::{Build, Rocket, State, get, launch, routes};
use rocket_dyn_templates::{Template, context};

use crate::services::localstack::{
    LocalstackConfiguration, ServiceInvocationError, get_sms_message_list,
    sms_message_list_for_template,
};

mod services;

#[get("/sms_messages")]
async fn sms_messages(a_config: &State<AppConfig>) -> Result<Template, ServiceInvocationError> {
    let data = get_sms_message_list(&a_config.localstack_config).await?;
    Ok(Template::render(
        "sms_messages/index",
        sms_message_list_for_template(data),
    ))
}

#[get("/")]
fn homepage() -> Template {
    Template::render("homepage", context! {})
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
        .mount("/", routes![homepage, sms_messages])
        .attach(Template::fairing())
        .manage(a_config)
        .ignite()
        .await?
        .launch()
        .await?;
    Ok(())
}
