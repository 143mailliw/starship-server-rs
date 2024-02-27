# Starship (server)

## Getting Started
To run the server, you must have at minimum:
- Cargo
- Rust
- A working PostgreSQL instance

As development progresses, the following will be required in the future:
- A working mail server (for sending confirmation & reset password emails, optional)
- A working Redis instance (for communication between servers, *maybe* optional)
- A working S3-compatible object-storage server (for file storage)

Starship requires several environment variables to be set:
- `DATABASE_URL`, a PostgreSQL database URL
- `SECRET`, the secret used for signing tokens
- `PORT`, the numerical port the server will bind to
- `IP_ADDR`, the IP address the sever will bind to

These environment variables can be set in a .env file, or provided as part of the environment.

Once these are set, the server can be run using `cargo run`.

## Features
| Feature              | Status        | Notes                                                                           |
|----------------------|---------------|---------------------------------------------------------------------------------|
| Users                | 70% complete  | No PFPs, banners, forgot password, register function (insertUser) is incomplete |
| Tokens               | 30% complete  | Basic backend only (just enough to allow for logging in)                        |
| Planets              | 80% complete  | Missing invites. See also: permissions, components, administration.             |
| Permissions          | 100% complete | Functionally complete as of 2023-04-12, needs additional testing                |
| Components           | 50% complete  | Missing ordering, folders, home changing, and GQL queries                       |
| Toolbox (Components) | 0% complete   |                                                                                 |
| Toolbox (Actions)    | 0% complete   |                                                                                 |
| Toolbox (Data)       | 0% complete   |                                                                                 |
| Toolbox (API)        | 0% complete   |                                                                                 |
| Custom Emojis        | 10% complete  | Only GQL resolvers are implemented                                              |
| Administration       | 0% complete   |                                                                                 |
| Attachments          | 0% complete   |                                                                                 |