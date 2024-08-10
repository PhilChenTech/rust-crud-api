mod user;

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use std::env;
use std::net::TcpListener;
use user::controller::UserController;
use user::service::UserService;
use user::dao::UserDao;

fn main() {
    dotenv().ok();

    // 讀取 PORT 和 DATABASE_URL
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let dao = UserDao::new(db_url.clone());
    if let Err(e) = dao.set_database() {
        println!("Error setting up the database: {}", e);
        return;
    }

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
