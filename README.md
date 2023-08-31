


# Pipe Viewer – A Unix Utility You Should Know About [Video]
Powerful Unix program called Pipe Viewer or pipev for short. Pipe viewer is a terminal-based tool for monitoring the progress of data through a pipeline. It can be inserted into any normal pipeline between two processes to give a visual indication of how quickly the data is passing through, how long it has taken, how near to completion it is, and an estimate of how long it will be until completion.

This project replicates _some_ of the functionality of [pv], but the main focus of this
project is to create the hands-on project so as to text my knowlege Rust
programming language. I hope you find this repository useful. 

[pv]: http://www.ivarch.com/programs/pv.shtml

## Update: 2022-06-17

Changes since tag `5.5`:

- Update `pipeviewer`'s version to `1.0.1` (and add a corresponding tag)
- Update from 2018 to 2021 edition of Rust in `Cargo.toml`
- Update `crossbeam` to version `0.8.1` in `Cargo.toml`
- Update `crossterm` to version `0.23.2` in `Cargo.toml`. In `src/stats.rs`, bring `crossterm::style::Stylize` into scope in a `use` statement, and change `cursor::MoveToColumn(0)` to `cursor::MoveToColumn(1)`.
- Update `clap` to version `3.2.5` in `Cargo.toml`. In `src/args.rs`, change `.short("o")` to `.short('o')` and `.short("s")` to `.short('s')`.
- Update all deep dependencies by running `cargo update`