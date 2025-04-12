use prost::Message;

async fn invoke(
    sdk_config: &aws_config::SdkConfig,
    stage_name: &str,
    request: crate::notification::Request,
) -> Result<crate::notification::Response, Box<dyn std::error::Error>> {
    let client = aws_sdk_lambda::Client::new(sdk_config);

    let mut body: Vec<u8> = Vec::new();

    request.encode(&mut body)?;

    let blob = aws_sdk_lambda::primitives::Blob::from(body);

    let request = client
        .invoke()
        .function_name(format!(
            "{}-46ki75-notification-lambda-function-notification-crud",
            stage_name
        ))
        .qualifier("stable")
        .payload(blob);

    let response = request
        .send()
        .await?
        .payload
        .ok_or("Lambda response payload is empty.")?
        .into_inner();

    let result = crate::notification::Response::decode(&*response)?;

    Ok(result)
}

pub async fn put(
    sdk_config: &aws_config::SdkConfig,
    stage_name: &str,
    command: crate::notification::PutCommand,
) -> Result<crate::notification::Response, Box<dyn std::error::Error>> {
    let request = crate::notification::Request {
        command: Some(crate::notification::request::Command::PutCommand(command)),
    };

    let response = invoke(sdk_config, stage_name, request).await?;

    Ok(response)
}
