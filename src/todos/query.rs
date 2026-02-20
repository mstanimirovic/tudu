use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TodosQuery {
    pub status: Option<TodoStatusQuery>, // open|done
    pub category_id: Option<i64>,
    pub q: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub sort: Option<TodoSortQuery>,   // created_at|due_at|priority
    pub order: Option<SortOrderQuery>, // asc|desc
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum TodoStatusQuery {
    Open,
    Done,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TodoSortQuery {
    CreatedAt,
    DueAt,
    Priority,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SortOrderQuery {
    Asc,
    Desc,
}
