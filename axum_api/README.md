# Axum Task API

A lightweight Rust REST API built with Axum for managing a simple in-memory task list. It demonstrates common API patterns using `axum`, `tokio`, and `serde`, with a small state model that persists only for the lifetime of the running process.

## Features

- Create, read, update, and delete tasks
- In-memory storage with a generated task ID sequence
- JSON request and response payloads
- Health and application status endpoints
- Built with Rust async runtime and Axum routing

## Project Structure

- `src/main.rs` — application setup, routes, handlers, and in-memory state
- `Cargo.toml` — Rust package metadata and dependencies
- `test_api.http` — sample HTTP requests for testing in VS Code or similar tools

## Tech Stack

- Rust 2024 edition
- Axum 0.8
- Tokio async runtime
- Serde for JSON serialization/deserialization

## Prerequisites

Before running the project, ensure you have:

- Rust installed (recommended via `rustup`)
- Cargo available on your PATH

## Getting Started

From the project root:

```bash
cargo run
```

The server listens on:

```text
http://localhost:3000
```

When the server starts, it prints:

```text
Running on http://localhost:3000
```

## API Endpoints

| Method | Endpoint | Description |
| --- | --- | --- |
| GET | `/` | Returns a simple application status message |
| GET | `/health` | Returns `Ok` for health checks |
| GET | `/tasks` | Returns all tasks |
| POST | `/tasks` | Creates a new task |
| GET | `/tasks/{id}` | Fetches a single task by ID |
| PATCH | `/tasks/{id}` | Updates a task |
| DELETE | `/tasks/{id}` | Removes a task |

### Request and Response Format

Tasks are returned as JSON objects like:

```json
{
  "id": 1,
  "title": "Learn Axum",
  "done": false
}
```

Create a task with:

```json
{
  "title": "Write API docs"
}
```

Update a task with partial fields:

```json
{
  "title": "Write API docs",
  "done": true
}
```

## Example Requests

### Check the app is running

```bash
curl http://localhost:3000/
```

Response:

```text
app is live
```

### Check health

```bash
curl http://localhost:3000/health
```

Response:

```text
Ok
```

### List all tasks

```bash
curl http://localhost:3000/tasks
```

### Create a task

```bash
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"Write API docs"}'
```

### Get a task by ID

```bash
curl http://localhost:3000/tasks/1
```

### Update a task

```bash
curl -X PATCH http://localhost:3000/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Write API docs","done":true}'
```

### Delete a task

```bash
curl -X DELETE http://localhost:3000/tasks/1
```

## Notes

- The data store is intentionally in-memory and resets when the server restarts.
- Successful creation returns HTTP status `201 Created`.
- Missing tasks return HTTP status `404 Not Found`.
- Deleting a task returns `204 No Content` on success.

## License

This project is provided for educational and demonstration purposes.
