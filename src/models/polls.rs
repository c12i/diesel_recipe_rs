use crate::schema::*;

#[derive(Queryable, Debug)]
pub struct Poll {
    pub id: i32,
    pub question: String,
    pub options: String,
    pub owner: Option<i32>,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "polls"]
pub struct NewPoll<'a> {
    pub question: &'a str,
    pub options: &'a str,
    pub owner: Option<i32>,
}

impl<'a> NewPoll<'a> {
    pub fn new(question: &'a str, options: &'a str, owner: Option<i32>) -> NewPoll<'a> {
        NewPoll {
            question,
            options,
            owner,
        }
    }
}
