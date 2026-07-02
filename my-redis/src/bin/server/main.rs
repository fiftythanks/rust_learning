mod db;
mod process;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::process::process;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        println!("Accepted!");

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
