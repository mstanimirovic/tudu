#[derive(Debug, Clone)]
pub struct CreateTodo {
    pub category_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub priority: i64,
    pub due_at: i64,
}

#[derive(Debug, Clone)]
pub struct UpdateTodo {
    pub category_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
    pub priority: Option<i64>,
    pub due_at: Option<i64>,
}
