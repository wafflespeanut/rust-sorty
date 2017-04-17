#![feature(plugin_registrar, plugin, box_syntax, rustc_private)]

#[macro_use]
extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

pub mod sorty;

use rustc_plugin::registry::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_early_lint_pass(box sorty::Sorty);
}
