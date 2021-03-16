use crate::schema::*;

pub struct Response {
    pub user_id: i32,
    pub poll_id: i32,
    pub selected: Option<i32>,
}
