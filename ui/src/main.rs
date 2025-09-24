use dioxus::prelude::*;
use reqwest::Url;
use web_sys::js_sys::Date;
use web_sys::window;

use localstack_viewer_data::SmsMessageList;

mod modal_dialog;

mod dashboard;

use crate::dashboard::Dashboard;

use crate::modal_dialog::Modal;

static MESSAGES_LAST_UPDATED: GlobalSignal<f64> = Global::new(|| Date::now());

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/sms/messages")]
    SmsMessages {},
    #[route("/sms/settings")]
    SmsSettings {},
}

#[derive(Clone)]
struct AppContext {
    base_url: String,
    sms_message_list: Resource<Result<SmsMessageList, reqwest::Error>>,
    blocked_list: Resource<Result<Vec<String>, reqwest::Error>>,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::prelude::launch(App);
}

fn read_base_url() -> String {
    let base_href = window().unwrap().location().href().unwrap();
    let mut url = Url::parse(&base_href).unwrap();
    url.path_segments_mut().unwrap().clear();
    url.to_string().trim_end_matches("/").to_owned()
}

#[component]
fn App() -> Element {
    let base_url = read_base_url();

    let base_url_value = base_url.clone();
    let base_url_value1 = base_url.clone();

    let message_list: Resource<Result<SmsMessageList, reqwest::Error>> = use_resource(move || {
        let _mlu = MESSAGES_LAST_UPDATED();
        let app_status_base_url = base_url_value.clone();
        async move {
            reqwest::get(app_status_base_url + "/api/sms/messages")
                .await?
                .json::<SmsMessageList>()
                .await
        }
    });

    let blocked_list = use_resource(move || {
        let app_status_base_url = base_url_value1.clone();
        async move {
            reqwest::get(app_status_base_url + "/api/sms/blocked-numbers")
                .await?
                .json::<Vec<String>>()
                .await
        }
    });

    let _ = use_context_provider(|| AppContext {
        base_url: base_url,
        sms_message_list: message_list,
        blocked_list: blocked_list,
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "relative isolate mt-8 max-w-3/4 place-content-center",
                div {
                    class: "text-center",
                    div {
                        h1 {
                            "Welcome to Localstack Viewer!"
                        }
                        p {
                            class: "text-lg pt-3",
                            "This application provides the ability to look at localstack status and contents.  It covers cases not available in the default LocalStack UI - or that are normally only in the paid versions."
                        }
                    }
                }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        header {
            class: "bg-black p-4 text-white",
            nav {
                id: "navbar",
                class: "flex items-center",
                Link {
                    class: "mr-4 text-xl flex items-center",
                    to: Route::Home {},
                    svg {
                        class: "mr-2",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        path {
                            d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"
                        }
                    }
                    { "LocalStack Viewer" }
                }
                Link {
                    class: "mr-4",
                    to: Route::SmsMessages {  },
                    "SMS Messages"
                }
                Link {
                    class: "mr-4",
                    to: Route::SmsSettings {  },
                    "SMS Settings"
                }
            }
        }
        main {
          Outlet::<Route> {}
        }
    }
}

fn render_sms_messages(ml: &SmsMessageList) -> Element {
    rsx! {
        h1 { { ml.region.clone() } }
        message_purge_dialog {

        }
        ul {
           class: "ml-8",
           for (k,v) in ml.sms_messages.iter() {
              li {
                h3 {
                  { k.to_string() }
                }
                ol {
                  class: "list-decimal ml-16",
                  for m in v {
                    li {
                        class: "mb-4",
                        dl {
                            for (sk, val) in m {
                                dt {
                                    class: "font-bold",
                                    "{sk}"
                                }
                                dd {
                                    class: "ml-8",
                                    { format!("{}", val.as_ref()) }
                                }
                            }
                        }
                    }
                  }
                }
              }
           }
        }
    }
}

#[component]
fn SmsMessages() -> Element {
    let app_status: AppContext = use_context();

    match &*app_status.sms_message_list.read_unchecked() {
        Some(Ok(ml)) => render_sms_messages(ml),
        Some(Err(e)) => rsx! {
            code { { format!("Error: {:?}",e) } }
        },
        None => rsx! {
            h1 { "No data yet..." }
        },
    }
}

#[component]
fn message_purge_dialog() -> Element {
    let app_status: AppContext = use_context();
    let app_status_bu = app_status.base_url.clone();
    let mut open = use_signal(|| false);
    let app_status_base_url = app_status_bu.clone();
    let on_message_delete_clicked = move |_e: MouseEvent| {
        let app_status_base_url = app_status_base_url.clone();
        spawn(async move {
            let app_status_base_url = app_status_base_url.clone();
            let _mlu = MESSAGES_LAST_UPDATED.read();
            drop(_mlu);
            open.set(false);
            let client = reqwest::Client::new();
            let _ = client
                .post(app_status_base_url + "/api/sms/purge-messages")
                .send()
                .await;
            *MESSAGES_LAST_UPDATED.write() = Date::now();
        });
    };
    rsx! {
        button {
          onclick: move |_e| open.set(true),
          class: "rounded-xl inset-ring inset-ring-gray-300 px-2",
          { "Purge Messages"}
        }
        Modal {
            title: "Purge Messages?", body: "This will purge all messages, are you sure?",
            open_signal: open,
                button {
                        class: "rounded-lg shadow-xs bg-white px-3 py-2 inset-ring inset-ring-black-300",
                        onclick: on_message_delete_clicked,
                        { "Delete" }
                }
        }
    }
}

#[component]
fn update_contact_gateway_blocklist() -> Element {
    let mut open = use_signal(|| false);
    let app_status: AppContext = use_context();
    let app_status_bu = app_status.base_url.clone();
    let app_status_base_url = app_status_bu.clone();
    let on_update_list_clicked = move |_e: MouseEvent| {
        let app_status_base_url = app_status_base_url.clone();
        spawn(async move {
            let app_status_base_url = app_status_base_url.clone();
            open.set(false);
            let client = reqwest::Client::new();
            let _ = client
                .post(app_status_base_url + "/api/sms/update-blocklist")
                .send()
                .await;
        });
    };
    rsx! {
        button {
          onclick: move |_e| open.set(true),
          class: "rounded-xl inset-ring inset-ring-gray-300 px-2",
          { "Update Blocklist"}
        }
        Modal {
            title: "Update Blocklist?", body: "This will request contact gateway to update it's blocklist.  This may also send blocklist messages to Enroll.  Are you sure?",
            open_signal: open,
            button {
                    class: "rounded-lg shadow-xs bg-white px-3 py-2 inset-ring inset-ring-black-300",
                    onclick: on_update_list_clicked,
                    { "Yes I'm Sure" }
            }
        }
    }
}

#[component]
fn blocked_sms_numbers(blocked_list: Resource<Result<Vec<String>, reqwest::Error>>) -> Element {
    match &*blocked_list.read_unchecked() {
        Some(Ok(l)) => rsx! {
            ul {
                class: "ml-8",
                for v in l {
                  li { "{v}" }
                }
            }
        },
        Some(Err(e)) => rsx! { code { { format!("Error: {:?}",e) } } },
        _ => rsx! { div { { "No data yet..."} } },
    }
}

#[component]
fn SmsSettings() -> Element {
    let app_status: AppContext = use_context();

    let blocked_list = app_status.blocked_list;

    rsx! {
        h1 { "Blocked Numbers" }
        blocked_sms_numbers { blocked_list }
        update_contact_gateway_blocklist { }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let app_status: AppContext = use_context();

    rsx! {
        Dashboard {
            sms_message_list: app_status.sms_message_list,
            blocked_list: app_status.blocked_list,
        }
    }
}
