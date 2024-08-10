// src/db.rs

use crate::user::dao::UserDao;
use std::env;

pub fn setup_database() -> Result<UserDao, Box<dyn std::error::Error>> {
    // 讀取 DATABASE_URL 環境變數
    let db_url = env::var("DATABASE_URL")?;

    // 創建 UserDao 實例
    let dao = UserDao::new(db_url);

    // 設定資料庫
    dao.set_database()?;

    Ok(dao)
}
