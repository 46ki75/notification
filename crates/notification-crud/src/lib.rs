pub mod operation;
pub mod r#type;

use lambda_runtime::{Error, LambdaEvent};

pub async fn function_handler(
    event: LambdaEvent<crate::r#type::Input>,
) -> Result<Vec<crate::r#type::Notification>, Error> {
    match event.payload {
        r#type::Input::Create(create_parameter) => {
            let response = crate::operation::create::create(create_parameter).await?;
            return Ok(vec![response]);
        }
    };
}
