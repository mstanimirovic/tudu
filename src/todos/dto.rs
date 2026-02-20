use serde::{Deserialize, Serialize};

use crate::todos::{
    command::{CreateTodo, UpdateTodo},
    model::Todo,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoDto {
    pub id: i64,
    pub category_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub priority: i64,
    pub due_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoRequest {
    pub category_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub priority: i64,
    pub due_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodoRequest {
    pub category_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
    pub priority: Option<i64>,
    pub due_at: Option<i64>,
}

impl Into<CreateTodo> for CreateTodoRequest {
    fn into(self) -> CreateTodo {
        CreateTodo {
            category_id: self.category_id,
            title: self.title,
            description: self.description,
            priority: self.priority,
            due_at: self.due_at,
        }
    }
}

impl Into<UpdateTodo> for UpdateTodoRequest {
    fn into(self) -> UpdateTodo {
        UpdateTodo {
            category_id: self.category_id,
            title: self.title,
            description: self.description,
            done: self.done,
            priority: self.priority,
            due_at: self.due_at,
        }
    }
}

impl From<Todo> for TodoDto {
    fn from(value: Todo) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            title: value.title,
            description: value.description,
            done: value.done,
            priority: value.priority,
            due_at: value.due_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<&Todo> for TodoDto {
    fn from(value: &Todo) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            title: value.title.clone(),
            description: value.description.clone(),
            done: value.done,
            priority: value.priority,
            due_at: value.due_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
