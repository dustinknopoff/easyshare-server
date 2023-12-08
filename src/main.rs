use std::{net::{IpAddr, Ipv6Addr, SocketAddr}, str::FromStr, sync::Arc};

use clap::Parser;
use axum::{extract::MatchedPath, http::Request, routing::*, Router, Extension};
use dotenvy::dotenv;
use object_store::aws::AmazonS3Builder;
use tokio::net::TcpListener;
use tower_http::{trace::TraceLayer, services::{ServeDir, ServeFile},};
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use anyhow::Context;

pub mod routes;
pub mod ui;
pub mod error;

// Setup the command line interface with clap.
#[derive(Parser, Debug, Clone)]
#[clap(name = "easyshare", about = "TODO")]
struct Opt {
    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./public")]
    static_dir: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "easyshare=debug,tower_http=debug,axum::rejection=trace".into()
            }))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let opt = Opt::parse();
    dotenv()?;

    let serve_dir = ServeDir::new(opt.static_dir).not_found_service(ServeFile::new("public/index.html"));

    let client = AmazonS3Builder::new()
    .with_region("auto")
        .with_access_key_id(dotenvy::var("ACCESS_KEY_ID").context("Missing account id env var")?)
        .with_secret_access_key(
            dotenvy::var("SECRET_ACCESS_KEY").context("Missing secret access env var")?,
        )
        .with_endpoint(format!(
            "https://{}.r2.cloudflarestorage.com",
            dotenvy::var("ACCOUNT_ID").context("Missing account id env var")?
        ))
        .with_bucket_name("easyshare")
        .build()?;

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    let app = Router::new()
    .route("/", get(routes::home::handler))
    .route("/upload", post(routes::upload::upload))
    .route("/share/:id", get(routes::share::list_files))
    .route("/obj/:key/:file_name", get(routes::object::get_object))
    .fallback_service(serve_dir)
    .with_state(sock_addr.to_string())
    .layer(Extension(Arc::new(client)))
    .layer(
        TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        }),
    );


    let listener = TcpListener::bind(sock_addr).await?;
    tracing::debug!("listening on {}", sock_addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
