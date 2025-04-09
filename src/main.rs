mod routes;

use routes::create_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    let app = create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Loto API rodando em http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

