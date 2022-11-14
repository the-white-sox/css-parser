# Contributing

This document includes instructions for how to contribute to this project including how to set up the development environment.

## Table of Contents

- [Installing and Updating Rust](#installing-and-updating-rust)
- [Setting up VS-Code](#setting-up-vs-code)
- [Useful Commands](#useful-commands)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Branch Naming Guidelines](#branch-naming-guidelines)
- [Code Style Guidelines](#code-style-guidelines)

## Installing and Updating Rust

If you don't have it already you will need to install rustup from https://rustup.rs

If you already have rust installed you can update it with the following command

```ps1
rustup update
```

If you are new to rust the [official Rust book](https://doc.rust-lang.org/book/) is an amazing reference for the basics of the Rust programing language

## Setting up VS-Code

We recommend using VS-Code to edit Rust code.

If you haven't already download and install VS-Code from https://code.visualstudio.com/download

Open the root director of the repo as a workspace.

Install the [rust-analyzer VS-Code Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Install the [Better TOML VS-Code Extension](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)

If you prefer using a GUI for unit tests instead of command line you might also like the [Rust Test Explorer VS-Code Extension](https://marketplace.visualstudio.com/items?itemName=swellaby.vscode-rust-test-adapter)

## Useful Commands

You can run the program with this command

```ps1
cargo run
```

You can run the unit tests with this command

```ps1
cargo test
```

You can format your code using

```ps1
cargo fmt
```

You can ask rust for advice on things you can improve about your program with

```ps1
cargo clippy
```

If you would like to compile the program into an exe you can do

```ps1
cargo build --release
```

your exe will be located at `/target/release/css-parser.exe`

## Commit Message Guidelines

Please follow the guidelines on https://www.conventionalcommits.org

You might find the [Conventional Commits VS-Code Extension](https://marketplace.visualstudio.com/items?itemName=vivaxy.vscode-conventional-commits) to be helpful.

## Branch Naming Guidelines

- Branch names should be lowercase.
- Use `-` to separate words
- Use `/` to denote containment (Think of it like folders)

Please name branches based in what changes you hope to make on that branch.

If you are making a branch that only you will use please place your name in the branch name to avoid accidentally naming your branches that same as someone else.

For example if Nate Stringham would like to a support for media queries he could name his branch `nstringham/media-query`

## Code Style Guidelines

Please format your Rust code with `cargo fmt` before you commit.

Please use [Prettier](https://prettier.io) for markdown files.

You may want to enable the `Format On Save` option in VS-Code settings
