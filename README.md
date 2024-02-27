# Starship

This is the monorepo for the next version of Starship.

> [!NOTE]  
> Starship is currently running on the old TypeScript server and client, which can be found [here](https://github.com/starshipapp/starship-server) and [here](https://github.com/starshipapp/starship-web-client) respectively.
>
> These versions are no longer seeing active development.

## Projects
| Name                | Path                          | Description                                            |
|---------------------|-------------------------------|--------------------------------------------------------|
| Migrations          | `/migration`                  | The database migrations for the server, used by SeaORM.|
| Playground          | `/playground`                 | Testing playground for the Toolbox editor & runtime.   |
| Server              | `/starship-server`            | The GraphQL server for this version of Starship.       |
| Toolbox (library)   | `/toolbox_types`              | The library providing the shared runtime for Toolbox.  |

### Building
See the 'Getting Started' section of each project's README.

### Features
For the current implementation status of critical feature categories, see each project's README.

## Contributing
Contributions are welcome for non-Toolbox related improvements. Currently, this only includes the server.