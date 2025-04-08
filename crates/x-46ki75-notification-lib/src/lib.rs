pub mod record;

#[async_trait::async_trait]
pub trait NotificationClient {
    async fn put(
        parameter: record::PutParameter,
    ) -> Result<Vec<record::Notification>, Box<dyn std::error::Error>>;

    async fn delete(
        parameter: record::DeleteParameter,
    ) -> Result<Vec<record::Notification>, Box<dyn std::error::Error>>;

    async fn fetch(
        parameter: record::FetchParameter,
    ) -> Result<Vec<record::Notification>, Box<dyn std::error::Error>>;
}
