use serde::{Deserialize, Serialize};

use crate::users::{command::CreateUser, dto::UserDto, model::User};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserDto,
}

impl AuthResponse {
    pub fn new(token: String, user: User) -> Self {
        Self {
            token: token,
            user: UserDto::from(user),
        }
    }
}

impl Into<CreateUser> for RegisterRequest {
    fn into(self) -> CreateUser {
        CreateUser {
            name: self.name,
            email: self.email,
            password: self.password,
        }
    }
}
