use rustc::lint::{Context, LintPass, LintArray};
use syntax::ast::{Mod, Item_, ViewPath_};
use syntax::codemap::Span;
use syntax::print::pprust::path_to_string;

declare_lint!(SORTED_DECLARATIONS, Warn,
              "Warn when the declarations of crates or modules are not in alphabetical order");

pub struct Sorter;

impl LintPass for Sorter {
    fn get_lints(&self) -> LintArray {
        lint_array!(SORTED_DECLARATIONS)
    }

    fn check_mod(&mut self, cx: &Context, module: &Mod, _span: Span, _id: u32) {
        let session_codemap = cx.tcx.sess.codemap();
        let mut extern_crates = Vec::new();
        let mut uses = Vec::new();
        let mut mods = Vec::new();
        for item in &module.items {
            let item_name = item.ident.name.as_str().to_owned();
            let item_span = item.span;
            match item.node.clone() {
                Item_::ItemExternCrate(_) if item_name != "std" => {
                    extern_crates.push((item_name, item_span));
                },
                Item_::ItemMod(module) => {
                    let mod_invoked_file = session_codemap.span_to_filename(item.span);
                    let mod_declared_file = session_codemap.span_to_filename(module.inner);
                    if mod_declared_file != mod_invoked_file {      // this ignores inline modules
                        mods.push((item_name, item_span));
                    }
                },
                Item_::ItemUse(spanned) => {
                    match spanned.node {
                        ViewPath_::ViewPathSimple(_, ref path) | ViewPath_::ViewPathList(ref path, _) => {
                            uses.push((path_to_string(&path), item_span));
                        },
                        ViewPath_::ViewPathGlob(ref path) => {
                            let path_str = path_to_string(&path);
                            // we don't have any use statements like `use std::prelude::*`
                            // since it's done only by rustc, we can safely neglect those here
                            match path_str.starts_with("std::") {
                                true => continue,
                                false => uses.push((path_str, item_span)),
                            }
                        },
                    }
                },
                _ => {},
            }
        }

        check_sort(&extern_crates, cx);
        check_sort(&mods, cx);
        check_sort(&uses, cx);

        fn check_sort(old_slice: &Vec<(String, Span)>, cx: &Context) {
            let mut new_slice = old_slice
                                .iter()
                                .map(|&(ref string, _span)| string.clone())
                                .collect::<Vec<String>>();
            new_slice.sort();
            for i in 0..old_slice.len() {
                let (declaration, span) = (old_slice[i].0.clone(), old_slice[i].1);
                if declaration != new_slice[i] {
                    let suggestion = format!("\n\texpected: {}\n\tfound: {}", new_slice[i], declaration);
                    cx.span_lint(SORTED_DECLARATIONS, span, &suggestion);
                }
            }
        }
    }
}
