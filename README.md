# Tudu — Todo API (Rust + Axum)

A minimal, production-lean backend for a Todo application built with **Rust**, **Axum**, **Tokio**, **SQLx**, and **SQLite** (current storage). The goal is to keep the codebase small, fast, and easy to extend.

---

## Tech Stack

- **Rust** — strong type-safety, performance, and reliability.
- **Axum** — HTTP server framework built on Tower/Hyper.
- **Tokio** — async runtime powering I/O concurrency.
- **SQLx** — async, compile-time checked SQL (when using `query!` macros) and a clean DB layer.
- **SQLite** — embedded database for simple local dev + single-instance deployments.

---

## Features (current / intended)

- CRUD for todos
- Health check endpoint
- SQLite persistence via SQLx
- Async execution (Tokio)

---

## Configuration

Use environment variables (or a `.env` file):

```bash
DATABASE_URL=sqlite://tudu.db
HOST=127.0.0.1
PORT=3000
```

Notes:

* For SQLite, `DATABASE_URL` typically looks like `sqlite://file.db`.
* If you want an in-memory DB for tests/dev: `sqlite::memory:` (depending on your setup).

---

## Run Locally

```bash
cargo run
```

Example output expectation:

* Server listening on `http://127.0.0.1:3000`

---

## API Endpoints

Base URL: `http://127.0.0.1:3000`

### 1) Health Check

**GET** `/health`

**Response 200**

```json
{
  "status": "ok"
}
```

---

### 2) Create Todo

**POST** `/api/todos`

**Request body**

```json
{
  "category_id": 1,
  "title": "Buy milk",
}
```

**Response 201**

```json
{
  "id": 1,
  "user_id": 1,
  "category_id": 1,
  "title": "Buy milk",
  "completed": false,
  "created_at": "2026-02-19T17:30:00Z"
}
```

**Notes**

* `id` can be an integer (SQLite autoincrement) or a string/UUID/ULID—whatever your model uses.
* Add validation rules as needed (e.g., title length, trimming, non-empty).

---

## Development Notes

* Prefer a single shared `sqlx::SqlitePool` stored in app state.
* Keep handlers thin: parse input → call service/repo → map result to HTTP response.
* Use `tracing` for structured logs (recommended if you add logging).

---

## Roadmap

* [ ] List todos (pagination)
* [x] Get todo by id
* [x] Update todo
* [x] Delete todo
* [x] Authentication
* [ ] OpenAPI / Swagger docs (optional)
* [ ] Switchable DB (Postgres later)

---
