#[macro_use]
extern crate diesel;
pub mod schema;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
