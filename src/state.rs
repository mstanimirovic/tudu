use crate::{
    repositories::category_repo::CategoryRepository, repositories::todo_repo::TodoRepository,
    repositories::user_repo::UserRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub users_repo: UserRepository,
    pub todos_repo: TodoRepository,
    pub categories_repo: CategoryRepository,
}

impl AppState {
    pub fn new(ur: UserRepository, tr: TodoRepository, ct: CategoryRepository) -> Self {
        Self {
            users_repo: ur,
            todos_repo: tr,
            categories_repo: ct,
        }
    }
}
