use crate::models::todo::{CreateTodoRequest, Todo, UpdateTodoRequest};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct TodoRepository {
    pool: SqlitePool,
}

impl TodoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: i64,
        payload: CreateTodoRequest,
    ) -> Result<Todo, sqlx::Error> {
        sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (user_id, category_id, title, description, priority, due_at) VALUES (?, ?, ?, ?, ?, ?) RETURNING *",
        )
        .bind(user_id)
        .bind(payload.category_id)
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(payload.priority)
        .bind(payload.due_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn find_all_by_user(&self, user_id: i64) -> Result<Vec<Todo>, sqlx::Error> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Todo>, sqlx::Error> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update(
        &self,
        id: i64,
        payload: UpdateTodoRequest,
    ) -> Result<Option<Todo>, sqlx::Error> {
        if let Some(category_id) = &payload.category_id {
            sqlx::query("UPDATE todos SET category_id = ? WHERE id = ?")
                .bind(category_id)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(title) = &payload.title {
            sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
                .bind(title)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(description) = &payload.description {
            sqlx::query("UPDATE todos SET description = ? WHERE id = ?")
                .bind(description)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(done) = &payload.done {
            sqlx::query("UPDATE todos SET done = ? WHERE id = ?")
                .bind(done)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(priority) = &payload.priority {
            sqlx::query("UPDATE todos SET priority = ? WHERE id = ?")
                .bind(priority)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(due_at) = &payload.due_at {
            sqlx::query("UPDATE todos SET due_at = ? WHERE id = ?")
                .bind(due_at)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
