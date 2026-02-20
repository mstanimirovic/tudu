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

# Authentication

## POST `/auth/register`

Creates new user.

### Request

```json
{
  "email": "user@example.com",
  "password": "secret123",
  "name": "Mladen"
}
```

### Success

* `201 Created`

```json
{
  "token": "jwt_token",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "name": "Mladen",
    "created_at": "timestamp",
    "updated_at": "timestamp"
  }
}
```

### Errors

* `422 Unprocessable Entity` – validation
* `409 Conflict` – email already exists
* `400 Bad Request` – invalid JSON

---

## POST `/auth/login`

User login.

### Request

```json
{
  "email": "user@example.com",
  "password": "secret123"
}
```

### Success

* `200 OK`

```json
{
  "token": "jwt_token",
  "user": { ...UserPublic }
}
```

### Errors

* `401 Unauthorized` – wrong email or password
* `422` – validation

---

# Users

## GET `/users/me` 🔐

Returns currenctly logged in user.

### Success

* `200 OK`

```json
{ ...UserPublic }
```

### Errors

* `401 Unauthorized`

---

# Categories (JWT required)

## GET `/categories` 🔐

List categories for logged in user.

### Success

* `200 OK`

```json
{
  "items": [ ...Category ]
}
```

---

## POST `/categories` 🔐

Creates new category.

### Request

```json
{
  "name": "Work",
  "color": "#3B82F6"
}
```

### Success

* `201 Created`

```json
{ ...Category }
```

### Errors

* `422` – validation
* `409` – name duplicate
* `401` – auth fail

---

## PATCH `/categories/:id` 🔐

Update category.

### Request

```json
{
  "name": "Personal",
  "color": "#22C55E"
}
```

### Success

* `200 OK`

### Errors

* `404 Not Found`
* `422`
* `401`

---

## DELETE `/categories/:id` 🔐

### Success

* `204 No Content`

### Errors

* `404`
* `401`
* `409` (if there are todos linked with this category)

---

# Todos (JWT required)

## GET `/todos` 🔐

List todo items.

Query parameters (optional):

```
?status=open|done
?category_id=uuid
?limit=50
```

### Success

* `200 OK`

```json
{
  "items": [ ...Todo ]
}
```

---

## POST `/todos` 🔐

Creates todo.

### Request

```json
{
  "title": "Buy milk",
  "description": "2L",
  "category_id": "uuid",
  "priority": 2,
  "due_at": "timestamp"
}
```

### Success

* `201 Created`

```json
{ ...Todo }
```

### Errors

* `422`
* `404` (category doesn't exists)
* `401`

---

## GET `/todos/:id` 🔐

### Success

* `200 OK`

```json
{ ...Todo }
```

### Errors

* `404`
* `401`

---

## PATCH `/todos/:id` 🔐

Partial update.

### Request

```json
{
  "title": "Buy oat milk",
  "done": true
}
```

### Success

* `200 OK`

```json
{ ...Todo }
```

### Errors

* `404`
* `422`
* `401`

---

## DELETE `/todos/:id` 🔐

### Success

* `204 No Content`

### Errors

* `404`
* `401`

---

# Error Format

```json
{
  "error": "Human readable message"
}
```

---

# Auth Header Example

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```
