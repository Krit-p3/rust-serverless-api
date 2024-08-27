use crate::domain::model::{Repository, Response, ToDo};
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::Error;

#[derive(Clone)]
pub struct Database {
    pub client: Client,
    pub table_name: String,
}

impl Database {
    pub async fn new(client: Client, table: String) -> Result<Database, Error> {
        Ok(Database {
            client,
            table_name: table,
        })
    }
}

#[async_trait]
impl Repository for Database {
    async fn create_todo(&self, todo: ToDo) -> Result<Response, Error> {
        self.client
            .put_item()
            .table_name(&self.table_name)
            .item("id", AttributeValue::S(todo.id.clone()))
            .item("title", AttributeValue::S(todo.title.clone()))
            .item("completed", AttributeValue::Bool(todo.completed.clone()))
            .send()
            .await?;

        let res_body = serde_json::json!({
           "id": todo.id,
            "title": todo.title,
            "completed": todo.completed
        });

        Ok(Response {
            status_code: 200,
            body: res_body.to_string(),
        })
    }

    async fn update_todo(&self, todo: ToDo) -> Result<Response, Error> {
        let key = "id";

        let res = self
            .client
            .update_item()
            .table_name(&self.table_name)
            .key(key, AttributeValue::S(todo.id))
            .update_expression("SET completed = :completed, title = :title")
            .expression_attribute_values(":completed", AttributeValue::Bool(todo.completed))
            .expression_attribute_values(":title", AttributeValue::S(todo.title))
            .return_values("ALL_NEW".into())
            .send()
            .await?;

        if let Some(attributes) = res.attributes {
            let default_string = "".to_string();
            let default_bool = false;

            let id = attributes
                .get("id")
                .and_then(|v| v.as_s().ok())
                .unwrap_or(&default_string); 

            let title = attributes
                .get("title")
                .and_then(|v| v.as_s().ok())
                .unwrap_or(&default_string); 

            let completed = attributes
                .get("completed")
                .and_then(|v| v.as_bool().ok())
                .unwrap_or(&default_bool); 
            
            let mut res_body = serde_json::Map::new();
            res_body.insert("id".to_string(), serde_json::json!(id));
            res_body.insert("title".to_string(), serde_json::json!(title));
            res_body.insert("completed".to_string(), serde_json::json!(completed));

            let res_body = serde_json::json!(res_body);

            Ok(Response {
                status_code: 200,
                body: res_body.to_string(), 
            })
        } else {
            Ok(Response {
                status_code: 404,
                body: "Todo not found".to_string(),
            })
        }

    }


    async fn read_todo(&self, id: &str) -> Result<Response, Error> {
        let key = "id";
        let res = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key(key, AttributeValue::S(id.to_string()))
            .send()
            .await?;

        if let Some(item) = res.item {
            let default_string = "".to_string();
            let default_bool = false;
            // Extract values from the DynamoDB item and provide defaults if needed
            let id = item
                .get("id")
                .and_then(|v| v.as_s().ok())
                .unwrap_or(&default_string);

            let title = item
                .get("title")
                .and_then(|v| v.as_s().ok())
                .unwrap_or(&default_string); 

            let completed = item
                .get("completed")
                .and_then(|v| v.as_bool().ok())
                .unwrap_or(&default_bool); 

            let mut res_body = serde_json::Map::new();
            res_body.insert("id".to_string(), serde_json::json!(id));
            res_body.insert("title".to_string(), serde_json::json!(title));
            res_body.insert("completed".to_string(), serde_json::json!(completed));

            let res_body = serde_json::json!(res_body);

            Ok(Response {
                status_code: 200,
                body: res_body.to_string(), 
            })
        } else {
            Ok(Response {
                status_code: 404,
                body: "Todo not found".to_string(),
            })
        }

    }

    async fn delete_todo(&self, id: &str) -> Result<Response, Error> {
        let key = "id";
        let res = self
            .client
            .delete_item()
            .table_name(&self.table_name)
            .key(key, AttributeValue::S(id.to_string()))
            .return_values("ALL_OLD".into())
            .send()
            .await?;

        if res.attributes.is_some() {
            let res_body = serde_json::json!({
            "message": "Todo deleted!"
            });

            Ok(Response {
                status_code: 200,
                body: res_body.to_string(),
            })
        } else {
            let res_body = serde_json::json!({
                "message": "Todo not found"
            });

            Ok(Response {
                status_code: 404,
                body: res_body.to_string(),
            })
        }
    }
}
