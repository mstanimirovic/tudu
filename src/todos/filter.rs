use crate::todos::query::{SortOrderQuery, TodoSortQuery, TodoStatusQuery, TodosQuery};

#[derive(Debug, Clone)]
pub struct TodosFilter {
    pub done: Option<bool>,
    pub category_id: Option<i64>,
    pub q: Option<String>,
    pub limit: u32,
    pub offset: u32,
    pub sort: TodoSort,
    pub order: SortOrder,
}

#[derive(Debug, Clone, Copy)]
pub enum TodoSort {
    CreatedAt,
    DueAt,
    Priority,
}

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl TodosFilter {
    pub fn from_query(q: TodosQuery) -> Self {
        let limit = q.limit.unwrap_or(50).min(100);
        let offset = q.offset.unwrap_or(0);

        let done = q.status.map(|s| match s {
            TodoStatusQuery::Open => false,
            TodoStatusQuery::Done => true,
        });

        let sort = match q.sort.unwrap_or(TodoSortQuery::CreatedAt) {
            TodoSortQuery::CreatedAt => TodoSort::CreatedAt,
            TodoSortQuery::DueAt => TodoSort::DueAt,
            TodoSortQuery::Priority => TodoSort::Priority,
        };

        let order = match q.order.unwrap_or(SortOrderQuery::Desc) {
            SortOrderQuery::Asc => SortOrder::Asc,
            SortOrderQuery::Desc => SortOrder::Desc,
        };

        Self {
            done,
            category_id: q.category_id,
            q: q.q,
            limit,
            offset,
            sort,
            order,
        }
    }
}

pub fn sort_column(sort: TodoSort) -> &'static str {
    match sort {
        TodoSort::CreatedAt => "created_at",
        TodoSort::DueAt => "due_at",
        TodoSort::Priority => "priority",
    }
}

pub fn order_dir(order: SortOrder) -> &'static str {
    match order {
        SortOrder::Asc => "ASC",
        SortOrder::Desc => "DESC",
    }
}
