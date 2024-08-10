mod user;
mod db;
mod app;  // 新增此行以導入 app 模組

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use app::initialize_app;  // 導入 initialize_app 函數

fn main() {
    dotenv().ok();

    // 初始化應用
    let (controller, listener) = match initialize_app() {
        Ok((controller, listener)) => (controller, listener),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("Server started at {}", listener.local_addr().unwrap());

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
