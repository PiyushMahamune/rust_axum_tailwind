use axum_tailwind_template::{get_configuration, startup};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, fmt};

#[tokio::main]
async fn main() {
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::INFO);

    let tracing_layer = fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    let config = get_configuration().expect("Failed to read configuration.");

    let app = startup().expect("Unable to start the server.");

    let address = format!("{}:{}", config.application.host, config.application.port)
        .parse::<SocketAddr>()
        .unwrap();

    tracing::debug!("listening on {}", address);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
