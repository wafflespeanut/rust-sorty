## rust-sorty

A lint to help with sorting the `extern crate`, `mod` and `use` declarations, according to the style rules. It will be very handy for large projects written in Rust (at least once!). [Have a look at the detailed sample!](https://github.com/Wafflespeanut/rust-sorty/tree/master/SAMPLE.md)

### Usage

Add this to your `Cargo.toml`...

``` toml
[dependencies.sorty]
git = "https://github.com/Wafflespeanut/rust-sorty"
```

... and then to the main module you wanna check,

``` rust
#![feature(plugin)]
#![plugin(sorty)]
```

(It can show warnings or errors based on your choice, just like any other lint)

``` rust
#![deny(unsorted_declarations)]         // throw errors! (poor choice for styling lints)

#![warn(unsorted_declarations)]         // show warnings (default)

#![allow(unsorted_declarations)]        // stay quiet!
```

*Remove it once you've done all the checks, when you no longer need the plugin!* I was just kidding. I'll be very happy if you just keep it :)

### Note:

This is a compiler lint, and it's unstable. So, make sure you're using the nightly Rust (v1.3.0). Though this lint shows an output of the lexicographically sorted declarations, it follows some rules:

- stuff with `#[macro_use]` are sorted and moved to the top, since macros become visible to the surroundings [only after that declaration](https://doc.rust-lang.org/book/macros.html#scoping-and-macro-import/export), unlike others.
- `pub` declarations (of uses & mods) are sorted and moved to the bottom
- `self` in use lists are moved to the left (other list items are sorted as usual)
