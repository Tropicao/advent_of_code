# Advent Of Code Template

This directory contains a minimal template to start working on [Advent of Code](https://adventofcode.com) challenges in Rust

## Dependencies

You need to install `cargo-generate` in order to be able to pull the template :

```sh
cargo install cargo-generate
```

## Usage

* Generate a new project for the current day challenge :
```
cargo generate --git Tropicao/aoc_template -d year=2022 -d day=1 -n <challenge_name> --vcs none <directory_name>
```
* Open `src/input.txt` and replace comments with your personnalized input
* The template offers a Visual Studio Code configuration which allows you to :
  * run the tests
  * format the code
  * lint the code
  * run you code against your personnalized input
To benefit the VSCode integration, install the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension