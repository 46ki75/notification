pub async fn put(
    command: crate::notification::PutCommand,
) -> Result<crate::notification::Notification, lambda_runtime::Error> {
    let stage_name = std::env::var("STAGE_NAME")?;

    let table_name = format!(
        "{}-46ki75-notification-dynamodb-table-notification",
        stage_name
    );

    let aws_sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let dynamodb_client = std::sync::Arc::new(aws_sdk_dynamodb::Client::new(&aws_sdk_config));

    let pk = uuid::Uuid::new_v4().to_string();

    let notification = crate::notification::Notification {
        pk,
        title: command.title,
        details: command.details,
        status: command.status.unwrap_or_default(),
        severity: command.severity.unwrap_or_default(),
        notified_at: chrono::Utc::now().to_rfc3339(),
        url: command.url,
    };

    let notification_cloned = notification.clone();

    let request = dynamodb_client
        .put_item()
        .table_name(table_name)
        .item(
            "PK",
            aws_sdk_dynamodb::types::AttributeValue::S(notification.pk),
        )
        .item(
            "title",
            aws_sdk_dynamodb::types::AttributeValue::S(notification.title),
        )
        .item(
            "details",
            aws_sdk_dynamodb::types::AttributeValue::S(
                notification.details.clone().unwrap_or_default(),
            ),
        )
        .item(
            "status",
            aws_sdk_dynamodb::types::AttributeValue::S(notification.status.to_string()),
        )
        .item(
            "severity",
            aws_sdk_dynamodb::types::AttributeValue::S(notification.severity.to_string()),
        )
        .item(
            "url",
            aws_sdk_dynamodb::types::AttributeValue::S(
                notification.url.unwrap_or_default().to_string(),
            ),
        );

    let _response = request.send().await?;

    Ok(notification_cloned)
}
