// User database

pub struct Users {
    list: Vec<User>,
}

impl Users {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
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

#[derive(PartialEq)]
pub enum Role {
    Admin,
    NotAdmin,
}

#[derive(Debug)]
pub enum LoginError {
    UserNotFound,
    UserIsNotAdmin,
    InvalidPassword,
}