use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

pub mod operation;
pub mod r#type;

pub async fn function_handler(
    event: LambdaEvent<crate::r#type::Input>,
) -> Result<crate::r#type::NotificationResult, Error> {
    match event.payload {
        r#type::Input::Put(put_parameter) => {
            let response = crate::operation::create::put(put_parameter).await?;
            return Ok(r#type::NotificationResult::Single(response));
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
