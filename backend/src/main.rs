use axum::{
    routing::get,
    Router,
};
use tokio_postgres::{Error, NoTls};

mod handlers;

#[tokio::main]
async fn main() -> Result <(), Error> {
    let (client, connection) = 
        tokio_postgres::connect("host=localhost user=postgres, password=123", NoTls).await?;

    // A tokio background thread for the connection object
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    let app = Router::new()
        .route("/", get(handlers::root::root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}