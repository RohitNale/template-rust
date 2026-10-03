# Actix Todo API

A lightweight Rust REST API for managing a simple in-memory todo list using Actix Web. This project demonstrates the core patterns for building a JSON-based API in Rust, including route handlers, request/response models, shared application state, and async server startup.

## Features

- Create new todo items
- Retrieve all todo items
- Update an existing todo by ID
- Delete a todo by ID
- In-memory storage for fast local testing and demo work
- JSON request and response handling with Serde

## Project Structure

- `src/main.rs` — application setup, data models, routes, and handlers
- `Cargo.toml` — Rust package configuration and dependencies
- `test_api.http` — example HTTP requests for quick API testing

## Tech Stack

- Rust 2024 edition
- Actix Web 4
- Serde for JSON serialization and deserialization

## Prerequisites

Before running the project, make sure you have:

- Rust and Cargo installed on your machine
- A terminal or IDE capable of running Rust projects

## Getting Started

From the project directory, run:

```bash
cargo run
```

The API will start on:

```text
http://127.0.0.1:3000
```

## API Endpoints

| Method | Endpoint      | Description              |
| ------ | ------------- | ------------------------ |
| POST   | `/todos`      | Create a new todo item   |
| GET    | `/todos`      | Retrieve all todo items  |
| PUT    | `/todos/{id}` | Update a todo item by ID |
| DELETE | `/todos/{id}` | Remove a todo item by ID |

### Todo Object

```json
{
  "id": 1,
  "title": "Learn Actix Web",
  "completed": false
}
```

## Example Requests

### Create a todo

```bash
curl -X POST http://127.0.0.1:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"Learn Actix Web"}'
```

### Fetch all todos

```bash
curl http://127.0.0.1:3000/todos
```

### Update a todo

```bash
curl -X PUT http://127.0.0.1:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Learn Actix Web","completed":true}'
```

### Delete a todo

```bash
curl -X DELETE http://127.0.0.1:3000/todos/1
```

## Testing with VS Code HTTP Client

The repository includes a `test_api.http` file with ready-to-run API requests. Open it in VS Code and use the REST client extension to send requests directly to the local server.

## Notes

- The data store is intentionally in-memory and resets when the server restarts.
- A newly created todo is assigned an incremental numeric ID.
- If a requested todo does not exist, the API responds with `404 Not Found`.

## License

This project is intended for learning and demonstration purposes.
