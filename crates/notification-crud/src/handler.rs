use lambda_runtime::{Error, LambdaEvent};
use prost::Message;

pub async fn function_handler(event: LambdaEvent<Vec<u8>>) -> Result<Vec<u8>, Error> {
    let req = crate::notification::Request::decode(&*event.payload)?;

    match req.command.ok_or("Command is not set.".to_string())? {
        crate::notification::request::Command::PutCommand(put_command) => {
            let mut buf: Vec<u8> = Vec::new();

            let response = crate::operation::create::put(put_command).await?;

            crate::notification::Response {
                results: vec![response],
            }
            .encode(&mut buf)?;

            Ok(buf)
        }
        crate::notification::request::Command::ListCommand(_list_command) => todo!(),
        crate::notification::request::Command::DeleteCommand(_delete_command) => todo!(),
    }
}
