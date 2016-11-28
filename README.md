## rust-sorty

[![Build Status](https://travis-ci.org/Wafflespeanut/rust-sorty.svg?branch=master)](https://travis-ci.org/Wafflespeanut/rust-sorty)
[![Current Version](https://meritbadge.herokuapp.com/sorty)](https://crates.io/crates/sorty)

A lint to help with sorting the `extern crate`, `mod` and `use` declarations, according to the style rules. Have a look at the [detailed example](https://github.com/Wafflespeanut/rust-sorty/tree/master/EXAMPLE.md) for a start! I guess it will be very handy for large projects written in Rust (well, at least once!).

And yeah, this should actually be done by **[rustfmt](https://github.com/rust-lang-nursery/rustfmt)**, but it [doesn't have this option](https://github.com/rust-lang-nursery/rustfmt/issues/298) for now. So, this plugin would serve until `rustfmt` becomes intelligent enough to detect the unsorted declarations.

### Usage

Add this to your `Cargo.toml`...

``` toml
sorty = "0.1"
```

... and then to the top of the main module you wanna check,

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

Remove it once you've done all the checks, when you'll no longer be needing the plugin!

I was just kidding. I'll be very happy if you just keep it :)

### Note:

This is a compiler lint, and it's unstable. So, make sure you're using the latest [nightly Rust](https://www.rust-lang.org/install.html). Though this lint shows an output of the lexicographically sorted declarations, it follows some rules:

- stuff with `#[macro_use]` are sorted and moved to the top, since macros become visible to the surroundings [only after their declaration](https://doc.rust-lang.org/book/macros.html#scoping-and-macro-importexport), unlike others.
- `pub` declarations (of uses & mods) are sorted and moved to the bottom
- `self` in use lists are moved to the left (other list items are sorted as usual)

Also, note that there are some stuff that aren't tracked (for now). It includes comments, spaces, etc.
