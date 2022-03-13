use std::collections::HashSet;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{ImplItem, ItemImpl, Type};
use syn::spanned::Spanned;
use super::{
    ParsedAttr, ParsedFn
};


pub struct ParsedImpl {
    pub name: Ident,
    pub attrs: HashSet<ParsedAttr>,
    pub functions: Vec<ParsedFn>,
    item_impl: ItemImpl
}

impl ToTokens for ParsedImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.item_impl.to_token_stream());
    }
}

impl ParsedImpl {
    pub fn parse(input: &proc_macro::TokenStream, attrs: &proc_macro::TokenStream) -> syn::Result<Self> {
        let item_impl = syn::parse::<ItemImpl>(input.clone())?;
        let impl_clone = item_impl.clone();

        // extract impl name
        let name = match *item_impl.self_ty.clone() {
            Type::Path(p) => {
                p.path.segments.last().unwrap().ident.clone()
            }

            n => return Err(syn::Error::new(n.span(), "Wrong impl type"))
        };

        let mut functions = vec![];
        for impl_item in &item_impl.items {
            match impl_item {
                ImplItem::Method(m) => {
                    // only insert if Some. If it's None, it was annotated with jignore
                    if let Some(v) = ParsedFn::parse_impl_fn(m.clone(), name.clone(), &attrs)? {
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
            functions,
            item_impl: impl_clone
        })
    }
}
