use std::io::{Read, Write};
use std::net::TcpStream;
use crate::user::model::User;
use crate::user::service::UserService;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

pub struct UserController {
    pub service: UserService,
}

impl UserController {
    pub fn new(service: UserService) -> Self {
        Self { service }
    }

    pub fn handle_request(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let mut request = String::new();

        if let Ok(size) = stream.read(&mut buffer) {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => self.handle_post_request(r),
                r if r.starts_with("GET /users/") => self.handle_get_request(r),
                r if r.starts_with("GET /users") => self.handle_get_all_request(),
                r if r.starts_with("PUT /users/") => self.handle_put_request(r),
                r if r.starts_with("DELETE /users/") => self.handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
    }

    fn handle_post_request(&self, request: &str) -> (String, String) {
        match get_user_request_body(&request) {
            Ok(user) => match self.service.create_user(&user) {
                Ok(_) => (OK_RESPONSE.to_string(), "User created".to_string()),
                Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
            },
            Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        }
    }

    fn handle_get_request(&self, request: &str) -> (String, String) {
        match get_id(&request).parse::<i32>() {
            Ok(id) => match self.service.get_user_by_id(id) {
                Ok(user) => (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap()),
                Err(_) => (NOT_FOUND.to_string(), "User not found".to_string()),
            },
            Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        }
    }

    fn handle_get_all_request(&self) -> (String, String) {
        match self.service.get_all_users() {
            Ok(users) => (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap()),
            Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        }
    }

    fn handle_put_request(&self, request: &str) -> (String, String) {
        match (get_id(&request).parse::<i32>(), get_user_request_body(&request)) {
            (Ok(id), Ok(user)) => match self.service.update_user(id, &user) {
                Ok(_) => (OK_RESPONSE.to_string(), "User updated".to_string()),
                Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
            },
            _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        }
    }

    fn handle_delete_request(&self, request: &str) -> (String, String) {
        match get_id(&request).parse::<i32>() {
            Ok(id) => match self.service.delete_user(id) {
                Ok(true) => (OK_RESPONSE.to_string(), "User deleted".to_string()),
                Ok(false) => (NOT_FOUND.to_string(), "User not found".to_string()),
                Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
            },
            Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        }
    }
}

// Helper functions
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
