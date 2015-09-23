## rust-sorty

A lint to help with sorting the declarations of `mod`s, `extern crate`s and `use`s (along with the lists inside them), according to the style rules. It will be very handy for large projects written in Rust.

### Usage

Add this to your `Cargo.toml`...

``` toml
[dependencies.sorty]
git = "https://github.com/Wafflespeanut/rust-sorty"
```

... and to the module you wanna check,

``` rust
#![feature(plugin)]
#![plugin(sorty)]
```

... which can be made to show warnings or errors depending on your choice, just like any other lint.

``` rust
#![deny(unsorted_declarations)]         // throw errors! (poor choice for styling lints)

#![warn(unsorted_declarations)]         // show warnings (default)

#![allow(unsorted_declarations)]        // stay quiet!
```

### Note:

Firstly, this is a compiler lint, and it's unstable. So, make sure you're using the nightly Rust. And secondly, though this sorts the declarations in lexicographical order, it does follow some rules:

- stuff with `#[macro_use]` are at the top, since macros become visible to the surrounding [only after this declaration](https://doc.rust-lang.org/book/macros.html#scoping-and-macro-import/export), unlike others.
- `public` declarations (of uses & mods) are at the bottom
- `self` in use lists are moved to the left (other items are sorted as usual)
