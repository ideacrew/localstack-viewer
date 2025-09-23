use lapin::{
    BasicProperties, Connection, ConnectionProperties,
    options::{BasicPublishOptions, ConfirmSelectOptions},
    publisher_confirm::Confirmation,
};

use crate::services::ServiceInvocationError;

#[derive(Debug)]
pub(crate) enum ContactGatewayMessagePublishError {
    ConfirmationFailureError,
    #[allow(dead_code)]
    PublishError(lapin::Error),
}

fn merge_uri_and_vhost(uri: &str, vhost: &str) -> String {
    let huri = fluent_uri::Uri::parse(uri).unwrap();
    if huri.path().is_empty() {
        if vhost == "/" {
            uri.to_owned() + "/"
        } else {
            uri.to_owned() + "/" + vhost
        }
    } else if huri.path() == "/" {
        if vhost == "/" {
            uri.to_owned()
        } else {
            uri.to_owned() + vhost
        }
    } else {
        uri.to_owned()
    }
}

pub(crate) async fn establish_connection(uri: &str, vhost: &str) -> lapin::Result<Connection> {
    let cp = ConnectionProperties::default();
    lapin::Connection::connect(&merge_uri_and_vhost(uri, vhost), cp).await
}

async fn publish_msg(c: &Connection) -> lapin::Result<Confirmation> {
    let ch = c.create_channel().await?;
    let cso = ConfirmSelectOptions { nowait: false };
    let _cs = ch.confirm_select(cso).await?;
    let bp = BasicProperties::default().with_delivery_mode(2);
    let bp = ch
        .basic_publish(
            "sms_blocklist.update_process",
            "sms_blocklist.update_process.trigger",
            BasicPublishOptions::default(),
            &[],
            bp,
        )
        .await?;
    bp.await
}

pub(crate) async fn publish_update_trigger_message(
    c: &Connection,
) -> Result<(), ServiceInvocationError> {
    let pcr = publish_msg(c).await;
    match pcr {
        Err(x) => Err(ServiceInvocationError::ContactGatewayMessageError(
            ContactGatewayMessagePublishError::PublishError(x),
        )),
        Ok(cr) => {
            if cr.is_ack() {
                Ok(())
            } else {
                Err(ServiceInvocationError::ContactGatewayMessageError(
                    ContactGatewayMessagePublishError::ConfirmationFailureError,
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::services::contact_gateway::merge_uri_and_vhost;

    #[test]
    fn test_uri_builder() {
        let result1 = merge_uri_and_vhost("amqp://guest:guest@localhost", "/");
        let result2 = merge_uri_and_vhost("amqp://guest:guest@localhost/", "/");
        let result3 = merge_uri_and_vhost("amqp://guest:guest@localhost", "event_source");
        let result4 = merge_uri_and_vhost("amqp://guest:guest@localhost/", "event_source");
        let result5 =
            merge_uri_and_vhost("amqp://guest:guest@localhost/event_source", "event_source");
        let result6 = merge_uri_and_vhost("amqp://guest:guest@localhost/", "");
        assert_eq!(result1, "amqp://guest:guest@localhost/");
        assert_eq!(result2, "amqp://guest:guest@localhost/");
        assert_eq!(result3, "amqp://guest:guest@localhost/event_source");
        assert_eq!(result4, "amqp://guest:guest@localhost/event_source");
        assert_eq!(result5, "amqp://guest:guest@localhost/event_source");
        assert_eq!(result6, "amqp://guest:guest@localhost/");
    }
}
