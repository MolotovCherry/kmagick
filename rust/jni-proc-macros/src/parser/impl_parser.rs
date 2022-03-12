use std::collections::HashSet;
use proc_macro2::Ident;
use syn::{ImplItem, ItemImpl, Type};
use syn::spanned::Spanned;
use super::{
    ParsedAttr, ParsedFn
};


pub struct ParsedImpl {
    pub name: Ident,
    pub attrs: HashSet<ParsedAttr>,
    pub functions: Vec<ParsedFn>
}

impl ParsedImpl {
    pub fn parse(input: proc_macro::TokenStream) -> syn::Result<Self> {
        let item_impl = syn::parse::<ItemImpl>(input)?;

        // extract impl name
        let name = match *item_impl.self_ty {
            Type::Path(p) => {
                p.path.segments.last().unwrap().ident.clone()
            }

            n => return Err(syn::Error::new(n.span(), "Wrong impl type"))
        };

        let mut functions = vec![];
        for impl_item in item_impl.items {
            match impl_item {
                ImplItem::Method(m) => {
                    // only insert if Some. If it's None, it was annotated with jignore
                    if let Some(v) = ParsedFn::parse_impl_fn(m)? {
                        functions.push(v);
                    }
                }

                // ignore everything that's not a method
                _ => continue
            }
        }

        let mut attrs = HashSet::new();
        for attr in item_impl.attrs {
            attrs.insert(
                ParsedAttr::parse(
                    &attr.path.segments.last().unwrap().ident,
                    &attr.tokens.into()
                )?
            );
        }

        Ok(Self{
            name,
            attrs,
            functions
        })
    }

    pub fn has_attr(&self, name: &str) -> bool {
        self.attrs.contains(name)
    }
}
