use aws_sdk_dynamodb::Client;
use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestPayloadExt,
};
use serde_json;
use serverless_todos::domain::model::Repository;
use serverless_todos::domain::model::ToDo;
use serverless_todos::infrastructure::db::Database;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let client = Client::new(&config);
    let db = Database::new(client, table_name).await?;

    run(service_fn(move |event: Request| {
        let _db = db.clone();
        async move { function_handler(event, &_db).await }
    }))
    .await
}

async fn function_handler(event: Request, db: &Database) -> Result<impl IntoResponse, Error> {
    let todo: ToDo = event
        .payload()?
        .ok_or_else(|| Error::from("Missing request body"))?;

    match db.create_todo(todo).await {
        Ok(response) => {
            let res = Response::builder()
                .status(StatusCode::from_u16(response.status_code as u16).unwrap_or(StatusCode::OK))
                .header("Content-Type", "application/json")
                .body(response.body)
                .map_err(Box::new)?;
            Ok(res)
        }
        Err(e) => {
            let res = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::json!({
                        "error": format!("Error creating todo: {:?}", e)
                    })
                    .to_string(),
                )
                .map_err(Box::new)?;
            Ok(res)
        }
    }
}
