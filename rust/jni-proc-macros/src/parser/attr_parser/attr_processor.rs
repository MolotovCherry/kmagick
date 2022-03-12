use std::collections::HashMap;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{AttributeArgs, Lit, LitStr, Meta, NestedMeta};
use syn::spanned::Spanned;

pub(super) fn process_attrs(values: &mut HashMap<Ident, LitStr>, jtarget_ts: &mut TokenStream, attrs: &AttributeArgs) -> syn::Result<()> {
    for attr in attrs {
        let value;
        let name;

        match attr {
            // only accept a named argument
            NestedMeta::Meta(m) => {
                match m {
                    Meta::NameValue(nv) => {
                        name = nv.path.segments.last().unwrap().ident.clone();

                        match &nv.lit {
                            Lit::Str(s) => value = s.clone(),
                            n => return Err(syn::Error::new(n.span(), "Value must be a string"))
                        }
                    }

                    // for jtarget, not(target_os="foo")
                    Meta::List(l) => {
                        if name == "jtarget" {
                            // not() is in the path segments
                            if l.path.segments.len() == 1 {
                                let i = &l.path.segments.first().unwrap().ident;
                                if i != "not" {
                                    return Err(syn::Error::new(l.span(), r#"Only not(target_os="foo") is supported"#))
                                }

                                if l.nested.len() == 1 {
                                    match l.nested.first().unwrap() {
                                        NestedMeta::Meta(m) => {
                                            match m {
                                                _ => return Err(syn::Error::new(l.span(), r#"Only not(target_os="foo") is supported"#)),

                                                Meta::NameValue(n) => {
                                                    if n.path.segments.len() == 1 {
                                                        name = n.path.segments.first().unwrap().ident.clone();
                                                        value = match &n.lit {
                                                            Lit::Str(s) => s.clone(),
                                                            n => return Err(syn::Error::new(n.span(), "Value must be a string"))
                                                        }
                                                    } else {
                                                        return Err(syn::Error::new(l.span(), r#"Only not(target_os="foo") is supported"#))
                                                    }
                                                }
                                            }
                                        }

                                        _ => return Err(syn::Error::new(l.span(), r#"Only not(target_os="foo") is supported"#))
                                    }
                                } else {
                                    return Err(syn::Error::new(l.span(), r#"Only not(target_os="foo") is supported"#))
                                }

                                jtarget_ts.extend(quote! {
                                    not(target_os=#value)
                                });
                            } else {
                                return Err(syn::Error::new(l.span(), r#"Only not(target_os="foo") is supported"#))
                            }
                        } else {
                            // except for jtarget(not(target_os="foo")), all other possible values are name="value"
                            return Err(syn::Error::new(l.span(), r#"Format not in name="value" syntax"#))
                        }
                    }

                    p => return Err(syn::Error::new(p.span(), r#"Format not in name="value" syntax"#))
                }
            }

            // refuse direct literals with no keys
            NestedMeta::Lit(l) => return Err(syn::Error::new(l.span(), r#"Format not in name="value" syntax"#))
        }

        values.insert(name, value);
    }

    Ok(())
}
