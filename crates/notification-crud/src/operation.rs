pub async fn put(
    command: crate::notification::PutCommand,
) -> Result<Vec<crate::notification::Notification>, lambda_runtime::Error> {
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
        title: command.title.unwrap_or_default(),
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
        )
        .item(
            "notifiedAt",
            aws_sdk_dynamodb::types::AttributeValue::S(notification.notified_at),
        );

    let _response = request.send().await?;

    Ok(vec![notification_cloned])
}

pub async fn list(
    command: crate::notification::ListCommand,
) -> Result<Vec<crate::notification::Notification>, lambda_runtime::Error> {
    let stage_name = std::env::var("STAGE_NAME")?;

    let table_name = format!(
        "{}-46ki75-notification-dynamodb-table-notification",
        stage_name
    );

    let aws_sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let dynamodb_client = std::sync::Arc::new(aws_sdk_dynamodb::Client::new(&aws_sdk_config));

    let mut results = Vec::new();

    for status in command.status {
        let request = dynamodb_client
            .query()
            .table_name(&table_name)
            .index_name("by-status")
            .key_condition_expression("#status = :status")
            .expression_attribute_names("#status", "status")
            .expression_attribute_values(
                ":status",
                aws_sdk_dynamodb::types::AttributeValue::S(status.to_string()),
            );

        for item in request.send().await?.items() {
            let notification = crate::notification::Notification {
                pk: item
                    .get("PK")
                    .ok_or("PK does not exist in DB.")?
                    .as_s()
                    .map_err(|e| format!("PK is not type `S`: {:?}", e))?
                    .to_owned(),
                title: item
                    .get("title")
                    .ok_or("title does not exist in DB.")?
                    .as_s()
                    .map_err(|e| format!("title is not type `S`: {:?}", e))?
                    .to_owned(),
                details: item
                    .get("details")
                    .and_then(|d| d.as_s().ok().map(|dd| dd.to_owned())),
                severity: crate::notification::Severity::from(
                    item.get("severity")
                        .and_then(|s| s.as_s().ok().map(|s| s.to_owned()))
                        .unwrap_or_else(|| "INFO".to_string())
                        .to_owned(),
                ),
                status: crate::notification::Status::from(
                    item.get("status")
                        .and_then(|s| s.as_s().ok().map(|s| s.to_owned()))
                        .unwrap_or_else(|| "NEW".to_string())
                        .to_owned(),
                ),
                url: item
                    .get("url")
                    .and_then(|url| url.as_s().ok().map(|url| url.to_owned())),
                notified_at: item
                    .get("notifiedAt")
                    .ok_or("notifiedAt does not exist in DB.")?
                    .as_s()
                    .map_err(|e| format!("notifiedAt is not type `S`: {:?}", e))?
                    .to_owned(),
            };

            results.push(notification);
        }
    }

    Ok(results)
}

pub async fn delete(
    command: crate::notification::DeleteCommand,
) -> Result<Vec<crate::notification::Notification>, lambda_runtime::Error> {
    let stage_name = std::env::var("STAGE_NAME")?;

    let table_name = format!(
        "{}-46ki75-notification-dynamodb-table-notification",
        stage_name
    );

    let aws_sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let dynamodb_client = std::sync::Arc::new(aws_sdk_dynamodb::Client::new(&aws_sdk_config));

    let request = dynamodb_client
        .delete_item()
        .table_name(table_name)
        .key("PK", aws_sdk_dynamodb::types::AttributeValue::S(command.pk));

    request.send().await?;

    Ok(vec![])
}
