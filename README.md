# Cratify 
[![Build Status](https://travis-ci.org/twilco/cratify.svg?branch=master)](https://travis-ci.org/twilco/cratify)

Want to know when another crate depends on yours?  We can notify (_cratify!_) you when this happens.  Here is a taste of some of Cratify's planned functionality:

* Periodic summary or immediate notification when crate(s) of your choice gain a new dependent crate.  Are you a crate maintainer who wants to know how many crates depend on each version of your crate?  We can do that!
* Periodic summary or immediate notification when crate(s) of your choice get a version bump.
* Interesting trivia about your crate subscriptions.  How many commits went into that last crate release?  How many total downloads do your crates have across all versions?  What is the average time between release for your crates?

Cratify is currently incomplete.  After launch, this README will be updated with a link to the site.  Until then, you can see Cratify by running it locally - check out the [developer guide](docs/developer-guide.md) to get started.

## How does it work?

* Sign up to receive cratifications for any crate (and optionally, a specific version of that crate).
* We monitor the [Cargo index](https://github.com/rust-lang/crates.io-index) for changes - when we see something you care about, we let you know via e-mail.
* And that's it!

## License

Licensed under either

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your decision.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
