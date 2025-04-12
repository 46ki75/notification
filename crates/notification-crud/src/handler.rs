use lambda_runtime::{Error, LambdaEvent};

pub async fn function_handler(
    event: LambdaEvent<crate::notification::Request>,
) -> Result<crate::notification::Response, Error> {
    // let req = crate::notification::Request::decode(&*event.payload)?;

    match event
        .payload
        .command
        .ok_or("Command is not set.".to_string())?
    {
        crate::notification::request::Command::PutCommand(put_command) => {
            let response = crate::operation::create::put(put_command).await?;

            Ok(crate::notification::Response {
                results: vec![response],
            })
        }
        crate::notification::request::Command::ListCommand(_list_command) => todo!(),
        crate::notification::request::Command::DeleteCommand(_delete_command) => todo!(),
    }
}
