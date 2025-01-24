# Tinychange [![Crates.io](https://img.shields.io/crates/v/tinychange)](https://crates.io/crates/tinychange) [![npm](https://img.shields.io/npm/v/tinychange)](https://www.npmjs.com/package/tinychange) [![Docs.rs](https://img.shields.io/docsrs/tinychange)](https://docs.rs/tinychange) [![Crates.io](https://img.shields.io/crates/d/tinychange)](https://crates.io/crates/tinychange) 

## Description

A tool for creating tiny changelogs on a fly. Create a tinychange file for every notable change, and then merge them all into the main changelog file with just one command.

> This approach is heavily inspired by [Changesets](https://github.com/changesets/changesets).

### Main benefits:
- Painless pull request merges. No more merge conflicts in changelog file, as every tinychange is in its own file
- Changes are not tied to a specific commit ol pull request. Not every commit needs a changelog entry, and some commits may require multiple, so let the developer decide what to include.

## Installation

### From crates.io
```sh
$ cargo install --locked tinychange
```

### From NPM
```sh
$ npm install -g tinychange
```

### From git
```sh
$ cargo install --locked --force --git https://github.com/juh9870/tinychange
```

### From Releases page
Download the binary from the [Releases page](https://github.com/juh9870/tinychange/releases/latest)

### As a library
tinychange can also be invoked from a Rust program, for example, as a part of your [cargo xtask](https://github.com/matklad/cargo-xtask)

```sh
$ cargo add tinychange
```

See the [tinychange.rs](src/bin/tinychange.rs) file for an example of how to invoke the library.

## Usage

Use the `tinychange` command to create and merge tinychanges. This section only covers the basic usage. For more detailed information, use the `--help` flag.

### Initialize configuraion
Before using the tool, you need to initialize the configuration file. This will create a `tinychange.toml` configuration file, and `.tinychange` directory in the current working directory, where all the tinychange files will be stored.

```sh
$ tinychange init
```

### Create a tinychange
To create a new tinychange, use just run the `tinychange` command. This will show an interactive prompt where you can fill in the details of the change.

```sh
$ tinychange
```

> Author name will be pulled from the active git author/user, if available. If not, you will be prompted for it.

#### Script usage

You can also manually provide arguments to create a tinychange by using the `new` subcommand. This is useful for automation or scripting. Use the `-I` flag to disable the interactive prompts and silence the output.

```sh
$ tinychange new --kind Added --message "A changelog" --author juh9870
```

### Merge tinychanges
To merge all the tinychanges into the main changelog file, use the `merge` command.

```sh
$ tinychange merge
```

## Merging behavior

Changelogs files do not have a standard format, and it's impossible to predict every possible format, so the coice was made to target a [keep a changelog](https://keepachangelog.com/en/1.1.0/)-like format. The tool assumes the changelog is in the markdown format, with the sections indicated by a number of `#` characters. The tool will try to find any section whose header contains `unreleased` (eg. `## [Unreleased]`) and append the tinychanges there. If no such section is found, a default `## [Unreleased]` section will be created after the first found `changelog` section, but before the next header. If no `changelog` section is found, the tool will bail out.