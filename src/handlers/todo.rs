use std::sync::Arc;

use askama::Template;
use axum::{extract::State, Form};
use serde::Deserialize;
use axum::{ http::StatusCode, response::{Html, IntoResponse, Response} };
use tokio::sync::Mutex;

struct HtmlTemplate<T>(T);

pub struct AppState {
    pub todos: Mutex<Vec<String>>,
}

#[derive(Template)]
#[template(path = "todo-list.html")]
pub struct TodoList {
    todos: Vec<String>,
}

#[derive(Deserialize)]
pub struct TodoRequest {
    todo: String,
}

#[axum::debug_handler]
pub async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    let mut lock = state.todos.lock().await;
    lock.push(todo.todo);
    let template = TodoList {
        todos: lock.clone(),
    };

    HtmlTemplate(template)
}

impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}