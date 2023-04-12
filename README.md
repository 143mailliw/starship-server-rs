# Starship (server)
This is the server component of Starship 2023.0 and later. This version of Starship is unfinished, missing most features, and incompatible with all currently existing clients.

For the currently deployed version, see [starshipapp/starship-server](https://github.com/starshipapp/starship-server).

For 0.6 (in use Jul 25th 2020 - Apr 9th 2021) and earlier, see [starshipapp/starship-classic](https://github.com/starshipapp/starship-classic).

## Getting Started
To run the server, you must have at minimum:
- Cargo
- Rust (the lowest tested version is 1.65)
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
| Feature        | Status        | Notes                                                                           |
|----------------|---------------|---------------------------------------------------------------------------------|
| Users          | 70% complete  | No PFPs, banners, forgot password, register function (insertUser) is incomplete |
| Tokens         | 30% complete  | Basic backend only (just enough to allow for logging in)                        |
| Planets        | 80% complete  | Missing invites. See also: permissions, components, administration.             |
| Permissions    | 80% complete  | Missing reorder, GQL member list resolver                                       |
| Components     | 50% complete  | Missing ordering, folders, deletion, and creation                               |
| Pages          | 0% complete   |                                                                                 |
| Files          | 0% complete   |                                                                                 |
| Forums         | 0% complete   |                                                                                 |
| Chats          | 0% complete   |                                                                                 |
| Notifications  | 0% complete   |                                                                                 |
| Custom Emojis  | 10% complete  | Only GQL resolvers are implemented                                              |
| Administration | 0% complete   |                                                                                 |
| Attachments    | 0% complete   |                                                                                 |

## Contributing
While I welcome contributions to parts of the code base that are already started, if you plan to start working on a major feature that hasn't been started yet, please contact me first, so that we can ensure that the data is represented in a way that is compatible with the already existing data from the older versions of the server.
