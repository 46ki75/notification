use lambda_runtime::{Error, LambdaEvent, run, service_fn, tracing};
use prost::Message;

pub mod operation;

pub mod notification {
    include!("./generated/notification.rs");
}

pub async fn function_handler(event: LambdaEvent<Vec<u8>>) -> Result<Vec<u8>, Error> {
    let req = notification::Request::decode(&*event.payload)?;

    match req.command.ok_or("Command is not set.".to_string())? {
        notification::request::Command::PutCommand(put_command) => {
            let mut buf: Vec<u8> = Vec::new();

            let response = crate::operation::create::put(put_command).await?;

            notification::Response {
                results: vec![response],
            }
            .encode(&mut buf)?;

            Ok(buf)
        }
        notification::request::Command::ListCommand(_list_command) => todo!(),
        notification::request::Command::DeleteCommand(_delete_command) => todo!(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
