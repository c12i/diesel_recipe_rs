use clap::{clap_app, crate_authors, crate_version};
use diesel::prelude::*;
use diesel_patches::schema::*;
use diesel_patches::{new_user, NewPoll, Poll, Response, User};

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
        (@subcommand respond =>
            (about: "Respond to polls")
            (@arg user_id: --user -u +takes_value +required "The user responding")
            (@arg poll_id: --poll -p +takes_value +required "The poll")
            (@arg selected: --select -s +takes_value +required "The selected option (0..)")
        )
        (@subcommand view_polls =>
            (about: "Views Polls")
        )
        (@subcommand view_users =>
            (about: "Views Users")
        )
        (@subcommand poll_results =>
            (about: "Get poll Results")
            (@arg poll_id: --poll -p +takes_value +required "The Poll")
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

    if let Some(ref sub) = clap.subcommand_matches("respond") {
        let user_id = sub.value_of("user_id").unwrap();
        let poll_id = sub.value_of("poll_id").unwrap();
        let selected: Option<i32> = match sub.value_of("selected") {
            Some(v) => Some(v.parse()?),
            None => None,
        };

        let response = Response::new(user_id.parse()?, poll_id.parse()?, selected);

        let added = diesel::insert_into(responses::table)
            .values(&response)
            .get_result::<Response>(&conn)?;

        println!("Added Response: {:?}", added);
    }

    if let Some(ref sub) = clap.subcommand_matches("poll_results") {
        let id: i32 = sub.value_of("poll_id").unwrap().parse()?;

        let poll: Poll = polls::table.find(id).first::<Poll>(&conn)?;

        let vals: Vec<(i32, Option<i32>)> = responses::table
            .inner_join(users::table)
            .inner_join(polls::table)
            .select((users::id, responses::selected))
            .filter(polls::id.eq(id))
            .load::<(i32, Option<i32>)>(&conn)?;

        println!("Question: {}", poll.question);

        let mut scores = Vec::new();

        for _ in poll.options.split(",") {
            scores.push(0);
        }

        for v in vals {
            if let Some(n) = v.1 {
                let n = n as usize;

                if n < scores.len() {
                    scores[n] += 1;
                }
            } else {
                println!("User did not select a value");
            }
        }

        for (n, op) in poll.options.split(",").enumerate() {
            println!("  {}) {} = {}", n, op, scores[n]);
        }
    }

    Ok(())
}
