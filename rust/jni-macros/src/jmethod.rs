use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::LitStr;

use crate::parser::{
    ParsedAttr, ParsedFn
};

pub fn jmethod_internal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = match compile_err!(ParsedFn::parse_fn(&item, &attr)) {
        // jignore was annotated on function - so no-op it
        None => return item,
        // normal function we want to keep
        Some(v) => v
    };

    let attrs = compile_err!(ParsedAttr::parse(&item_fn.bind_name, &attr));

    // if jtarget tag exists, make a matching #[cfg()] tokenstream for later
    let cfg = if item_fn.attrs.contains("cfg") {
        item_fn.get_attr("cfg").unwrap().to_token_stream()
    } else {
        proc_macro2::TokenStream::new()
    };

    // cls is required
    let java_fn = &item_fn.java_binding_fn_name;
    // default if not specified is java lang runtime exception
    let exc_ = LitStr::new("java/lang/RuntimeException", Span::mixed_site());
    let exc = attrs.get("exc").unwrap_or(&exc_);

    let fn_name_str = item_fn.orig_name.to_string();
    let fn_name = &item_fn.orig_name;

    let caller_args = &item_fn.calling_fn_args;
    let binding_args = &item_fn.binding_fn_args;
    let java_return = &item_fn.ret_type;
    let env = &item_fn.env_name;

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

    let mut null_mut = proc_macro2::TokenStream::new();
    if item_fn.is_returning {
        null_mut.extend(item_fn.null_ret_type.clone());
    }

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
                    log::error!("`{}` threw an exception: {:?}", #fn_name_str, e);
                    let _ = #env.throw_new(#exc, "`{}`: {}", #fn_name_str, e.to_string());

                    #null_mut
                }
            }
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let new_tokens = quote! {
        #item_fn

        #cfg
        #[no_mangle]
        pub extern "system" fn #java_fn(#binding_args) #java_return {
            let p_res = std::panic::catch_unwind(|| {
                #res_binding #fn_name(#caller_args)#res_semicolon

                #match_res
            });

            match p_res {
                Ok(#v_or_underscore) => #v_or_unit,
                Err(e) => {
                    let msg;
                    let e = e.downcast_ref::<&'static str>();
                    if let Some(r) = e {
                        msg = format!("`{}()` panicked: {}", #fn_name_str, r);
                    } else {
                        msg = format!("`{}()` panicked", #fn_name_str);
                    }

                    log::error!("{}", msg);
                    #env.throw_new("java/lang/RuntimeException", msg).ok();

                    #null_mut
                }
            }
        }
    };

    new_tokens.into()
}
