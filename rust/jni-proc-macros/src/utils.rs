use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{format_ident, ToTokens};
use syn::{Attribute, LitStr};


pub fn java_fn_name(class: &str, fn_name: &str) -> TokenStream {
    let class = class.replace("/", "_").replace(".", "_").replace("\"", "");
    let fn_name = fn_name.to_string().replace("\"", "");

    format_ident!("Java_{}_{}", class, fn_name).to_token_stream()
}

pub fn fix_class_path(cls: &LitStr, slashes: bool) -> LitStr {
    // if not slashes, then underscores
    let cls = cls.value();
    let res = if slashes {
        cls.replace(".", "/").replace("_", "/")
    } else {
        cls.replace("/", "_").replace(".", "_")
    };

    LitStr::new(&res, Span::mixed_site())
}

pub fn get_set_take_attrs(attributes: &Vec<Attribute>) -> (Option<String>, Option<String>, Option<String>) {
    let mut jget_option = None;
    let mut jget = false;
    let mut jset_option = None;
    let mut jset = false;
    let mut jtake_option = None;
    let mut jtake = false;
    for attr in attributes {
        if attr.path.segments.len() == 0 {
            continue;
        }

        let last = attr.path.segments.last().unwrap();
        if last.ident.to_string() == "jget" {
            jget = true;
        } else if last.ident.to_string() == "jset" {
            jset = true;
        } else if last.ident.to_string() == "jtake" {
            jtake = true;
        }

        let mut passed = false;
        for token in attr.tokens.clone() {
            if let TokenTree::Group(g) = token {
                for t in g.stream() {
                    if let TokenTree::Ident(i) = &t {
                        if i.to_string() == "from" && (jget || jtake) {
                            passed = true;
                        } else if i.to_string() == "to" && jset {
                            passed = true;
                        }
                    }

                    if passed {
                        if let TokenTree::Literal(l) = &t {
                            let value = Some(l.to_string().replace("\"", ""));

                            if jget {
                                jget_option = value;
                                break;
                            } else if jset {
                                jset_option = value;
                                break;
                            } else if jtake {
                                jtake_option = value;
                                break;
                            }
                        }
                    }
                }
            }
        }

        jget = false;
        jset = false;
        jtake = false;
    }

    (jget_option, jset_option, jtake_option)
}
