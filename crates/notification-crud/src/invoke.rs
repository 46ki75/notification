async fn invoke(
    sdk_config: &aws_config::SdkConfig,
    stage_name: &str,
    request: crate::notification::Request,
) -> Result<crate::notification::Response, Box<dyn std::error::Error>> {
    let client = aws_sdk_lambda::Client::new(sdk_config);

    let body = serde_json::to_string(&request)?;

    let blob = aws_sdk_lambda::primitives::Blob::from(body.as_bytes());

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

    let results = serde_json::from_slice::<Vec<crate::notification::Notification>>(&response)?;

    Ok(crate::notification::Response { results })
}

pub async fn put(
    sdk_config: &aws_config::SdkConfig,
    stage_name: &str,
    command: crate::notification::PutCommand,
) -> Result<crate::notification::Response, Box<dyn std::error::Error>> {
    let request = crate::notification::Request::Put(command);

    let response = invoke(sdk_config, stage_name, request).await?;

    Ok(response)
}

pub async fn delete(
    sdk_config: &aws_config::SdkConfig,
    stage_name: &str,
    command: crate::notification::DeleteCommand,
) -> Result<crate::notification::Response, Box<dyn std::error::Error>> {
    let request = crate::notification::Request::Delete(command);

    let response = invoke(sdk_config, stage_name, request).await?;

    Ok(response)
}

pub async fn list(
    sdk_config: &aws_config::SdkConfig,
    stage_name: &str,
    command: crate::notification::ListCommand,
) -> Result<crate::notification::Response, Box<dyn std::error::Error>> {
    let request = crate::notification::Request::List(command);

    let response = invoke(sdk_config, stage_name, request).await?;

    Ok(response)
}
