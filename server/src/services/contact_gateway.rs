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

pub(crate) async fn establish_connection(uri: &str) -> lapin::Result<Connection> {
    let cp = ConnectionProperties::default();
    lapin::Connection::connect(uri, cp).await
}

async fn publish_msg(c: &Connection) -> lapin::Result<Confirmation> {
    let ch = c.create_channel().await?;
    let cso = ConfirmSelectOptions { nowait: true };
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
