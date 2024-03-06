# process-logger (gdext evaluation project)

This is part of a project to evaluate the godot-rust bindings [gdext](https://github.com/godot-rust/gdext).

The evaluation project consists of four Repositories:

- [this](https://github.com/franziskusz/process-logger)
- [Godot Rust benchmark game](https://github.com/franziskusz/dodge-r)
- [Godot GDScript benchmark game](https://github.com/franziskusz/dodge-gds)
- [python-pandas-plotter](https://github.com/franziskusz/pandas-plotter)

## Setup
(assuming rust compiler and cargo is installed)
1. Clone this Repository
2. Run `cargo build`

or (UNIX)
1. Download a binary from the releases.
2. Open Terminal and go to the directory where you downloaded it to / saved it to.
3. Give it read/write/executing rights: `chmod +rwx [binary-name]`.
4. execute it with `./[binary-name]`

## How to use this
This is a simple CLI process logger.

## Notes

- As said before, I am new to Godot and Rust. This whole project is also a learning experience for me. If the way some things are implemented gives you headaches, I am sorry. I am open for any kind of criticism.
- .csv logging is based on the Rust crate [csv](https://crates.io/crates/csv)
