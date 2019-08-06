# Beat Boards API

[![Build Status](https://travis-ci.com/beat-boards/api.svg?branch=master)](https://travis-ci.com/beat-boards/api)

## Contributing

We do not accept contributions yet, as the basics aren't even quite implemented.

### Requirements

Here is a list of requirements to build and run the API and the versions we currently use.

* Rust - 1.36
* PostgreSQL - 11.4
* Redis - 5.0
* Diesel CLI - 1.4 - `--no-default-features --features "postgres"`

You should rename .env.example to .env and set the variables according to your current setup. You will also need to restore the PostgreSQL database from [the provided dump](db.sql).

### Windows

Windows is not supported as the API uses Redis 5.0, which is not compatible with Windows.

## License

Our API is licensed under the [MIT License](LICENSE).