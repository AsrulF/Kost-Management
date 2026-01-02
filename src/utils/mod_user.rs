// User database
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    pub fn add_user(&mut self, new_user: User) -> Result<(), UserError> {
        let new_username = new_user.username.to_ascii_lowercase();

        if self.list.iter().any(|user| user.username.to_ascii_lowercase() == new_username) {
            return Err(UserError::UsernameAlreadyExist);
        }

        self.list.push(new_user);
        Ok(())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub user_role: Role,
    pub user_id: Uuid,
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
            user_id: Uuid::new_v4()
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
    TokenCreationError,
}

pub enum UserError {
    UsernameAlreadyExist
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
            user_id: new_user.user_id,
        })
    }

    #[test]
    fn new_users() {
        let new_users = Users::new();
        let expected: Users = Users {
            list: vec![
                User {
                    username: "admin1".to_string(),
                    password: "123456".to_string(),
                    user_role: Role::Admin,
                    user_id: new_users.list[0].user_id,
                }
            ]
        };


        assert_eq!(new_users, expected);
    }
}