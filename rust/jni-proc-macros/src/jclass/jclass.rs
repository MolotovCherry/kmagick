use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::ToTokens;
use syn::LitStr;

use crate::parser::{
    ParsedAttr, ParsedImpl
};
use crate::utils;


pub fn jclass_internal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = compile_err!(ParsedImpl::parse(item, attr));
    let attrs = compile_err!(ParsedAttr::parse(&item_impl.name, &attr));

    // Use either one, not both
    let mut pkg = attrs.get("pkg");
    let mut cls = attrs.get("cls");
    if pkg.is_some() && cls.is_some() {
        return syn::Error::new(Span::mixed_site(), "Can't use both pkg and cls attributes at same time").to_compile_error().into();
    } else if pkg.is_none() && cls.is_none() {
        return syn::Error::new(Span::mixed_site(), "Must specify either pkg or cls attributes").to_compile_error().into();
    }

    // runtime exception if not specified
    let exc = match attrs.get("exc") {
        Some(v) => utils::fix_class_path(&*v, true),
        None => LitStr::new("java/lang/RuntimeException", Span::mixed_site())
    };

    let funcs = compile_err!(
        super::impl_generator::generate_impl_functions(&item_impl, exc)
    );

    let mut stream = item_impl.to_token_stream();

    for f in funcs {
        stream.extend(f);
    }

    stream.into()
}
