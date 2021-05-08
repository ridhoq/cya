# cya
A HTTP load testing utility written in Rust

## Installation
`cargo install cya`

## Usage
```bash
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

Run with default values
```bash
▶ cya https://google.com
correlation id: 782cf1c9-0e87-439d-9f0c-cb3167dea90f
{
  "succeeded": 1000,
  "failed": 0,
  "histogram": {
    "min": 0.19085092,
    "p50": 0.312810605049921,
    "p95": 0.62576039240028,
    "p99": 1.212863284567282,
    "max": 1.440752238
  },
  "responseCodes": {
    "200 OK": 1000
  },
  "failureReasons": {
    "body": 0,
    "builder": 0,
    "connect": 0,
    "decode": 0,
    "redirect": 0,
    "status": 0,
    "timeout": 0
  }
}
```

Run 10000 requests with 1000 concurrent connections
```bash
▶ cya -r 10000 -c 1000 https://notsoperformant.com
correlation id: a3d3f8e5-c6df-4494-ad05-7b5fce4fcec8
{
  "succeeded": 2847,
  "failed": 7153,
  "histogram": {
    "min": 0.007582634,
    "p50": 2.7572858303668895,
    "p95": 19.868029980283385,
    "p99": 17.111358295062168,
    "max": 27.366927133
  },
  "responseCodes": {
    "200 OK": 2847
  },
  "failureReasons": {
    "body": 0,
    "builder": 0,
    "connect": 7153,
    "decode": 0,
    "redirect": 0,
    "status": 0,
    "timeout": 0
  }
}
```