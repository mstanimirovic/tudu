use serde::{Deserialize, Serialize};

use crate::categories::{
    command::{CreateCategory, UpdateCategory},
    model::Category,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryDto {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub color: Option<String>,
}

impl Into<CreateCategory> for CreateCategoryRequest {
    fn into(self) -> CreateCategory {
        CreateCategory {
            name: self.name,
            color: self.color,
        }
    }
}

impl Into<UpdateCategory> for UpdateCategoryRequest {
    fn into(self) -> UpdateCategory {
        UpdateCategory {
            name: self.name,
            color: self.color,
        }
    }
}

impl From<&Category> for CategoryDto {
    fn from(value: &Category) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            color: value.color.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Category> for CategoryDto {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
            color: value.color,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
