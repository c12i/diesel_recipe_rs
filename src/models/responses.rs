use crate::schema::*;

#[derive(Debug, Queryable)]
pub struct Response {
    pub user_id: i32,
    pub poll_id: i32,
    pub selected: Option<i32>,
}
