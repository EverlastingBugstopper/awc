use awc::{AwcCompiler, AwcDiagnosticSeverity};
use axum::{
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    routing::{get, get_service, post},
    Json, Router,
};
use cansi::{Color, Intensity};
use serde_json::Value;
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("AWC_LOG").unwrap_or_else(|_| {
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info,awc=debug,salsa=off".into())
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("server")
        .join("public");

    info!("serving files from {}", assets_dir.display());
    assert!(std::fs::metadata(&assets_dir).is_ok());

    let app = Router::new()
        .route("/", get(index_html))
        .route("/", post(validate))
        .fallback(
            get_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
                .handle_error(handle_error),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect(&format!("could not bind to {}", &addr));
}

async fn validate(graphql: String) -> impl IntoResponse {
    let result = AwcCompiler::builder()
        .input(graphql)
        .fail_level(AwcDiagnosticSeverity::Error)
        .ignore_warnings(false)
        .ignore_advice(false)
        .build()
        .validate();
    let context = get_context(&result.pretty());
    let mut json = result.json();
    json["context"] = Value::from(context);
    (StatusCode::OK, Json(json))
}

async fn index_html() -> Html<&'static str> {
    Html(include_str!("./public/index.html"))
}

async fn handle_error(e: std::io::Error) -> impl IntoResponse {
    error!("{}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}

fn get_context(pretty: &str) -> String {
    let split = cansi::categorise_text(pretty);
    // let mut possible_fg_colors = Vec::new();
    // let mut possible_bg_colors = Vec::new();
    // let mut possible_intensities = Vec::new();
    let result = cansi::line_iter(&split)
        .map(|line| {
            line.iter()
                .map(|style_block| {
                    let css_style = match (
                        &style_block.fg_colour,
                        &style_block.bg_colour,
                        &style_block.intensity,
                    ) {
                        // not sure why this shows up like this but it's red i promise
                        (Color::Black, Color::Black, Intensity::Faint) => {
                            // errors
                            "text-error".to_string()
                        }
                        (Color::Black, Color::White, Intensity::Faint) => {
                            // line numbers
                            "text-secondary".to_string()
                        }
                        (Color::White, Color::Black, Intensity::Normal) | _ => {
                            "text-content".to_string()
                        } // TODO: couple more colors to handle!
                    };
                    // info!("fg_colour: {:?}", &style_block.fg_colour);
                    // if !possible_fg_colors.contains(&style_block.fg_colour) {
                    //     possible_fg_colors.push(style_block.fg_colour);
                    // }
                    // info!("bg_colour: {:?}", &style_block.bg_colour);
                    // if !possible_fg_colors.contains(&style_block.bg_colour) {
                    //     possible_bg_colors.push(style_block.bg_colour);
                    // }
                    // info!("intensity: {:?}", &style_block.intensity);
                    // if !possible_intensities.contains(&style_block.intensity) {
                    //     possible_intensities.push(style_block.intensity);
                    // }
                    // info!("-------------");
                    if !style_block.text.is_empty() {
                        format!(
                            "<code class=\"m-0 p-0 {}\">{}</code>",
                            &css_style,
                            &style_block.text.replace(" ", "&nbsp;")
                        )
                    } else {
                        "".to_string()
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("<br/>");
    result
}
