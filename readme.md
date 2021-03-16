# working with databases in Rust

What is covered:
1. Using Diesel ORM
2. Building command-line tools with clap

## Migrations

Generate

```shell
diesel migration generate <name>
```

Run

```shell
diesel migration run
```

## CLI

`--help | -h`
```shell
diesel_patches 0.1.0
Collins Muriuki <murerwacollins@gmail.com>
A cli for the diesel_patches database

USAGE:
    diesel_database_recipes [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    new_user    Creates a new user
```

`new_user`
```shell
diesel_database_recipes-new_user 
Creates a new user

USAGE:
    diesel_database_recipes new_user --name <name> --pass <password>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>        The name of the new user
    -p, --pass <password>    The password of the user
```
