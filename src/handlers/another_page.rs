use askama::Template;
use axum::{ http::StatusCode, response::{Html, IntoResponse, Response} };


struct HtmlTemplate<T>(T);

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.

#[derive(Template)]
#[template(path = "another-page.html")]
pub struct AnotherPageTemplate;

pub async fn another_page() -> impl IntoResponse {
    let template = AnotherPageTemplate {};
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