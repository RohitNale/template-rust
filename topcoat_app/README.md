# TaskFlow

A modern Rust todo application built with Topcoat, Tailwind CSS, and a lightweight JSON API. This project demonstrates a server-rendered web app with a clean dashboard, in-memory task management, and REST endpoints in a single Rust application.

## Overview

TaskFlow is a simple but polished productivity app for tracking tasks and priorities. It combines:

- a server-rendered UI powered by Topcoat
- Tailwind-based styling for a modern dark dashboard
- a REST API for managing todos
- in-memory persistence for quick prototyping and development

This project is ideal for learning full-stack Rust patterns, routing, component-based views, and API design with Topcoat.

## Features

- Create, view, update, and delete tasks
- Track completion status and priority levels
- Responsive dark UI with stats cards and task list
- API endpoints for health checks and CRUD operations
- No external database required for local development
- Built as a single Rust service with embedded UI and backend routes

## Tech Stack

- Rust
- Topcoat
- Tokio
- Serde
- Tailwind CSS

## Project Structure

```text
topcoat_app/
├── Cargo.toml
├── build.rs
├── README.md
├── test_api.http
├── src/
│   └── main.rs
└── target/
```

## Getting Started

### Prerequisites

Make sure you have Rust and Cargo installed:

```bash
rustc --version
cargo --version
```

If Rust is not installed yet, follow the official guide:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Run the app

From the project directory:

```bash
cargo run
```

Then open the local address printed by the server in your terminal.

## Main Routes

### Web UI

- `/` — task dashboard
- `/todos/{id}` — individual task detail page

### API

- `GET /api/health` — health check
- `GET /api/todos` — list all todos
- `GET /api/todos/{id}` — fetch one todo
- `POST /api/todos` — create a new todo
- `PATCH /api/todos/{id}` — update a todo
- `DELETE /api/todos/{id}` — delete a todo

## Example API Calls

### Health check

```bash
curl http://localhost:PORT/api/health
```

### Get all tasks

```bash
curl http://localhost:PORT/api/todos
```

### Create a task

```bash
curl -X POST http://localhost:PORT/api/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Ship release",
    "description": "Prepare the deployment checklist",
    "priority": "high",
    "completed": false
  }'
```

### Update a task

```bash
curl -X PATCH http://localhost:PORT/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{
    "completed": true
  }'
```

## Data Model

Each todo contains:

- `id`: numeric identifier
- `title`: task title
- `description`: optional description
- `completed`: completion state
- `priority`: `low`, `medium`, or `high`

The application stores todos in an in-memory `RwLock<Vec<Todo>>`, which is reset when the process restarts.

## Build

To build the app without running it:

```bash
cargo build
```

To build for release:

```bash
cargo build --release
```

## Notes

This app is intentionally lightweight and designed for learning and experimentation. It is a strong example of how to combine a small full-stack Rust application, API routing, and UI rendering in a single project.

## License

This project is distributed under the repository's license terms.
