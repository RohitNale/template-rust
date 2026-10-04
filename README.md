# Template Rust Projects

A collection of modern, production-ready Rust starter templates demonstrating backend REST APIs and server-rendered full-stack web applications.

Each template is self-contained with its own `Cargo.toml`, documentation, and ready-to-use HTTP testing files (`test_api.http`).

---

## 📑 Table of Contents

- [Template Projects Overview](#-template-projects-overview)
  - [1. Actix Web API (`actix_api`)](#1-actix-web-api-actix_api)
  - [2. Axum API (`axum_api`)](#2-axum-api-axum_api)
  - [3. Topcoat Full-Stack App (`topcoat_app`)](#3-topcoat-full-stack-app-topcoat_app)
- [Comparison Matrix](#-comparison-matrix)
- [Rust Installation & Setup](#-rust-installation--setup)
  - [Installation](#installation)
  - [Configure Environment Path](#configure-environment-path-if-required)
  - [Check Installed Versions](#check-installed-versions)
  - [Keeping Rust Up to Date](#keeping-rust-up-to-date)
- [Framework Comparison & Selection Guide](#-framework-comparison--selection-guide)
- [License](#-license)

---

## 📦 Template Projects Overview

### 1. Actix Web API (`actix_api`)

> **Path:** [`actix_api/`](./actix_api/) | **Detailed Docs:** [`actix_api/README.md`](./actix_api/README.md)

A lightweight REST API for managing a todo list using **Actix Web 4**. This project showcases core backend patterns including async route handlers, shared thread-safe application state (`web::Data<AppState>`), request/response JSON serialization with **Serde**, and structured error responses.

- **Stack:** Rust (2024 edition), Actix Web 4, Serde
- **Default Port:** `http://127.0.0.1:3000`
- **Storage:** In-memory `Mutex<Vec<Todo>>` with auto-incrementing ID

#### Quick Start

```bash
cd actix_api
cargo run
```

#### API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `POST` | `/todos` | Create a new todo item |
| `GET` | `/todos` | Retrieve all todo items |
| `PUT` | `/todos/{id}` | Update an existing todo by ID |
| `DELETE` | `/todos/{id}` | Remove a todo by ID |

---

### 2. Axum API (`axum_api`)

> **Path:** [`axum_api/`](./axum_api/) | **Detailed Docs:** [`axum_api/README.md`](./axum_api/README.md)

An ergonomic, high-performance async REST API built with **Axum 0.8** and **Tokio**. It demonstrates modern Axum routing conventions, atomic ID generation (`AtomicUsize`), path extractors, payload validation with Serde, and standard HTTP response codes (`201 Created`, `204 No Content`, `404 Not Found`).

- **Stack:** Rust (2024 edition), Axum 0.8, Tokio 1.x, Serde
- **Default Port:** `http://localhost:3000`
- **Storage:** In-memory `Arc<RwLock<HashMap<usize, Task>>>`

#### Quick Start

```bash
cd axum_api
cargo run
```

#### API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/` | Application status / root message |
| `GET` | `/health` | Health check endpoint (`Ok`) |
| `GET` | `/tasks` | List all tasks |
| `POST` | `/tasks` | Create a new task |
| `GET` | `/tasks/{id}` | Fetch a task by ID |
| `PATCH` | `/tasks/{id}` | Partially update a task |
| `DELETE` | `/tasks/{id}` | Delete a task by ID |

---

### 3. Topcoat Full-Stack App (`topcoat_app`)

> **Path:** [`topcoat_app/`](./topcoat_app/) | **Detailed Docs:** [`topcoat_app/README.md`](./topcoat_app/README.md)

A full-stack, server-side rendered (SSR) productivity application built with **Topcoat**, **Tailwind CSS**, and an embedded REST API. It features a polished dark-mode dashboard with statistics cards, task priority management (`low`, `medium`, `high`), status filtering, and synchronous HTML rendering without heavy client-side WASM bundles.

- **Stack:** Rust (2024 edition), Topcoat 0.10 (with Tailwind CSS), Tokio, Serde
- **Default Port:** `http://localhost:3000`
- **Storage:** In-memory `RwLock<Vec<Todo>>`

#### Quick Start

```bash
cd topcoat_app
cargo run
```

#### Routes & Endpoints

| Type | Method | Route | Description |
| :--- | :--- | :--- | :--- |
| **Web UI** | `GET` | `/` | Responsive dark dashboard with stats & task controls |
| **Web UI** | `GET` | `/todos/{id}` | Task detail view |
| **API** | `GET` | `/api/health` | Health check endpoint |
| **API** | `GET` | `/api/todos` | List all todos (JSON) |
| **API** | `GET` | `/api/todos/{id}` | Fetch single todo by ID |
| **API** | `POST` | `/api/todos` | Create a new todo |
| **API** | `PATCH` | `/api/todos/{id}` | Update title, description, or priority |
| **API** | `DELETE` | `/api/todos/{id}` | Delete a todo |

---

## 📊 Comparison Matrix

| Project | Primary Purpose | Framework | Architecture | Frontend / UI |
| :--- | :--- | :--- | :--- | :--- |
| [`actix_api`](./actix_api/) | Backend Microservice / API | Actix Web 4 | RESTful JSON API | Headless / API only |
| [`axum_api`](./axum_api/) | Backend Microservice / API | Axum 0.8 + Tokio | RESTful JSON API | Headless / API only |
| [`topcoat_app`](./topcoat_app/) | Full-Stack Web Application | Topcoat 0.10 | SSR Web App + REST API | Embedded HTML + Tailwind CSS |

---

## 🦀 Rust Installation & Setup

Follow the steps below based on your operating system to install Rust using **rustup** (the official toolchain installer) and verify its version.

### Installation

#### 🍏 macOS & 🐧 Linux
Open your terminal and run the following command to download and execute the official installation script:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

*When prompted during installation, press `1` and hit **Enter** to proceed with the default setup.*

#### 🪟 Windows
1. Download the **rustup-init.exe** installer from the official [Rust Downloads](https://rust-lang.org) page.
2. Run the `.exe` file.
3. If prompted, install the required Microsoft C++ Build Tools.
4. Press `1` and hit **Enter** to proceed with the default installation.

### Configure Environment Path (If Required)
After installation, you may need to restart your terminal window or manually source the environment variables:

- **Linux / macOS:** Run `source $HOME/.cargo/env` or restart your shell.
- **Windows:** Close your current command prompt / PowerShell window and open a new one.

### Check Installed Versions

To verify that the Rust compiler (`rustc`) and package manager (`cargo`) were successfully installed, run:

```bash
# Check Rust Compiler Version
rustc --version

# Check Cargo Version
cargo --version

# Check Rustup Version
rustup --version
```

### Keeping Rust Up to Date
To update your installation to the latest stable release:

```bash
rustup update
```

---

## 🧭 Framework Comparison & Selection Guide

Selecting the right framework for Rust web development depends on your performance requirements, architectural preferences, and target platforms:

### Pure Backend APIs
- **Axum**: The modern standard from the Tokio team. Highly composable, ergonomic, and leverages the Tower/Hyper ecosystem natively.
- **Actix Web**: A mature, battle-tested framework offering extreme throughput, comprehensive feature sets, and a robust actor-inspired architecture in v4+.

### Full-Stack Frameworks
- **Topcoat**: A newer, "batteries-included" server-first framework from the Tokio team. Prioritizes server-side rendering (SSR) and minimal JavaScript overhead over full WASM client bundles, offering quick initial page loads and a familiar Rails/Laravel-like mental model.
- **Leptos**: Ideal for high-performance reactive full-stack web applications compiled directly to WebAssembly (WASM) in the browser.
- **Dioxus**: A React-like declarative framework supporting universal deployments across web (WASM), desktop (WebView), and mobile.

---

## 📄 License

This repository is distributed under the terms of the [MIT License](./LICENSE).