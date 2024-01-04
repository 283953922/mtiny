use crate::models::user::User;

pub(crate) struct UserService;

impl UserService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl UserService {
    pub(crate) fn add_user(&self, user: User) -> bool {
        println!("add user to db:{:?} ", &user);
        true
    }

    pub(crate) fn get_user(&self, id: String) -> User {
        println!("get user by id:{:?} ", &id);
        User::new("123".to_string(), "张三".to_string(), 20)
    }

    pub(crate) fn dele_user(&self, id: String) -> bool {
        println!("delete user id:{:?} ", &id);
        true
    }
}
