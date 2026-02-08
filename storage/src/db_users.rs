use crate::MEM_USERS;
use domain-core::User;

pub fn all_users() -> Vec<User> {
    MEM_USERS.read().clone()
}
