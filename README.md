# Beat Boards API

[![Build Status](https://travis-ci.com/beat-boards/api.svg?branch=master)](https://travis-ci.com/beat-boards/api)

## Contributing

We do not accept contributions yet, as the basics aren't even quite implemented.

### Requirements

Here is a list of requirements to build and run the API and the versions we currently use.

* Rust - 1.36
* PostgreSQL - 11.4
* Diesel CLI - 1.4.0 - `--no-default-features --features "postgres"`

### Windows

Building and running on Windows is a bit more troublesome. 

* Include `<POSTGRES_DIR>\lib` in your `LIB` environment variable
* Include `<POSTGRES_DIR>\bin` and `<POSTGRES_DIR>\lib` in your `PATH`

### License

Our API is licensed under the [MIT License](LICENSE).