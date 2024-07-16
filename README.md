# Astrid

Astrid is a CLI tool written in rust to perform ad-hoc tasks that I need.  \
This is mostly a toy project so I can learn more about CLI tools and configurating.

## Requirements

Astrid is written in rust and needs rust to be installed [here](https://www.rust-lang.org/tools/install).  \
The `yrun` command uses yarn which will need to be installed [here](https://classic.yarnpkg.com/lang/en/docs/install/).

## Building from Source

With rust installed.  \
The only step to building is to run `cargo build --release` in the root directory.  \
To have access to astrid globaly, `cargo install --path .` needs to be ran.

## Useage

Astrid requires a subcommand which can all be found when running `astrid -h`.

## Help

Running `astrid -h` will bring up the help meny.
