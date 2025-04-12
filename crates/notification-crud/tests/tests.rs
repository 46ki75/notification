#[tokio::test]
async fn invoke() {
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let res = x_46ki75_notification_lib::invoke::put(
        &sdk_config,
        "dev",
        x_46ki75_notification_lib::notification::PutCommand {
            title: "Integration Test".to_string(),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!("{:?}", res);
}
