# Food Backend

## Database configuration

The database is specified via the `DATABASE_URL` env var, which can be specified in a `.env` file.
The variable is in the format `postgresql://[user[:password]@][netloc][:port]/dbname` (per [stackoverflow](https://stackoverflow.com/questions/3582552/what-is-the-format-for-the-postgresql-connection-string-url)).

Install the `sqlx` cli with `cargo install sqlx-cli`. You can now create the database and run all migrations with `sqlx database setup`.

You can create migrations with `sqlx migrate add <name>` and run all pending migrations with `sqlx migrate run`.

After changing the schema, you must run `cargo sqlx prepare` to generate information for offline compilation.

See the [sqlx repo](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) for more info.