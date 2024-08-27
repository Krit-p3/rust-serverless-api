use aws_sdk_dynamodb::Client;
use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt,
};
use serde::Deserialize;
use serde_json;
use serverless_todos::domain::model::Repository;
use serverless_todos::infrastructure::db::Database;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .init();
    // use new function not load_from_env
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
    let uri = event.uri().path();

    let path_parts: Vec<&str> = uri.trim_start_matches('/').split('/').collect();

    if path_parts.len() == 3 && path_parts[1] == "todo" {
        let id = path_parts[2];

        match db.delete_todo(id).await {
            Ok(response) => {
                let res = Response::builder()
                    .status(
                        StatusCode::from_u16(response.status_code as u16).unwrap_or(StatusCode::OK),
                    )
                    .header("Content-Type", "application/json")
                    .body(
                        serde_json::json!({
                            "status_code": response.status_code,
                            "body": response.body,
                        })
                        .to_string(),
                    )
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
    } else {
        let res = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(
                serde_json::json!({
                    "error": "Invalid path format. Expected '/todo/{id}'."
                })
                .to_string(),
            )
            .map_err(Box::new)?;
        Ok(res)
    }
}
