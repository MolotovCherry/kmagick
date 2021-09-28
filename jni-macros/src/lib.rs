use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{self, Expr, ReturnType};
use quote::{ToTokens, TokenStreamExt, quote};

mod utils;

#[proc_macro_attribute]
pub fn jnimethod(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let attrs = syn::parse_macro_input!(attr as syn::AttributeArgs);

    let args = match utils::get_args(attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let fn_inputs = &item_fn.sig.inputs;
    let res = utils::validate_fn_args(fn_inputs);
    match res {
        Err(e) => return e.to_compile_error().into(),
        Ok(v) => v
    }

    let env_ident = utils::extract_two_params(fn_inputs);
    let env_ident = match env_ident {
        Err(e) => return e.to_compile_error().into(),
        Ok(v) => v.0
    };

    let name = &item_fn.sig.ident;
    let name_str = name.to_string();
    let (java_return, is_result) = match utils::extract_return(&item_fn.sig.output, &name, None) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let is_returning = match java_return {
        ReturnType::Default => false,
        ReturnType::Type(_, _) => true
    };

    // cls is required
    let cls = args.get("cls");
    let java_fn;

    match cls {
        Some(v) => {
            java_fn = utils::class_to_ident(v, &name.to_string());
        }

        None => return syn::Error::new(Span::call_site(), "cls is a required attribute").to_compile_error().into()
    }

    let fn_call = utils::fn_call(fn_inputs, name);
    let fn_call = match fn_call {
        Ok(v) => v,
        Err(e) => return syn::Error::new(Span::call_site(), e.to_string()).to_compile_error().into()
    };

    let exc = args.get("exc");
    let exc = match exc {
        Some(v) => format!("\"{}\"", v),
        _ => String::from("\"java/lang/RuntimeException\"")
    };
    let exc = syn::parse_str::<Expr>(&exc);
    let exc = match exc {
        Ok(v) => v,
        Err(e) => return syn::Error::new(Span::call_site(), e.to_string()).to_compile_error().into()
    };

    // change the function output depending on whether it's a result type or not
    let inner_body = match is_result {
        true => {
            quote! {
                let res = #fn_call;
                match res {
                    Ok(v) => return v,
                    Err(e) => {
                        #env_ident.throw_new(#exc, e.to_string()).ok();
                        return ::std::ptr::null_mut();
                    }
                }
            }
        }

        false => {
            if is_returning {
                quote! {
                    return #fn_call;
                }
            } else {
                quote! {
                    #fn_call;
                }
            }
        }
    };

    let panic_body = match is_returning {
        true => {
            quote! {
                match panic_res {
                    Ok(_) => ::std::ptr::null_mut(),
                    Err(e) => {
                        #env_ident.throw_new("java/lang/RuntimeException", &format!("`{}()` panicked", #name_str)).ok();
                        ::std::ptr::null_mut()
                    }
                }
            }
        }

        false => {
            quote! {
                match panic_res {
                    Ok(_) => (),
                    Err(e) => {
                        #env_ident.throw_new("java/lang/RuntimeException", &format!("`{}()` panicked", #name_str)).ok();
                    }
                }
            }
        }
    };

    let new_tokens = quote! {
        #item_fn

        #[no_mangle]
        pub extern "C" fn #java_fn(#fn_inputs) #java_return {
            let panic_res = ::std::panic::catch_unwind(|| {
                #inner_body
            });

            #panic_body
        }
    };

    new_tokens.into()
}

/// Don't generate an implementation for a method in an impl
#[proc_macro_attribute]
pub fn jniignore(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn jniclass(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = syn::parse_macro_input!(item as syn::ItemImpl);
    let mut item_impl_mod = item_impl.clone();
    let attrs = syn::parse_macro_input!(attr as syn::AttributeArgs);

    let args = match utils::get_args(attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let pkg = args.get("pkg");
    let cls = args.get("cls");
    if let Some(_) = pkg {
        if let Some(_) = cls {
            return syn::Error::new(Span::call_site(), "Can't use both pkg and cls attributes at same time").to_compile_error().into();
        }
    }
    if let None = pkg {
        if let None = cls {
            return syn::Error::new(Span::call_site(), "Must specify either pkg or cls attributes").to_compile_error().into();
        }
    }

    let name = utils::extract_impl_name(&*item_impl_mod.self_ty);
    let name = match name {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    // (data, is_pkg *, ident)
    // * as opposed to is_cls
    let namespace = if pkg.is_some() {
        (pkg.unwrap(), true, &name)
    } else {
        (cls.unwrap(), false, &name)
    };

    // filter out ignored methods
    utils::filter_out_ignored(&mut item_impl_mod);

    let vl = utils::validate_impl_args(&item_impl_mod.items);
    match vl {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let env_idents = utils::impl_extract_two_params(&item_impl_mod.items);
    let env_idents = match env_idents {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let impl_returns = utils::validate_impl_returns(&item_impl_mod.items, &name);
    let impl_returns = match impl_returns {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };
    
    let exc = match args.get("exc") {
        Some(v) => &*v,
        None => "java/lang/RuntimeException"
    };

    let funcs = utils::generate_impl_functions(&item_impl_mod.items, &impl_returns, &env_idents, namespace, &exc);
    let funcs = match funcs {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    //println!("{:#?}", funcs);

    let mut stream = quote! {
        #item_impl
    };

    for f in funcs {
        stream.extend(f);
    }

    stream.into()
}
