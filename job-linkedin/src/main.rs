mod domain;
mod browser;
mod scrapers;
mod analysis;

use axum::{extract::Form, response::IntoResponse, routing::{get, post}, Router};
use askama::Template;
use serde::Deserialize;
use domain::Job;
use scrapers::ScraperStrategy;
use std::env;

#[derive(Deserialize)]
struct SearchParams {
    keywords: String,
    location: String,
    provider: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "results.html")]
struct ResultsTemplate {
    jobs: Vec<Job>,
    keywords: String,
    provider: String,
    jobs_json: String,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: String,
}

async fn index_handler() -> impl IntoResponse {
    IndexTemplate
}

async fn search_handler(Form(p): Form<SearchParams>) -> impl IntoResponse {
    let result = async {
        let scraper = ScraperStrategy::get_scraper(&p.provider).map_err(|e| e.to_string())?;
        let jobs = scraper.search(&p.keywords, &p.location).await
            .map_err(|e| format!("{}", e))?;
        Ok::<Vec<Job>, String>(jobs)
    }.await;

    match result {
        Ok(jobs) => {
            let jobs_json = serde_json::to_string(&jobs).unwrap_or_else(|_| "[]".to_string());
            ResultsTemplate { jobs, keywords: p.keywords, provider: p.provider.to_uppercase(), jobs_json }.into_response()
        },
        Err(msg) => ErrorTemplate { message: msg }.into_response()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/search", post(search_handler));

    // MODIFICACIÓN PARA RENDER: Leer puerto dinámico
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("✅ Servidor v0.4.0 iniciado en {}", addr);
    axum::serve(listener, app).await.unwrap();
}
