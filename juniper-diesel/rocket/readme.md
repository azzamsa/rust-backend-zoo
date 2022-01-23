## Setting Up Database

``` shell
$ cargo install diesel_cli --no-default-features --features postgres

$ cp .example.env .env
$ # change the username and password of the database

$ diesel setup
$ diesel migration generate create_user_table
$ diesel migration run
```
