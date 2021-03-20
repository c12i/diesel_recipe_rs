use crate::{schema::*, Poll, User};

#[derive(Debug, Queryable, Insertable)]
#[table_name = "responses"]
pub struct Response {
    pub poll_id: i32,
    pub user_id: i32,
    pub selected: Option<i32>,
}

#[derive(Debug, Queryable)]
pub struct FullResponse {
    pub response: Response,
    pub user: User,
    pub poll: Poll,
}

impl Response {
    pub fn new(user_id: i32, poll_id: i32, selected: Option<i32>) -> Self {
        Response {
            user_id,
            poll_id,
            selected,
        }
    }
}
