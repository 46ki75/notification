use lambda_runtime::{Error, LambdaEvent, run, service_fn, tracing};

pub mod operation;

pub mod notification {
    include!("./generated/notification.rs");
}

pub async fn function_handler(
    event: LambdaEvent<crate::notification::Request>,
) -> Result<crate::notification::Response, Error> {
    match event
        .payload
        .command
        .ok_or("Command is not set.".to_string())?
    {
        notification::request::Command::PutCommand(put_command) => {
            let response = crate::operation::create::put(put_command).await?;
            return Ok(notification::Response {
                results: vec![response],
            });
        }
        notification::request::Command::ListCommand(_list_command) => todo!(),
        notification::request::Command::DeleteCommand(_delete_command) => todo!(),
    };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
