![Rust](https://github.com/K-JBoon/totpgen/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/totpgen?style=flat-square)](https://crates.io/crates/totpgen)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/K-JBoon/totpgen/blob/master/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/K-JBoon/totpgen/blob/master/LICENSE-MIT)


# totpgen

totpgen is a tool for managing and generating TOTP tokens on the command line quickly. You can
configure your tokens with the CLI interface or directly in your config directory.

## Installation

To install, you can either use

```
cargo install totpgen
```

or build the project yourself

```
$ git clone https://github.com/K-JBoon/totpgen
$ cd totpgen
$ cargo build --release
$ sudo mv ./target/release/totpgen /usr/bin/totpgen
```

## Usage

```
USAGE:
    totpgen <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete-token      Delete the token with the given ID
    generate-token    Generates a token for the given ID and current time
    help              Prints this message or the help of the given subcommand(s)
    insert-token      Insert or update a token in your configuration
    list-tokens       List all configured tokens
```

### Insert Token

Insert or update a token in your configuration

The formatting option allows you to specify a string where the following variables will be replaced:

- `{digits}`: The configured length of the token
- `{id}`: The ID of the token
- `{secret}`: The configured secret of the token
- `{timestep}`: The configured timestep of the token
- `{token}`: The generated token

```
USAGE:
    totpgen insert-token [OPTIONS] --id <id> --secret <secret> --digits <digits> --timestep <timestep>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --digits <digits>        The length to generate for this TOTP token
    -f, --format <format>        An optional formatting rule for the output of this token
    -i, --id <id>                A unique ID for this token
    -s, --secret <secret>        The secret to be used for this TOTP token
    -t, --timestep <timestep>    The timestep for this TOTP token
```

### Generate Token

Generate a TOTP token for the given ID and current time

```
USAGE:
    totpgen generate-token [FLAGS] <input>

    ARGS:
        <input>    The ID of the token to generate

    FLAGS:
        -h, --help                 Prints help information
        -i, --ignore-formatting    Ignore the specified formatting for the token in the output
        -V, --version              Prints version information
```

### Delete Token

Delete the token with the given ID

```
USAGE:
    totpgen delete-token --id <id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --id <id>
```

### List Tokens

List all configured tokens

```
USAGE:
    totpgen list-tokens

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```
