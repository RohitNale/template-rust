use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, RwLock};

use serde::{Deserialize, Serialize};
use topcoat::router::error::RouterErrorExt;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{
        Router, RouterBuilderDiscoverExt, Slot, StatusCode, content::Json, layout, page,
        path_param, route,
    },
    view::{View, component, view},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub priority: String, // "low", "medium", "high"
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
    #[serde(default)]
    pub priority: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PatchTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<String>,
}

static NEXT_ID: AtomicU32 = AtomicU32::new(4);

static TODOS: LazyLock<RwLock<Vec<Todo>>> = LazyLock::new(|| {
    RwLock::new(vec![
        Todo {
            id: 1,
            title: "Build Rust web app with Topcoat".to_string(),
            description: "Explore server-side rendering, component views, and Tailwind v4 integration.".to_string(),
            completed: true,
            priority: "high".to_string(),
        },
        Todo {
            id: 2,
            title: "Design sleek Tailwind UI".to_string(),
            description: "Craft a modern dark-mode interface with badges, filters, and smooth micro-interactions.".to_string(),
            completed: false,
            priority: "medium".to_string(),
        },
        Todo {
            id: 3,
            title: "Test CRUD REST APIs".to_string(),
            description: "Verify GET, POST, PATCH, and DELETE endpoints with automated checks.".to_string(),
            completed: false,
            priority: "low".to_string(),
        },
    ])
});

fn all_todos() -> Vec<Todo> {
    TODOS.read().unwrap().clone()
}

fn find_todo(id: u32) -> Option<Todo> {
    TODOS.read().unwrap().iter().find(|t| t.id == id).cloned()
}

fn create_todo(input: CreateTodo) -> Todo {
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let todo = Todo {
        id,
        title: input.title,
        description: input.description.unwrap_or_default(),
        completed: input.completed.unwrap_or(false),
        priority: input
            .priority
            .map(|p| p.to_lowercase())
            .unwrap_or_else(|| "medium".to_string()),
    };
    TODOS.write().unwrap().push(todo.clone());
    todo
}

fn patch_todo(id: u32, patch: PatchTodo) -> Option<Todo> {
    let mut todos = TODOS.write().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if let Some(title) = patch.title {
            todo.title = title;
        }
        if let Some(desc) = patch.description {
            todo.description = desc;
        }
        if let Some(completed) = patch.completed {
            todo.completed = completed;
        }
        if let Some(priority) = patch.priority {
            todo.priority = priority.to_lowercase();
        }
        Some(todo.clone())
    } else {
        None
    }
}

fn delete_todo(id: u32) -> Option<Todo> {
    let mut todos = TODOS.write().unwrap();
    if let Some(pos) = todos.iter().position(|t| t.id == id) {
        Some(todos.remove(pos))
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    topcoat::start(
        Router::builder()
            .discover()
            .assets(AssetBundle::load().unwrap())
            .build(),
    )
    .await
    .unwrap();
}

// -------------------------------------------------------------
// REST API Endpoints
// -------------------------------------------------------------

#[route(GET "/api/health")]
async fn health() -> Result<&'static str> {
    Ok("ok")
}

/// GET all todos: GET /api/todos
#[route(GET "/api/todos")]
async fn api_get_todos() -> Result<Json<Vec<Todo>>> {
    Ok(Json(all_todos()))
}

path_param!(todo_id: u32, error=not_found);

/// GET one todo by id: GET /api/todos/{todo_id}
#[route(GET "/api/todos/{todo_id}")]
async fn api_get_todo(cx: &Cx) -> Result<Json<Todo>> {
    let id = path_param::<TodoId>(cx)?;
    let todo = find_todo(*id).ok_or_not_found()?;
    Ok(Json(todo))
}

/// CREATE a new todo: POST /api/todos
#[route(POST "/api/todos")]
async fn api_create_todo(Json(input): Json<CreateTodo>) -> Result<(StatusCode, Json<Todo>)> {
    let todo = create_todo(input);
    Ok((StatusCode::CREATED, Json(todo)))
}

/// PATCH an existing todo: PATCH /api/todos/{todo_id}
#[route(PATCH "/api/todos/{todo_id}")]
async fn api_patch_todo(cx: &Cx, Json(patch): Json<PatchTodo>) -> Result<Json<Todo>> {
    let id = path_param::<TodoId>(cx)?;
    let updated = patch_todo(*id, patch).ok_or_not_found()?;
    Ok(Json(updated))
}

/// DELETE a todo by id: DELETE /api/todos/{todo_id}
#[route(DELETE "/api/todos/{todo_id}")]
async fn api_delete_todo(cx: &Cx) -> Result<Json<Todo>> {
    let id = path_param::<TodoId>(cx)?;
    let deleted = delete_todo(*id).ok_or_not_found()?;
    Ok(Json(deleted))
}

// -------------------------------------------------------------
// UI Helpers & Styling
// -------------------------------------------------------------

// ponytail: static class mapping instead of dynamic badge builder. Upgrade path: theme token system if dynamic theme switching needed.
fn priority_style(priority: &str) -> &'static str {
    match priority {
        "high" => "bg-rose-950 text-rose-300 border-rose-800",
        "low" => "bg-emerald-950 text-emerald-300 border-emerald-800",
        _ => "bg-amber-950 text-amber-300 border-amber-800",
    }
}

// -------------------------------------------------------------
// UI Layout & Pages
// -------------------------------------------------------------

#[layout("/")]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en" class="h-full bg-slate-950 text-slate-100">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"TaskFlow - Modern Todo App"</title>
                // ponytail: uses native system font-sans stack instead of external Google Fonts CDN. Upgrade path: add webfonts if custom branding is required.
                <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!()) />
                topcoat::dev::script()
            </head>
            <body class="min-h-full flex flex-col font-sans antialiased">
                <header class="border-b border-slate-800 bg-slate-900/90 sticky top-0 z-10">
                    <div class="max-w-4xl mx-auto px-4 h-14 flex items-center justify-between">
                        <div class="flex items-center gap-2.5">
                            <span class="h-7 w-7 rounded-lg bg-indigo-600 flex items-center justify-center text-white font-bold text-sm">
                                "✓"
                            </span>
                            <a href="/" class="font-bold text-white tracking-tight">
                                "TaskFlow"
                            </a>
                            <span class="text-xs text-indigo-400 bg-indigo-950/60 border border-indigo-800/60 px-2 py-0.5 rounded-full">
                                "Topcoat + Rust"
                            </span>
                        </div>
                        <nav class="flex items-center gap-3 text-xs font-medium">
                            <a href="/" class="text-slate-300 hover:text-white">"Tasks"</a>
                            <a
                                href="/api/todos"
                                target="_blank"
                                class="text-slate-400 hover:text-slate-200 px-2 py-1 rounded bg-slate-800 border border-slate-700"
                            >
                                "API (JSON)"
                            </a>
                        </nav>
                    </div>
                </header>

                <main class="flex-1 max-w-4xl w-full mx-auto px-4 py-6">
                    (slot)
                </main>

                <footer class="border-t border-slate-900 py-6 text-center text-xs text-slate-500">
                    <p>"Built with Topcoat, Rust & Tailwind CSS • Full REST API"</p>
                </footer>
            </body>
        </html>
    })
}

#[page("/")]
async fn home() -> Result<impl View> {
    let list = all_todos();
    let total = list.len();
    let completed = list.iter().filter(|t| t.completed).count();
    let pending = total.saturating_sub(completed);

    Ok(view! {
        <div class="space-y-6">
            // Stats row
            <div class="grid grid-cols-3 gap-3">
                <div class="p-4 rounded-xl bg-slate-900 border border-slate-800">
                    <div class="text-xs text-slate-400">"Total"</div>
                    <div class="text-2xl font-bold text-white mt-1">(total)</div>
                </div>
                <div class="p-4 rounded-xl bg-slate-900 border border-slate-800">
                    <div class="text-xs text-emerald-400">"Completed"</div>
                    <div class="text-2xl font-bold text-emerald-400 mt-1">(completed)</div>
                </div>
                <div class="p-4 rounded-xl bg-slate-900 border border-slate-800">
                    <div class="text-xs text-amber-400">"Pending"</div>
                    <div class="text-2xl font-bold text-amber-400 mt-1">(pending)</div>
                </div>
            </div>

            // Create Task Form
            <div class="p-5 rounded-xl bg-slate-900 border border-slate-800">
                <h2 class="text-sm font-semibold text-white mb-3">"Add New Task"</h2>
                <form id="new-todo-form" class="space-y-3">
                    <div class="grid grid-cols-1 sm:grid-cols-4 gap-3">
                        <div class="sm:col-span-3">
                            <label class="block text-xs text-slate-400 mb-1">"Title *"</label>
                            <input
                                type="text"
                                id="todo-title"
                                required=""
                                placeholder="What needs to be accomplished?"
                                class="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-sm text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500"
                            />
                        </div>
                        <div>
                            <label class="block text-xs text-slate-400 mb-1">"Priority"</label>
                            <select
                                id="todo-priority"
                                class="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-sm text-white focus:outline-none focus:border-indigo-500"
                            >
                                <option value="medium">"Medium"</option>
                                <option value="high">"High"</option>
                                <option value="low">"Low"</option>
                            </select>
                        </div>
                    </div>
                    <div>
                        <label class="block text-xs text-slate-400 mb-1">"Description (optional)"</label>
                        <input
                            type="text"
                            id="todo-desc"
                            placeholder="Add details..."
                            class="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-sm text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500"
                        />
                    </div>
                    <div class="flex justify-end pt-1">
                        <button
                            type="submit"
                            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-medium rounded-lg cursor-pointer transition-colors"
                        >
                            "+ Create Task"
                        </button>
                    </div>
                </form>
            </div>

            // Task List
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <h2 class="text-sm font-semibold text-white">"Tasks"</h2>
                    <span class="text-xs text-slate-500 font-mono">(format!("{total} total"))</span>
                </div>

                if list.is_empty() {
                    <div class="p-8 text-center rounded-xl bg-slate-900 border border-dashed border-slate-800 text-slate-400 text-sm">
                        "No tasks yet! Add your first task above."
                    </div>
                } else {
                    <div class="space-y-2">
                        for item in list {
                            todo_card(
                                id: item.id,
                                title: item.title.as_str(),
                                description: item.description.as_str(),
                                completed: item.completed,
                                priority: item.priority.as_str()
                            )
                        }
                    </div>
                }
            </div>

            // ponytail: simple API badge list instead of 90 lines of bespoke endpoint cards. Upgrade path: swagger-ui if interactive sandbox needed.
            <div class="p-4 rounded-xl bg-slate-900/60 border border-slate-800 text-xs">
                <div class="font-medium text-slate-400 mb-2">"API Endpoints"</div>
                <div class="flex flex-wrap gap-2 font-mono">
                    <span class="px-2 py-1 bg-slate-800 text-emerald-400 rounded">"GET /api/todos"</span>
                    <span class="px-2 py-1 bg-slate-800 text-emerald-400 rounded">"GET /api/todos/{id}"</span>
                    <span class="px-2 py-1 bg-slate-800 text-sky-400 rounded">"POST /api/todos"</span>
                    <span class="px-2 py-1 bg-slate-800 text-amber-400 rounded">"PATCH /api/todos/{id}"</span>
                    <span class="px-2 py-1 bg-slate-800 text-rose-400 rounded">"DELETE /api/todos/{id}"</span>
                    <span class="px-2 py-1 bg-slate-800 text-emerald-400 rounded">"GET /api/health"</span>
                </div>
            </div>
        </div>

        <script>
            "document.addEventListener('DOMContentLoaded', function() {
                var form = document.getElementById('new-todo-form');
                if (form) {
                    form.addEventListener('submit', function(e) {
                        e.preventDefault();
                        var title = document.getElementById('todo-title').value.trim();
                        var description = document.getElementById('todo-desc').value.trim();
                        var priority = document.getElementById('todo-priority').value;
                        if (!title) return;

                        fetch('/api/todos', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ title: title, description: description, priority: priority, completed: false })
                        }).then(function(res) {
                            if (res.ok) {
                                window.location.reload();
                            } else {
                                alert('Failed to create task');
                            }
                        }).catch(function(err) {
                            console.error(err);
                            alert('Network error');
                        });
                    });
                }

                window.toggleTodo = function(id, currentStatus) {
                    fetch('/api/todos/' + id, {
                        method: 'PATCH',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ completed: !currentStatus })
                    }).then(function(res) {
                        if (res.ok) window.location.reload();
                    }).catch(function(err) {
                        console.error(err);
                    });
                };

                window.deleteTodo = function(id) {
                    if (!confirm('Are you sure you want to delete this task?')) return;
                    fetch('/api/todos/' + id, {
                        method: 'DELETE'
                    }).then(function(res) {
                        if (res.ok) window.location.reload();
                    }).catch(function(err) {
                        console.error(err);
                    });
                };
            });"
        </script>
    })
}

#[page("/todos/{todo_id}")]
async fn todo_details(cx: &Cx) -> Result<impl View> {
    let id = path_param::<TodoId>(cx)?;
    let item = find_todo(*id).ok_or_not_found()?;

    Ok(view! {
        <div class="max-w-xl mx-auto space-y-4">
            <a
                href="/"
                class="inline-block text-xs text-indigo-400 hover:text-indigo-300 transition-colors"
            >
                "← Back to tasks"
            </a>

            <div class="p-6 rounded-xl bg-slate-900 border border-slate-800 space-y-5">
                <div class="flex items-start justify-between gap-3">
                    <div>
                        <span class="text-xs font-mono text-slate-500">
                            (format!("#{}", item.id))
                        </span>
                        <h1 class="text-xl font-bold text-white mt-0.5">
                            (item.title.as_str())
                        </h1>
                    </div>
                    <span
                        class=(format!(
                            "text-xs px-2 py-0.5 rounded border uppercase tracking-wider font-semibold {}",
                            priority_style(item.priority.as_str())
                        ))
                    >
                        (item.priority.as_str())
                    </span>
                </div>

                <div class="border-t border-slate-800 pt-3">
                    <div class="text-xs font-medium text-slate-400 mb-1">"Description"</div>
                    if item.description.is_empty() {
                        <p class="text-slate-500 italic text-sm">"No description provided."</p>
                    } else {
                        <p class="text-slate-200 text-sm whitespace-pre-wrap">
                            (item.description.as_str())
                        </p>
                    }
                </div>

                <div class="border-t border-slate-800 pt-3 flex items-center justify-between">
                    <div>
                        <span class="text-xs text-slate-400 block mb-0.5">"Status"</span>
                        if item.completed {
                            <span class="text-xs font-medium text-emerald-400">"✓ Completed"</span>
                        } else {
                            <span class="text-xs font-medium text-amber-400">"○ Pending"</span>
                        }
                    </div>

                    <div class="flex items-center gap-2">
                        <button
                            onclick=(format!(
                                "window.toggleDetail({}, {})",
                                item.id,
                                item.completed,
                            ))
                            class="px-3 py-1.5 rounded-lg text-xs font-medium bg-indigo-600 hover:bg-indigo-500 text-white cursor-pointer transition-colors"
                        >
                            if item.completed {
                                "Mark Pending"
                            } else {
                                "Mark Completed"
                            }
                        </button>
                        <button
                            onclick=(format!("window.deleteDetail({})", item.id))
                            class="px-3 py-1.5 rounded-lg text-xs font-medium bg-rose-950 text-rose-300 border border-rose-800 hover:bg-rose-900 cursor-pointer transition-colors"
                        >
                            "Delete"
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <script>
            "window.toggleDetail = function(id, currentStatus) {
                fetch('/api/todos/' + id, {
                    method: 'PATCH',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ completed: !currentStatus })
                }).then(function(res) {
                    if (res.ok) window.location.reload();
                }).catch(function(e) { console.error(e); });
            };
            window.deleteDetail = function(id) {
                if (!confirm('Are you sure you want to delete this task?')) return;
                fetch('/api/todos/' + id, { method: 'DELETE' }).then(function(res) {
                    if (res.ok) window.location.href = '/';
                }).catch(function(e) { console.error(e); });
            };"
        </script>
    })
}

#[component]
async fn todo_card(
    id: u32,
    title: &str,
    description: &str,
    completed: bool,
    priority: &str,
) -> Result<impl View> {
    Ok(view! {
        <article class="p-3.5 rounded-xl bg-slate-900 border border-slate-800 flex items-center justify-between gap-3">
            <div class="flex items-center gap-3 flex-1 min-w-0">
                <button
                    onclick=(format!("window.toggleTodo({}, {})", id, completed))
                    class=(format!(
                        "h-5 w-5 rounded border flex items-center justify-center text-xs transition-colors cursor-pointer {}",
                        if completed {
                            "bg-emerald-600 border-emerald-500 text-white"
                        } else {
                            "border-slate-700 hover:border-slate-500 text-transparent"
                        },
                    ))
                    title="Toggle task completion"
                >
                    "✓"
                </button>
                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2 truncate">
                        <a
                            href=(format!("/todos/{id}"))
                            class=(format!(
                                "text-sm font-medium hover:text-indigo-400 truncate {}",
                                if completed { "text-slate-400 line-through" } else { "text-slate-100" }
                            ))
                        >
                            (title)
                        </a>
                        <span
                            class=(format!(
                                "text-[10px] px-1.5 py-0.5 rounded border uppercase tracking-wider font-semibold {}",
                                priority_style(priority)
                            ))
                        >
                            (priority)
                        </span>
                    </div>
                    if !description.is_empty() {
                        <p class="text-xs text-slate-400 truncate mt-0.5">(description)</p>
                    }
                </div>
            </div>

            <div class="flex items-center gap-1 shrink-0">
                <a
                    href=(format!("/todos/{id}"))
                    class="text-xs text-slate-400 hover:text-white px-2 py-1 rounded hover:bg-slate-800 transition-colors"
                >
                    "View"
                </a>
                <button
                    onclick=(format!("window.deleteTodo({})", id))
                    class="text-xs text-slate-400 hover:text-rose-400 px-2 py-1 rounded hover:bg-slate-800 transition-colors cursor-pointer"
                >
                    "Delete"
                </button>
            </div>
        </article>
    })
}
