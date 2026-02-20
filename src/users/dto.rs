use serde::{Deserialize, Serialize};

use crate::users::command::UpdateUser;

use super::model::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDto {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl Into<UpdateUser> for UpdateUserRequest {
    fn into(self) -> UpdateUser {
        UpdateUser {
            name: self.name,
            email: self.email,
        }
    }
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
