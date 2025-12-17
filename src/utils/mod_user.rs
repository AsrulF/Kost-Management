// User database

#[derive(Debug, PartialEq)]
pub struct Users {
    list: Vec<User>,
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

    pub fn user_login(
        &self,
        username: String,
        password: String,
    ) -> Result<&User, LoginError > {
        if self.list.is_empty() {
            println!("Users is empty, please make a new user")
        }

        let user = self
            .list
            .iter()
            .find(|user| user.username == username)
            .ok_or(LoginError::UserNotFound)?;

        if user.password != password {
            return Err(LoginError::InvalidPassword);
        }

        if user.user_role != Role::Admin {
            return Err(LoginError::UserIsNotAdmin);
        }

        Ok(&user)
    }
}

#[derive(Debug, PartialEq)]
pub struct User {
    username: String,
    password: String,
    user_role: Role,
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
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Role {
    Admin,
    NotAdmin,
}

#[derive(Debug, PartialEq)]
pub enum LoginError {
    UserNotFound,
    UserIsNotAdmin,
    InvalidPassword,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_user() {
        let new_user: User = User::new("admin2".to_string(), "123456".to_string());
        assert_eq!(new_user, User{ username: "admin2".to_string(), password: "123456".to_string(), user_role: Role::Admin});
    }

    #[test]
    fn new_users() {
        let expected: Users = Users {
            list: vec![
                User {
                    username: "admin1".to_string(),
                    password: "123456".to_string(),
                    user_role: Role::Admin,
                }
            ]
        };

        let new_users = Users::new();

        assert_eq!(new_users, expected);
    }

    #[test]
    fn login_success() {
        let users = Users::new();
        let expected: Result<&User, LoginError> = Ok( &User{
            username: "admin1".to_string(),
            password: "123456".to_string(),
            user_role: Role::Admin,
        });

        let login = users.user_login("admin1".to_string(), "123456".to_string());

        assert_eq!(login, expected)
    }

    #[test]
    fn login_failed() {
        let users = Users::new();
        let user_not_found = users.user_login("admin3".to_string(), "123456".to_string());
        let invalid_password = users.user_login("admin1".to_string(), "7891011".to_string());


        assert_eq!(user_not_found, Err(LoginError::UserNotFound));
        assert_eq!(invalid_password, Err(LoginError::InvalidPassword));
    }
}