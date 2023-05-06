# cidranger

Command-line utility to match an IP address against classless inter-domain routing (CIDR) notation.

## Running

    cargo run

For example:

    cargo run -- --ip=127.0.0.1 --ranges-url=https://developers.google.com/static/search/apis/ipranges/googlebot.json

## Exit codes

| Code | Description |
| ---- | --- |
| `0` | Success. IP was matched against one or more ranges |
| `1` | Error. IP was not matched against any of the ranges |

## Test suite

To run the test suite, e.g. when developing a feature:

    cargo test

## Licence

Copyright © 2023 Ain Tohvri. Licenced under [GPL-3](https://github.com/ain/botranger/blob/main/LICENSE).
