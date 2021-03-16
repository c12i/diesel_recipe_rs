use clap::{clap_app, crate_authors, crate_version};
use diesel::prelude::*;
use diesel_patches::schema::*;
use diesel_patches::{new_user, User};

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
    )
    .get_matches();

    let conn = diesel_patches::create_connection()?;

    if let Some(sub) = clap.subcommand_matches("new_user") {
        let new_user = new_user(
            sub.value_of("name").unwrap(),
            sub.value_of("password").unwrap(),
        );

        let added: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&conn)?;
        println!("New user added: {:?}", added);
    }

    Ok(())
}
