# cursive-calendar-view

[![cursive-calendar-view on crates.io][cratesio-image]][cratesio]
[![cursive-calendar-view on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/cursive_calendar_view.svg
[cratesio]: https://crates.io/crates/cursive_calendar_view
[docsrs-image]: https://docs.rs/cursive_calendar_view/badge.svg?version=0.2.2
[docsrs]: https://docs.rs/cursive_calendar_view/0.2.2/

A basic calendar view implementation for [cursive](https://crates.io/crates/cursive).

![Month View](https://cloud.githubusercontent.com/assets/124674/25067601/b4f39c9e-2248-11e7-8bea-5d1a1c7669ac.png) ![Year View](https://cloud.githubusercontent.com/assets/124674/25067602/b6233084-2248-11e7-81e1-c7874b2c3d7c.png) ![Decade View](https://cloud.githubusercontent.com/assets/124674/25067604/c10f8bc8-2248-11e7-9c1c-f8a1b9fdff8c.png)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cursive_calendar_view = "0.2.2"
```

and this to your crate root:

```rust
extern crate cursive_calendar_view;
```

### Different backends

If you are using `cursive` with a different backend, you'll need to *forward*
the identical features to your `cursive_calendar_view` dependency:

```toml
[dependencies.cursive]
version = "0.7"
default-features = false
features = ["blt-backend"]

[dependencies.cursive_calendar_view]
version = "0.2.2"
default-features = false
features = ["blt-backend"]
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

