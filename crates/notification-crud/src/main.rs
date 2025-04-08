use lambda_runtime::{run, service_fn, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(notification_crud::function_handler)).await
}
