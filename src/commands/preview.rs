use crate::db::init_db;
use crate::stats::StatsCalculator;
use crate::ui::AsciiHeatmap;
use anyhow::Result;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use crossterm::style::Stylize;
use include_dir::{include_dir, Dir};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

// Embed the web dist directory into the binary
static WEB_DIST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/web/dist");

pub async fn run(web: bool, port: u16) -> Result<()> {
    println!("{}", "📊 KitMap - Keyboard Statistics".cyan().bold());
    println!("{}", "━".repeat(40).dark_grey());
    println!();

    let db = init_db()?;
    let calculator = StatsCalculator::new(db);
    let stats = calculator.calculate_all()?;

    if stats.total_keys == 0 {
        println!("{}", "No keyboard data recorded yet!".yellow());
        println!("Run {} to start recording.", "kitmap listen".cyan());
        return Ok(());
    }

    if web {
        // Start web server
        println!(
            "{} Starting web server on port {}...",
            "→".dark_grey(),
            port
        );
        println!();
        println!(
            "{} Open {} in your browser",
            "✓".green(),
            format!("http://localhost:{}", port).cyan().underlined()
        );
        println!("{}", "Press Ctrl+C to stop the server.".dark_grey());

        let app_state = Arc::new(stats);

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = Router::new()
            .route("/", get(serve_index))
            .route("/api/stats", get(get_stats))
            .route("/assets/*path", get(serve_static))
            .layer(cors)
            .with_state(app_state);

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

        // Open browser automatically
        #[cfg(target_os = "macos")]
        let _ = std::process::Command::new("open")
            .arg(format!("http://localhost:{}", port))
            .spawn();

        #[cfg(target_os = "linux")]
        let _ = std::process::Command::new("xdg-open")
            .arg(format!("http://localhost:{}", port))
            .spawn();

        #[cfg(target_os = "windows")]
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", &format!("http://localhost:{}", port)])
            .spawn();

        axum::serve(listener, app).await?;
    } else {
        // ASCII heatmap mode
        let heatmap = AsciiHeatmap::new(&stats);

        println!("{}", heatmap.render());
        println!("{}", heatmap.render_stats(&stats));

        println!();
        println!(
            "{}",
            "Tip: Run `kitmap preview --web` for detailed web visualization.".dark_grey()
        );
    }

    Ok(())
}

async fn serve_index() -> impl IntoResponse {
    match WEB_DIST.get_file("index.html") {
        Some(file) => Html(file.contents_utf8().unwrap_or("")).into_response(),
        None => (StatusCode::NOT_FOUND, "Index not found").into_response(),
    }
}

async fn get_stats(
    State(stats): State<Arc<crate::stats::calculator::AllStats>>,
) -> Json<crate::stats::calculator::AllStats> {
    Json((*stats).clone())
}

async fn serve_static(axum::extract::Path(path): axum::extract::Path<String>) -> Response {
    let content_type = if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else {
        "application/octet-stream"
    };

    // Try to get file from embedded assets
    let asset_path = format!("assets/{}", path);

    match WEB_DIST.get_file(&asset_path) {
        Some(file) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, content_type)],
            file.contents().to_vec(),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}
