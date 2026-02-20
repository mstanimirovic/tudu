#[derive(Debug, Clone)]
pub struct CreateCategory {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub color: Option<String>,
}
