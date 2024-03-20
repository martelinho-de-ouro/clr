# cLr

[![ci](https://github.com/martelinho-de-ouro/clr/actions/workflows/ci.yml/badge.svg)](https://github.com/martelinho-de-ouro/clr/actions/workflows/ci.yml)
![coverage badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/helio-frota/d86fe4168b61872f7e63d85ce3c9fea6/raw/cov.json)

This repository contains everything I found useful-and-practical in the
`Command-Line Rust` book + other things.

## This uses the following

### Dependencies

* `clap`
* `assert_cmd`
* `toml`
* `serde`

### CI

* `actions/checkout@v4`
* `Swatinem/rust-cache@v2`
* `taiki-e/install-action@cargo-llvm-cov`
  * Local development workflow use the report to navigate on html and see uncovered code:

    ```sh
    cargo install cargo-llvm-cov
    cargo llvm-cov --html --open
    ```

* `schneegans/dynamic-badges-action@v1.7.0` to publish the coverage results to a `gist` and use that gist with `shields.io` to provide a coverage badge. (thanks to: <https://bitspittle.dev/blog/2022/kover-badge>)
