use lambda_runtime::{Error, LambdaEvent};

pub async fn function_handler(
    event: LambdaEvent<crate::notification::Request>,
) -> Result<crate::notification::Response, Error> {
    match event.payload {
        crate::notification::Request::Put(put_command) => {
            let res = crate::operation::put(put_command).await?;
            return Ok(crate::notification::Response { results: res });
        }
        crate::notification::Request::List(list_command) => {
            let res = crate::operation::list(list_command).await?;
            return Ok(crate::notification::Response { results: res });
        }
        crate::notification::Request::Delete(delete_command) => {
            let res = crate::operation::delete(delete_command).await?;
            return Ok(crate::notification::Response { results: res });
        }
    }
}
