use lambda_runtime::{Error, run, service_fn, tracing};

mod handler;

pub mod notification;
pub mod operation;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(handler::function_handler)).await
}
