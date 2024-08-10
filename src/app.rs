// src/app.rs

use std::net::TcpListener;
use crate::db::setup_database;
use crate::user::controller::UserController;
use crate::user::service::UserService;

pub fn initialize_app() -> Result<(UserController, TcpListener), String> {
    // 設定資料庫
    let dao = match setup_database() {
        Ok(dao) => dao,
        Err(e) => return Err(format!("Error setting up the database: {}", e)),
    };

    let service = UserService::new(dao);
    let controller = UserController::new(service);

    // 設置 TCP 監聽器
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address).map_err(|e| e.to_string())?;

    Ok((controller, listener))
}
