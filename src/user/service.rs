use crate::user::dao::UserDao;
use crate::user::model::User;

pub struct UserService {
    pub dao: UserDao,
}


impl UserService {
    pub fn new(dao: UserDao) -> Self {
        Self { dao }
    }

    pub fn create_user(&self, user: &User) -> Result<(), String> {
        self.dao.create_user(user).map_err(|e| e.to_string())
    }

    pub fn get_user_by_id(&self, id: i32) -> Result<User, String> {
        self.dao.get_user_by_id(id).map_err(|e| e.to_string())
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, String> {
        self.dao.get_all_users().map_err(|e| e.to_string())
    }

    pub fn update_user(&self, id: i32, user: &User) -> Result<(), String> {
        self.dao.update_user(id, user).map_err(|e| e.to_string())
    }

    pub fn delete_user(&self, id: i32) -> Result<bool, String> {
        match self.dao.delete_user(id) {
            Ok(0) => Ok(false),
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string()),
        }
    }
}
