mod controller;
mod service;
mod dao;
mod model;
#[macro_use]
extern crate serde_derive;
use dotenv::dotenv;
use std::env;
use std::net::TcpListener;
use controller::UserController;
use service::UserService;
use dao::UserDao;

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let dao = UserDao::new(db_url.clone());
    if let Err(e) = dao.set_database() {
        println!("Error setting up the database: {}", e);
        return;
    }

    let service = UserService::new(dao);
    let controller = UserController::new(service);

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server started at port 8080");

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
