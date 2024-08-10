use crate::user::dao::UserDao;
use crate::user::model::User;

pub struct UserService {
    pub dao: UserDao,
}


impl UserService {
    pub fn new(dao: UserDao) -> Self {
        Self { dao }
    }

    pub fn create(&self, user: &User) -> Result<(), String> {
        self.dao.create_user(user).map_err(|e| e.to_string())
    }

    pub fn find_by_id(&self, id: i32) -> Result<User, String> {
        self.dao.get_user_by_id(id).map_err(|e| e.to_string())
    }

    pub fn find_all(&self) -> Result<Vec<User>, String> {
        self.dao.get_all_users().map_err(|e| e.to_string())
    }

    pub fn update(&self, id: i32, user: &User) -> Result<(), String> {
        self.dao.update_user(id, user).map_err(|e| e.to_string())
    }

    pub fn delete(&self, id: i32) -> Result<bool, String> {
        match self.dao.delete_user(id) {
            Ok(0) => Ok(false),
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string()),
        }
    }
}
