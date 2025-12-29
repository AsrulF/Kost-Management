// User database
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Clone)]
pub struct Users {
    pub list: Vec<User>,
}

impl Users {
    pub fn new() -> Self {
        let mut users = Vec::<User>::new();
        let user = User::new("admin1".to_string(), "123456".to_string());
        users.push(user);
        
        Self {
            list: users,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub user_role: Role,
    pub user_id: u64,
}

impl User {
    pub fn new(
        username: String,
        password: String,
    ) -> Self {
        Self {
            username,
            password,
            user_role: Role::Admin,
            user_id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Role {
    Admin,
    NotAdmin,
}

#[derive(Debug, PartialEq)]
pub enum LoginError {
    UserNotFound,
    UserIsNotAdmin,
    InvalidPassword,
    TokenCreationError,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_user() {
        let new_user: User = User::new("admin2".to_string(), "123456".to_string());
        assert_eq!(new_user, User{ 
            username: "admin2".to_string(), 
            password: "123456".to_string(), 
            user_role: Role::Admin,
            user_id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        })
    }

    #[test]
    fn new_users() {
        let expected: Users = Users {
            list: vec![
                User {
                    username: "admin1".to_string(),
                    password: "123456".to_string(),
                    user_role: Role::Admin,
                    user_id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                }
            ]
        };

        let new_users = Users::new();

        assert_eq!(new_users, expected);
    }
}