# process-logger (gdext evaluation project)

This is part of a project to evaluate the godot-rust bindings [gdext](https://github.com/godot-rust/gdext).

The evaluation project consists of four Repositories:

- [this](https://github.com/franziskusz/process-logger)
- [Godot Rust benchmark game](https://github.com/franziskusz/dodge-r)
- [Godot GDScript benchmark game](https://github.com/franziskusz/dodge-gds)
- [python-pandas-plotter](https://github.com/franziskusz/pandas-plotter)

## Setup

A:
If the prebuild executables should not work for you, you can build the project from source with the following steps:
1. [Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
2. Clone this repository by opening a shell and entering:
   - `git clone git@github.com:franziskusz/process-logger.git` (requires having a ssh key setup)
   - or `git clone https://github.com/franziskusz/process-logger.git` 
3. Change to the just cloned repository directory with `cd process-logger` (Unix)
- Run `cargo run` to run the project with cargo
- Run `cargo build` or `cargo build --release` to build an executable

B:
With an executable (either way downloaded from releases or build with the above instructions
1. Open Terminal and go to the directory where you downloaded it to / saved it to.
2. Give it read/write/executing rights: `chmod +rwx ./process-logger`. (Unix)
3. execute it with `./process-logger`

## How to use this
This is a simple CLI process logger:

1. You will be asked to enter a process name you wish to log (this might not work correctly on Linux: [sysinfo docs](https://docs.rs/sysinfo/0.30.6/sysinfo/struct.Process.html)).
2. Next you will be asked to enter the duration you wish to log the process (in seconds).
3. For the given time, a set of data (CPU usage, RAM, written/read bytes) will be written to a .csv file every second. You can always abort with ctrl-c.
4. You can find the log in a directory above the location of the binary/rust crate with the name `/process_stats/`.
5. The log file names will be `[process-name]-[timestamp-secs].csv`.

## Notes

- I am new to Rust. This whole project is also a learning experience for me. If the way some things are implemented gives you headaches, I am sorry. I am open for any kind of criticism.
- Befor the program asks you to enter a process name it will print a message to the shell, if the sysinfo crate supports your operating system
- .csv logging is based on the Rust crate [csv](https://crates.io/crates/csv)
- system/process monitoring is based on the Rust crate [sysinfo](https://crates.io/crates/sysinfo)
