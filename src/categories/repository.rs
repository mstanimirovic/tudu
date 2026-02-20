use crate::categories::{
    command::{CreateCategory, UpdateCategory},
    model::Category,
};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct CategoryRepository {
    pool: SqlitePool,
}

impl CategoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: i64,
        payload: CreateCategory,
    ) -> Result<Category, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "INSERT INTO categories (user_id, name, color) VALUES (?, ?, ?) RETURNING *",
        )
        .bind(user_id)
        .bind(&payload.name)
        .bind(&payload.color)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_all(&self) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn find_all_by_user(&self, user_id: i64) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update(
        &self,
        id: i64,
        payload: UpdateCategory,
    ) -> Result<Option<Category>, sqlx::Error> {
        if let Some(name) = &payload.name {
            sqlx::query("UPDATE categories SET name = ? WHERE id = ?")
                .bind(name)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(color) = &payload.color {
            sqlx::query("UPDATE categories SET color = ? WHERE id = ?")
                .bind(color)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
