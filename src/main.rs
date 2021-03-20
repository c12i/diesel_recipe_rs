use clap::{clap_app, crate_authors, crate_version};
use diesel::prelude::*;
use diesel_patches::schema::*;
use diesel_patches::{new_user, NewPoll, Poll, User};

fn main() -> Result<(), failure::Error> {
    let clap = clap_app!(diesel_patches =>
        (about: "A cli for the diesel_patches database")
        (version:crate_version!())
        (author:crate_authors!())
        (@subcommand new_user =>
            (about: "Creates a new user")
            (@arg name: -n --name +required +takes_value "The name of the new user")
            (@arg password: -p --pass +required +takes_value "The password of the user")
        )
        (@subcommand new_poll =>
            (about: "Creates a new poll")
            (@arg user_id: --user -u +takes_value "The owner of the poll")
            (@arg question: --question -q +required +takes_value "The question")
            (@arg options: --opts -o +required +takes_value "The options (comma separated)")
        )
        (@subcommand view_polls =>
            (about: "Views Polls")
        )
        (@subcommand view_users =>
            (about: "Views Users")
        )
    )
    .get_matches();

    let conn = diesel_patches::create_connection()?;

    if let Some(ref sub) = clap.subcommand_matches("new_user") {
        let new_user = new_user(
            sub.value_of("name").unwrap(),
            sub.value_of("password").unwrap(),
        );

        let added = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&conn)?;
        println!("New user added: {:?}", added);
    }

    if let Some(_) = clap.subcommand_matches("view_users") {
        let res = users::table.limit(10).load::<User>(&conn)?;

        for v in res {
            println!("{}: {:?}", v.id, v);
        }
    }

    if let Some(ref sub) = clap.subcommand_matches("new_poll") {
        let owner: Option<i32> = match sub.value_of("user_id") {
            Some(v) => Some(v.parse()?),
            None => None,
        };

        let question = sub.value_of("question").unwrap();
        let options = sub.value_of("options").unwrap();

        let new_poll = NewPoll::new(question, options, owner);

        let added = diesel::insert_into(polls::table)
            .values(&new_poll)
            .get_result::<Poll>(&conn)?;

        println!("Added poll: {:?}", added);
    }

    if let Some(_) = clap.subcommand_matches("view_polls") {
        let res = polls::table.limit(10).load::<Poll>(&conn)?;

        for v in res {
            println!("{}: {:?}", v.id, v);
        }
    }
    Ok(())
}
