async fn invoke(
    sdk_config: &aws_config::SdkConfig,
    stage_name: &str,
    input: crate::r#type::Input,
) -> Result<crate::r#type::NotificationResult, Box<dyn std::error::Error>> {
    let client = aws_sdk_lambda::Client::new(sdk_config);

    let body = serde_json::to_string(&input)?;

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

    let result = serde_json::from_slice::<crate::r#type::NotificationResult>(&response)?;

    Ok(result)
}

pub async fn put(
    sdk_config: &aws_config::SdkConfig,
    stage_name: &str,
    input: crate::r#type::PutParameter,
) -> Result<crate::r#type::Notification, Box<dyn std::error::Error>> {
    let response = invoke(sdk_config, stage_name, crate::r#type::Input::Put(input)).await?;

    match response {
        crate::r#type::NotificationResult::Single(notification) => Ok(notification),
        crate::r#type::NotificationResult::Vector(_) => {
            Err("Expected a single notification, but got multiple.".into())
        }
    }
}
