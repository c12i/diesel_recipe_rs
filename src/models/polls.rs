use crate::schema::*;

#[derive(Debug, Queryable)]
pub struct Poll {
    pub id: i32,
    pub question: String,
    pub owner: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "polls"]
pub struct NewPoll<'a> {
    question: &'a str,
    options: &'a str,
    owner: Option<i32>,
}
