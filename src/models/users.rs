use crate::schema::*;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    password: String,
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    name: &'a str,
    password: String,
}

impl<'a> NewUser<'a> {
    pub fn new(name: &'a str, password: String) -> Self {
        NewUser { name, password }
    }
}

pub fn new_user<'a>(name: &'a str, password: &str) -> NewUser<'a> {
    NewUser::new(name, bcrypt::hash(password, 7).unwrap())
}
