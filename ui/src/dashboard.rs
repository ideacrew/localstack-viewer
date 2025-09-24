use dioxus::prelude::*;
use localstack_viewer_data::SmsMessageList;

fn sms_message_count(ml: &SmsMessageList) -> Element {
    rsx! {
        div {
            class: "rounded-2xl inset-ring inset-ring-gray-300 flex flex-row mr-2 p-4 place-items-center",
            div {
              class: "sms_count_image text-white rounded-xl flex items-center justify-center mr-3",
              svg {
                  width: "24",
                  height: "24",
                  view_box: "0 0 24 24",
                  fill: "currentColor",
                  path { d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z" }
              }
            }
            div {
              class: "flex flex-col",
              div {
                class: "text-3xl font-bold",
                { ml.sms_messages.len().to_string() }
              }
              div {
                  class: "text-1xl",
                  { "SMS Messages" }
              }
            }
        }
    }
}

fn blocked_number_count(ml: &Vec<String>) -> Element {
    rsx! {
        div {
            class: "rounded-2xl inset-ring inset-ring-gray-300 flex flex-col p-4 mr-2",
            div {
              class: "text-3xl font-bold",
              { ml.len().to_string() }
            }
            div {
                class: "text-1xl",
                { "Blocked Numbers" }
            }
        }
    }
}

#[component]
pub(crate) fn Dashboard(
    sms_message_list: Resource<Result<SmsMessageList, reqwest::Error>>,
    blocked_list: Resource<Result<Vec<String>, reqwest::Error>>,
) -> Element {
    rsx! {
        h1 { { "LocalStack Messages Dashboard" } }
        div {
          class: "flex mt-10",
          match &*sms_message_list.read_unchecked() {
              Some(Ok(ml)) => sms_message_count(ml),
              Some(Err(e)) =>
                  rsx! { code { { format!("Error: {:?}",e) } } },
              None => rsx! { h1 { "No data yet..." } }
          }
          match &*blocked_list.read_unchecked() {
              Some(Ok(bl)) => blocked_number_count(bl),
              Some(Err(e)) =>
                  rsx! { code { { format!("Error: {:?}",e) } } },
              None => rsx! { h1 { "No data yet..." } }
          }
        }
    }
}
