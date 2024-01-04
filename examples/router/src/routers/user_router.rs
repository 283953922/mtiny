use mtiny::response::json::Json;
use serde::{Deserialize, Serialize};

use crate::{models::user::User, services::user_service::UserService};

pub(crate) struct UserRouter {
    service: UserService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserQueryParam {
    pub user_id: String,
}

impl UserRouter {
    pub(crate) fn new() -> Self {
        Self {
            service: UserService::new(),
        }
    }
}

impl UserRouter {
    pub(crate) async fn add_user(&self, user: User) -> Json<bool> {
        let result = self.service.add_user(user);
        Json(result)
    }
    pub(crate) async fn get_user(&self, user_id: String) -> Json<User> {
        let user = self.service.get_user(user_id);
        Json(user)
    }
    pub(crate) async fn del_user(&self, user_id: String) -> Json<bool> {
        let result = self.service.dele_user(user_id);
        Json(result)
    }
}
