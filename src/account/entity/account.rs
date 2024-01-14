use bcrypt::{BcryptError, hash};
use diesel::{Insertable, Queryable, table};

#[derive(Queryable, Insertable, Debug)]
#[table_name = "accounts"]
pub struct Account {
    #[column_name = "id"]
    pub id: Option<i32>,
    user_id: String,
    password: String,
}

table! {
    accounts (user_id) {
        id -> Integer,
        user_id -> Text,
        password -> Text,
    }
}

impl Account {
    pub fn new(user_id: &str, password: &str) -> Result<Self, BcryptError> {
        // 비밀번호를 bcrypt로 해싱
        let hashed_password = hash(password, 12)?;

        Ok(Account {
            id: None,
            user_id: user_id.to_string(),
            password: hashed_password,
        })
    }
}
