use crate::{
    todos::repository::TodoRepository,
    users::{models::User, repository::UserRepository},
};

#[derive(Clone)]
pub struct AppState {
    pub users_repo: UserRepository,
    pub todos_repo: TodoRepository,
}

impl AppState {
    pub fn new(ur: UserRepository, tr: TodoRepository) -> Self {
        Self {
            users_repo: ur,
            todos_repo: tr,
        }
    }
}
