mod user;
mod db;

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use std::env;
use std::net::TcpListener;
use user::controller::UserController;
use user::service::UserService;
use db::setup_database; // 導入 db 模組

fn main() {
    dotenv().ok();

    // 讀取 PORT 環境變數
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // 設定資料庫
    let dao = match setup_database() {
        Ok(dao) => dao,
        Err(e) => {
            println!("Error setting up the database: {}", e);
            return;
        }
    };

    let service = UserService::new(dao);
    let controller = UserController::new(service);

    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address).unwrap();
    println!("Server started at {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                controller.handle_request(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
