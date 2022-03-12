use proc_macro::TokenStream;
use quote::quote;
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
    // this is already verified, so unwrap is ok
    let java_fn = utils::java_fn_name(&attrs.get("cls").expect("cls key required").value(), &item_fn.name());

    let exc = attrs.get("exc").expect("exc key required");

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
                    let msg = format!("`{}` threw an exception : {}", #name_str, e);
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
        pub extern "system" fn #java_fn(env: jni::JNIEnv #fn_inputs) #java_return {
            let p_res = std::panic::catch_unwind(|| {
                #res_binding #real_fn_name(#fn_call)#res_semicolon

                #match_res
            });

            match p_res {
                Ok(#v_or_underscore) => #v_or_unit,
                Err(e) => {
                    let msg = &format!("`{}()` panicked", #name_str);
                    log::error!("{}", msg);
                    env.throw_new("java/lang/RuntimeException", msg).ok();

                    #null_mut
                }
            }
        }
    };

    new_tokens
}
