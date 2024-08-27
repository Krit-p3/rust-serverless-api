use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ToDo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, PartialEq)]
pub struct ToDoResponse {
    pub id: Option<AttributeValue>,
    pub title: Option<AttributeValue>,
    pub completed: Option<AttributeValue>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub status_code: u32,
    pub body: String,
}

#[async_trait]
pub trait Repository {
    async fn create_todo(&self, todo: ToDo) -> Result<Response, aws_sdk_dynamodb::Error>;

    async fn update_todo(&self, todo: ToDo) -> Result<Response, aws_sdk_dynamodb::Error>;

    async fn read_todo(&self, id: &str) -> Result<Response, aws_sdk_dynamodb::Error>;

    async fn delete_todo(&self, id: &str) -> Result<Response, aws_sdk_dynamodb::Error>;
}


