#![allow(unused)]

use axum::response::Html;
use axum::{
    body::{Bytes, Full},
    extract::Query,
    http::StatusCode,
    response::Response,
    response::{IntoResponse, Json as JsonResponse},
    routing::{get, post},
    Error, Json, Router,
};
use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::SqliteDatabase;
use bdk::electrum_client::Client;
use bdk::{SyncOptions, Wallet};
use dotenv::from_filename;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::path::Path;

#[derive(Deserialize)]
struct User {
    name: Option<String>,
}

async fn test() -> impl IntoResponse {
    Html("<h1>Hello, this is your Axum web server!</h1>")
}

async fn response() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("x-foo", "custom header")
        .body(Full::from("not found"))
        .unwrap()
}

async fn get_balance() -> Html<String> {
    // Retrieve wallet balance logic here
    from_filename(".env").ok();
    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();

    let my_path: &Path = Path::new("wallet.db");
    let wallet = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        SqliteDatabase::new(my_path),
    );

    let balance = wallet.expect("REASON").get_balance().unwrap();
    Html(format!(
        "<h1 style=\"color: green; font-weight: bold;\">Wallet Balance: {:?}</h1>",
        balance.confirmed
    ))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/balance", get(get_balance))
        .route("/test", get(test))
        .route("/", get(response));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
