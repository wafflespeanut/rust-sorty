use rustc::lint::{Context, LintPass, LintArray};
use std::cmp::Ordering;
use syntax::ast::{Mod, Item, Item_, Lit_, PathListItem_, ViewPath_, Visibility, MetaItem_};
use syntax::codemap::Span;
use syntax::print::pprust::path_to_string;

declare_lint!(UNSORTED_DECLARATIONS, Warn,
              "Warn when the declarations of crates or modules are not in alphabetical order");

pub struct Sorter;

impl LintPass for Sorter {
    fn get_lints(&self) -> LintArray {
        lint_array!(UNSORTED_DECLARATIONS)
    }

    fn check_mod(&mut self, cx: &Context, module: &Mod, _span: Span, _id: u32) {
        let session_codemap = cx.tcx.sess.codemap();
        let mut extern_crates = Vec::new();
        let mut uses = Vec::new();
        let mut mods = Vec::new();
        for item in &module.items {
            let item_name = format!("{}", item.ident.name.as_str());
            let item_span = item.span;
            match item.node.clone() {
                Item_::ItemExternCrate(_) if item_name != "std" => {
                    let item_attrs = get_item_attrs(&item, false);
                    extern_crates.push((item_name, item_attrs, item_span, false));
                },
                Item_::ItemMod(module) => {
                    let mod_invoked_file = session_codemap.span_to_filename(item.span);
                    let mod_declared_file = session_codemap.span_to_filename(module.inner);
                    if mod_declared_file != mod_invoked_file {      // ignores inline modules
                        let item_attrs = get_item_attrs(&item, true);
                        mods.push((item_name, item_attrs, item_span, false));
                    }
                },
                Item_::ItemUse(spanned) => {
                    let item_attrs = get_item_attrs(&item, true);
                    match spanned.node {
                        ViewPath_::ViewPathSimple(_, ref path) => {
                            uses.push((path_to_string(&path), item_attrs, item_span, false));
                        },
                        ViewPath_::ViewPathList(ref path, ref list) => {
                            let old_list = list
                                           .iter()
                                           .filter_map(|&list_item| {
                                                match list_item.node {
                                                    PathListItem_::PathListIdent { name, .. } => {
                                                        let interned = name.name.as_str();
                                                        let string = &*interned;
                                                        Some(string.to_owned())
                                                    },
                                                    _ => None,
                                                }
                                            }).collect::<Vec<String>>();
                            let mut new_list = old_list.clone();
                            new_list.sort_by(|a, b| {
                                match *a == "self" {    // `self` should be first in an use list
                                    true => Ordering::Less,
                                    false => a.cmp(b),
                                }
                            });
                            let mut warn = false;
                            for i in 0..old_list.len() {
                                if old_list[i] != new_list[i] {
                                    warn = true;
                                    break;
                                }
                            }
                            let use_list = format!("{}::{{{}}}", path_to_string(&path), new_list.connect(", "));
                            uses.push((use_list, item_attrs, path.span, warn));
                        },
                        ViewPath_::ViewPathGlob(ref path) => {
                            let path_str = path_to_string(&path);
                            // we don't have any use statements like `use std::prelude::*`
                            // since it's done only by rustc, we can safely neglect those here
                            if !path_str.starts_with("std::") {
                                uses.push((path_str, item_attrs, item_span, false));
                            }
                        },
                    }
                },
                _ => (),
            }
        }

        check_sort(&extern_crates, cx, "crate declarations", "extern crate");
        check_sort(&mods, cx, "module declarations", "mod");
        check_sort(&uses, cx, "use statements", "use");

        fn get_item_attrs(item: &Item, pub_check: bool) -> String {
            let attr_vec = item.attrs
                           .iter()
                           .filter_map(|attr| {
                                let meta_item = attr.node.value.node.clone();
                                let meta_string = get_meta_as_string(&meta_item);
                                match meta_string.starts_with("doc = ") {
                                    true => None,
                                    false => Some(format!("#[{}]", meta_string)),
                                }
                           }).collect::<Vec<String>>();
            let attr_string = attr_vec.connect("\n");
            match item.vis {
                Visibility::Public if pub_check => {
                    match attr_string.is_empty() {
                        true => "pub ".to_owned(),
                        false => attr_string + "\npub ",    // `pub` for mods and uses
                    }
                },
                _ => {
                    match attr_string.is_empty() {
                        true => attr_string,
                        false => attr_string + "\n",
                    }
                },
            }
        }

        fn get_meta_as_string(meta_item: &MetaItem_) -> String {
            match *meta_item {      // recursively collect the information from meta items into Strings
                MetaItem_::MetaWord(ref string) => format!("{}", string),
                MetaItem_::MetaList(ref string, ref meta_items) => {
                    let stuff = meta_items
                                .iter()
                                .map(|meta_item| {
                                    get_meta_as_string(&meta_item.node)
                                }).collect::<Vec<String>>();
                    format!("{}({})", string, stuff.connect(", "))
                },
                MetaItem_::MetaNameValue(ref string, ref literal) => {
                    let value = match literal.node {
                        Lit_::LitStr(ref inner_str, _style) => inner_str,
                        _ => panic!("unexpected literal found for meta item!"),
                    }; format!("{} = \"{}\"", string, value)
                },
            }
        }

        // slice of tuples with a name, related attributes, spans and whether to warn for an use list
        fn check_sort(old_list: &[(String, String, Span, bool)], cx: &Context, kind: &str, syntax: &str) {
            let length = old_list.len();
            let mut new_list = old_list
                                .iter()
                                .map(|&(ref name, ref attrs, _span, warn)| (name.clone(), attrs.clone(), warn))
                                .collect::<Vec<(String, String, bool)>>();
            new_list.sort_by(|&(ref str_a, ref attr_a, _), &(ref str_b, ref attr_b, _)| {
                match attr_b.ends_with("pub ") {
                    true => {   // a closure only to move the ordered `pub` statements to the bottom
                        let new_str_b = "~".to_owned() + &str_b;
                        match attr_a.ends_with("pub ") {
                            true => {   // since `~` is superior to almost all the ASCII chars
                                let new_str_a = "~".to_owned() + &str_a;
                                new_str_a.cmp(&new_str_b)
                            },
                            false => str_a.cmp(&new_str_b)
                        }
                    },
                    false => str_a.cmp(str_b),
                }
            });

            let mut span: Option<Span> = None;
            let message = format!("{} should be in alphabetical order!\nTry this...\n", kind);
            let mut suggested_list = Vec::new();
            suggested_list.push(message);
            for i in 0..length {
                let name = new_list[i].0.clone();
                if (old_list[i].0 != name) || new_list[i].2 {
                    let suggestion = format!("{}{} {};", new_list[i].1, syntax, name);
                    suggested_list.push(suggestion);
                    span = match span {
                        Some(old_span) => {
                            let mut sp = old_span;
                            sp.hi = old_list[i].2.hi;   // increase the span to include more lines
                            Some(sp)
                        },
                        None => Some(old_list[i].2),
                    }
                }
            }

            match span {
                Some(sp) => cx.span_lint(UNSORTED_DECLARATIONS, sp, &suggested_list.connect("\n")),
                None => (),
            }
        }
    }
}
