use super::model::Todo;
use crate::todos::{
    command::{CreateTodo, UpdateTodo},
    filter::{TodosFilter, order_dir, sort_column},
};
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

#[derive(Clone)]
pub struct TodoRepository {
    pool: SqlitePool,
}

impl TodoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user_id: i64, payload: CreateTodo) -> Result<Todo, sqlx::Error> {
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

    pub async fn find_many(
        &self,
        user_id: i64,
        filter: TodosFilter,
    ) -> Result<Vec<Todo>, sqlx::Error> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
                SELECT
                    id, user_id, category_id, title, description,
                    done, priority, due_at, created_at, updated_at
                FROM todos
                WHERE user_id =
                "#,
        );

        qb.push_bind(user_id);

        if let Some(done) = filter.done {
            qb.push(" AND done = ");
            qb.push_bind(if done { true } else { false });
        }

        if let Some(category_id) = filter.category_id {
            qb.push(" AND category_id = ");
            qb.push_bind(category_id);
        }

        if let Some(q) = filter.q.as_deref().filter(|s| !s.trim().is_empty()) {
            let pat = format!("%{}%", q.trim());
            qb.push(" AND (title LIKE ");
            qb.push_bind(pat.clone());
            qb.push(" OR description LIKE ");
            qb.push_bind(pat.clone());
            qb.push(")");
        }

        qb.push(" ORDER BY ");
        qb.push(sort_column(filter.sort));
        qb.push(" ");
        qb.push(order_dir(filter.order));

        qb.push(" LIMIT ");
        qb.push_bind(filter.limit as i64);
        qb.push(" OFFSET ");
        qb.push_bind(filter.offset as i64);

        Ok(qb.build_query_as::<Todo>().fetch_all(&self.pool).await?)
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

    pub async fn update(&self, id: i64, payload: UpdateTodo) -> Result<Option<Todo>, sqlx::Error> {
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
