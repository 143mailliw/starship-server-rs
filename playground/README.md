# Toolbox (playground)

## Getting Started
> [!IMPORTANT]  
> The playground (as a result of it's dependencies) currently requires nightly rust to build properly.

To run the server, you must have at minimum:
- Cargo
- Rust (nightly)
- [Trunk](https://trunkrs.dev/)

Then run `trunk serve --open`.

## Features
| Feature              | Status        | Notes                                                                           |
|----------------------|---------------|---------------------------------------------------------------------------------|
| Rendering            | 10% complete  | Basic read-only rendering is implemented. Poorly tested.                        |
| Editing              | 4% complete   | Basic property editing (for text) and tree manipulation is implemented.         |
| Events               | 0% complete   | No support in `toolbox_types`.                                                  |
| Lua Execution        | 0% complete   | No support in `toolbox_types`.                                                  |
| Actions              | 0% complete   | No support in `toolbox_types`.                                                  |
| Server Actions       | 0% complete   | Not planned until the start of full client development.                         |
| User Types           | 0% complete   | No support in `toolbox_types` or `starship-server`.                             |
