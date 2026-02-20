#[derive(Debug, Clone)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
