use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::LitStr;
use crate::utils;
use crate::parser::{
    ParsedFn, ParsedAttr
};

pub fn jmethod_internal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = match compile_err!(ParsedFn::parse_fn(&item)) {
        // jignore was annotated on function - so no-op it
        None => return item,
        // normal function we want to keep
        Some(v) => v
    };

    let attrs = compile_err!(ParsedAttr::parse(&item_fn.name, &attr));

    // if jtarget tag exists, make a matching #[cfg()] tokenstream for later
    let mut target = proc_macro2::TokenStream::new();
    item_fn.call_attr("jtarget", |f| {
        target.extend(f.to_cfg_tokens());
    });

    // cls is required
    let java_fn = utils::java_fn_name(&attrs.get("cls").unwrap().value(), &item_fn.name());
    // default if not specified is java lang runtime exception
    let exc = attrs.get("exc").unwrap_or(&LitStr::new("java/lang/RuntimeException", Span::mixed_site()));

    let fn_name_str = item_fn.name();
    let fn_name = item_fn.name;

    let caller_args = item_fn.get_calling_args();
    let binding_args = item_fn.get_binding_args();
    let java_return = item_fn.ret_type;

    let res_binding = if item_fn.is_result {
        quote! {
            let c_res =
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let res_semicolon = if item_fn.is_returning {
        if item_fn.is_result {
            quote! { ; }
        } else {
            proc_macro2::TokenStream::new()
        }
    } else {
        quote! { ; }
    };

    let null_mut = if item_fn.is_returning {
        item_fn.null_ret_type
    } else {
        proc_macro2::TokenStream::new()
    };

    let v_or_underscore = if item_fn.is_returning {
        quote! { v }
    } else {
        quote! { _ }
    };

    let v_or_unit = if item_fn.is_returning {
        quote! { v }
    } else {
        quote! { () }
    };

    // change the function output depending on whether it's a result type or not
    let match_res = if item_fn.is_result {
        quote! {
            match c_res {
                Ok(#v_or_underscore) => #v_or_unit,
                Err(e) => {
                    let msg = format!("`{}` threw an exception : {}", #fn_name_str, e);
                    log::error!("{}", msg);
                    env.throw_new(#exc, msg).ok();

                    #null_mut
                }
            }
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let new_tokens = quote! {
        #item_fn

        #target
        #[no_mangle]
        pub extern "system" fn #java_fn(#binding_args) #java_return {
            let p_res = std::panic::catch_unwind(|| {
                #res_binding #fn_name(#caller_args)#res_semicolon

                #match_res
            });

            match p_res {
                Ok(#v_or_underscore) => #v_or_unit,
                Err(e) => {
                    let msg = &format!("`{}()` panicked", #fn_name_str);
                    log::error!("{}", msg);
                    env.throw_new("java/lang/RuntimeException", msg).ok();

                    #null_mut
                }
            }
        }
    };

    new_tokens.into()
}
