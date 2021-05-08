# cya
A HTTP load testing utility written in Rust

## Installation
`cargo install cya`

## Usage
```
▶ cya --help
cya 0.0.4
A HTTP load testing utility

USAGE:
    cya [OPTIONS] <URL>

ARGS:
    <URL>    HTTP URL under test

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --connections <connections>    Maximum number of concurrent connections [default: 32]
    -r, --requests <requests>          Number of requests to send to the HTTP URL under test
                                       [default: 1000]
```

## Examples
TODO: flesh out