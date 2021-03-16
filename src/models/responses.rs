use crate::schema::*;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "responses"]
pub struct Response {
    pub user_id: i32,
    pub poll_id: i32,
    pub selected: Option<i32>,
}
