use postgres::{Client, NoTls, Error as PostgresError};
use crate::model::User;


pub struct UserDao {
    pub db_url: String,
}

impl UserDao {
    pub fn new(db_url: String) -> Self {
        Self { db_url }
    }

    pub fn set_database(&self) -> Result<(), PostgresError> {
        let mut client = Client::connect(&self.db_url, NoTls)?;
        client.batch_execute(
            "CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                email VARCHAR NOT NULL
            )"
        )?;
        Ok(())
    }

    pub fn create_user(&self, user: &User) -> Result<(), PostgresError> {
        let mut client = Client::connect(&self.db_url, NoTls)?;
        client.execute(
            "INSERT INTO users (name, email) VALUES ($1, $2)",
            &[&user.name, &user.email]
        )?;
        Ok(())
    }

    pub fn get_user_by_id(&self, id: i32) -> Result<User, PostgresError> {
        let mut client = Client::connect(&self.db_url, NoTls)?;
        let row = client.query_one("SELECT * FROM users WHERE id = $1", &[&id])?;
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        })
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, PostgresError> {
        let mut client = Client::connect(&self.db_url, NoTls)?;
        let mut users = Vec::new();
        for row in client.query("SELECT * FROM users", &[])? {
            users.push(User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
            });
        }
        Ok(users)
    }

    pub fn update_user(&self, id: i32, user: &User) -> Result<(), PostgresError> {
        let mut client = Client::connect(&self.db_url, NoTls)?;
        client.execute(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            &[&user.name, &user.email, &id]
        )?;
        Ok(())
    }

    pub fn delete_user(&self, id: i32) -> Result<u64, PostgresError> {
        let mut client = Client::connect(&self.db_url, NoTls)?;
        let result = client.execute("DELETE FROM users WHERE id = $1", &[&id])?;
        Ok(result)
    }
}
