#[tokio::test]
async fn invoke() {
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let res = x_46ki75_notification_lib::invoke::put(
        &sdk_config,
        x_46ki75_notification_lib::notification::StageName::Development,
        x_46ki75_notification_lib::notification::PutCommand {
            title: Some("Integration Test".to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!("{:?}", res);
}
