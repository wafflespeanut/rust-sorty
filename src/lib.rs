#![feature(plugin_registrar, plugin, box_syntax, rustc_private)]

#[macro_use]
extern crate rustc;
#[macro_use]
extern crate syntax;

pub mod sorty;

use rustc::lint::LintPassObject;
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box sorty::Sorty as LintPassObject);
}
