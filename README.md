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

Windows is not supported as the API uses Redis, which is not compatible with Windows.

## Implementation

Basic explanation of language, framework and libraries choices, addressed at people wanting to base their API on this one or just get some ideas and advice.

### Speed

At the time, speed has been really, really impressive. No real stress testing has been done to this date, but we plan on it. On average, using a *development* build, single requests involving only JSON serialisation are processed in around 10 microseconds (0.01 ms), involving database queries and JSON serialisation in around 1000 microseconds (1 ms) and involving cache queries and no serialisation in 500 microseconds (0.5 ms). Most of the overhead in the two last cases seem to be coming from the query. 

### Language

We needed something fast, reliable and concurrent with a good web ecosystem. The initial ideas were C++, C#, Rust and Go. C++ and Go were eliminated because of personnal preference, and because of the lesser performance of Go web frameworks compared to the ones from other languages.

The choice of Rust was made for the same two reasons. First of, the sheer performance of Serde for JSON serialisation and deserialisation and because Auros wanted to learn Rust.

### Framework

Three main options were considered: [Actix web](https://github.com/actix/actix-web), [Tide](https://github.com/rustasync/tide), and [Thruster](https://github.com/trezm/Thruster). Tide was eliminated first because of its reliance on nightly, which would have been an issue in production. Once async/await gets to stable Rust, it would certainly be the best choice.

The choice of Thruster over Actix web might seem a bit weird as Actix web is insanely more popular and [tested in production](https://www.reddit.com/r/rust/comments/cdg5b4/rust_in_the_on_of_the_biggest_music_festival/). But [this post](https://www.reddit.com/r/rust/comments/ce09id/why_we_need_alternatives_to_actix/) made me choose Thruster. The dev behind Thruster seems to be putting a lot of effort into it and is actively maintaining it while trying to stay close to the community (I stumbled upon him a lot of time on Reddit posts while trying to make a choice). Thruster also seems to be as fast as Actix web in normal conditions.

### Database and cache

The choice of PostgreSQL was easy, it's battle tested, complete and the database with the best Rust ecosystem. And because of Diesel support of course. Redis was a go-to for cache, it has excellent Rust support and no real competiton.

The database implementation is pretty straightforward, using a R2D2 connection pool, Diesel for ORM and Serde for serialising query responses to JSON. The caching implementation also uses a R2D2 connection pool, but directly stores the serialized JSON as chunks. In case requests where `offset % chunk_size == 0` and `limit % chunk_size == 0`, no serialisation or deserialisation is needed and the preserialised JSON is returned as is with simple string concatenation if required, achieving 2-3x the speed of a database query.

### Recommendations

First of, we can definitely recommend Rust for building fast REST APIs, or pretty much any web service. The web ecosystem, and more widely the Rust ecosystem, are great in pretty much every way. We can't recommend Thruster for production yet as we haven't deployed yet, but updates will come once it is the case. We can still definitely recommend it for small projects and it has been a joy to work with.

### Special mentions

* [thruster-cli](https://github.com/trezm/thruster-cli) for getting us started with a pretty good template.
* [diesel-derive-enum](https://github.com/adwhit/diesel-derive-enum) for making working with Diesel and enums super easy.
* [r2d2-redis](https://github.com/sorccu/r2d2-redis) for making working with Redis and R2D2 as easy.
* [cargo-husky](https://github.com/rhysd/cargo-husky) for making sure commits don't break stuff.

## License

Our API is licensed under the [MIT License](LICENSE).
