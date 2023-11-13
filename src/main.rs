// test❯ curl http://127.0.0.1:3000/\?name\=foo

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
use bdk::bitcoin::{Address, Network};
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::MemoryDatabase;
use bdk::database::SqliteDatabase;
use bdk::electrum_client::Client;
use bdk::{descriptor, wallet};
use bdk::{SyncOptions, Wallet};
use dotenv::from_filename;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::env;
use std::net::SocketAddr;
use std::path::Path;

//#[derive(serde::Serialize)]
//struct AddressResponse {
//address: String,
//index: u32,
//}

// 1
async fn test() -> impl IntoResponse {
    Html("<h1>Hello, this is your Axum web server!</h1>")
}

// 2
async fn response() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("x-foo", "custom header")
        .body(Full::from("not found"))
        .unwrap()
}

// 3
#[derive(Deserialize)]
struct User {
    name: Option<String>,
}


async fn user(user: Query<User>) -> Html<String> {
    match &user.name {
        Some(username) => Html(format!("Hello, {}!", username)),
        None => Html("Hello, World!".to_string()),
    }
}

#[tokio::main]
async fn main() {
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

    let balance = &wallet.expect("REASON").get_balance().unwrap();
    println!("{:?}", balance);
    //let address = &wallet
    //.get_address(wallet::AddressIndex::New);

    // AXUM STUFF ----------------------------------------------------------------

    let app = Router::new()
        .route("/:query", get(user))
        .route("/test", get(test))
        .route("/", get(response));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
