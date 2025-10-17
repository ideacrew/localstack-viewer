use dioxus::prelude::*;
use localstack_viewer_data::SmsMessageList;
use web_sys::js_sys::Date;

use crate::{modal_dialog::Modal, AppContext, MESSAGES_LAST_UPDATED};

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
          { "Purge Messages?"}
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
/*
We'll switch over to this component once we move to an interactive list - but it's not ready yet.
fn MessageRow(smsm: &SmsMessage) -> Element {
    rsx! {
        tr {
            td {
                { smsm.submitted_at() }
            }
            td {
                { smsm.MessageId.as_str() }
            }
            td {
                { smsm.PhoneNumber.as_str() }
            }
        }
    }
}*/

pub(crate) fn sms_messages_list(ml: &SmsMessageList) -> Element {
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
                            dt {
                                class: "font-bold",
                                { "Message ID" }
                            }
                            dd {
                                class: "ml-8",
                                { m.MessageId.as_str() }
                            }
                            if m.submitted_at().is_some() {
                                dt {
                                    class: "font-bold",
                                    { "Submitted At" }
                                }
                                dd {
                                    class: "ml-8",
                                    { m.submitted_at() }
                                }
                            }
                            dt {
                                class: "font-bold",
                                { "Phone Number" }
                            }
                            dd {
                                class: "ml-8",
                                { m.PhoneNumber.as_str() }
                            }
                            dt {
                                class: "font-bold",
                                { "Message" }
                            }
                            dd {
                                class: "ml-8",
                                { m.Message.as_str() }
                            }
                            for (sk, val) in m.extra_attributes.clone() {
                                dt {
                                    class: "font-bold",
                                    "{sk}"
                                }
                                dd {
                                    class: "ml-8",
                                    { format!("{}", val) }
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
