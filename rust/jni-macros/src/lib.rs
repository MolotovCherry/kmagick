use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{self, Expr, ReturnType};
use quote::{ToTokens, quote};

mod utils;


// wrap a function for jni
#[proc_macro_attribute]
pub fn jmethod(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let attrs = syn::parse_macro_input!(attr as syn::AttributeArgs);

    let target = utils::get_cfg_target(&item_fn.attrs);

    let args = match utils::get_args(attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let fn_inputs = utils::fn_full_args(&item_fn.sig.inputs);
    let (fn_call, fn_inputs) = match fn_inputs {
        Err(e) => return e.to_compile_error().into(),
        Ok(v) => v
    };
    let res = utils::validate_fn_args(&item_fn.sig.inputs, false, &vec![]);
    match res {
        Err(e) => return e.to_compile_error().into(),
        Ok(v) => v
    }

    let fn_name = args.get("name");
    let fn_name = match fn_name {
        Some(v) => v.to_token_stream(),
        None => item_fn.sig.ident.to_token_stream()
    };
    let real_fn_name = &item_fn.sig.ident;
    let name_str = item_fn.sig.ident.to_string();
    let (java_return, return_ident, is_result) = match utils::extract_return(&item_fn.sig.output, &item_fn.sig.ident, None, &utils::top_attrs(&item_fn.attrs)) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let is_returning = match java_return {
        ReturnType::Default => false,
        ReturnType::Type(..) => true
    };

    // cls is required
    let cls = args.get("cls");
    let java_fn = match cls {
        Some(v) => utils::class_to_ident(v, &fn_name),

        None => return syn::Error::new(Span::mixed_site(), "cls is a required attribute").to_compile_error().into()
    };

    let exc = args.get("exc");
    let exc = match exc {
        Some(v) => format!("\"{}\"", v),
        _ => String::from("\"java/lang/RuntimeException\"")
    };
    let exc = syn::parse_str::<Expr>(&exc);
    let exc = match exc {
        Ok(v) => v,
        Err(e) => return syn::Error::new(Span::mixed_site(), e.to_string()).to_compile_error().into()
    };


    let res_binding = if is_result {
        quote! {
            let c_res =
        }
    } else {
        proc_macro2::TokenStream::new()
    };

    let res_semicolon = if is_returning {
        if is_result {
            quote! { ; }
        } else {
            proc_macro2::TokenStream::new()
        }
    } else {
        quote! { ; }
    };

    let null_mut = if is_returning {
        utils::get_null_return_obj(&*return_ident)
    } else {
        proc_macro2::TokenStream::new()
    };

    let v_or_underscore = if is_returning {
        quote! { v }
    } else {
        quote! { _ }
    };

    let v_or_unit = if is_returning {
        quote! { v }
    } else {
        quote! { () }
    };

    // change the function output depending on whether it's a result type or not
    let match_res = if is_result {
        quote! {
            match c_res {
                Ok(#v_or_underscore) => #v_or_unit,
                Err(e) => {
                    let cls = env.find_class(#exc).ok();
                    let msg = format!("`{}` threw an exception : {}", #name_str, e);
                    log::error!("{}", msg);
                    if cls.is_some() {
                        env.throw_new(cls.unwrap(), msg).ok();
                    } else {
                        env.throw_new(#exc, msg).ok();
                    }

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
        pub extern "system" fn #java_fn(env: jni::JNIEnv#fn_inputs) #java_return {
            let p_res = std::panic::catch_unwind(|| {
                #res_binding #real_fn_name(#fn_call)#res_semicolon

                #match_res
            });

            match p_res {
                Ok(#v_or_underscore) => #v_or_unit,
                Err(e) => {
                    let cls = env.find_class("java/lang/RuntimeException").ok();
                    let msg = &format!("`{}()` panicked", #name_str);
                    log::error!("{}", msg);
                    if cls.is_some() {
                        env.throw_new(cls.unwrap(), msg).ok();
                    } else {
                        env.throw_new("java/lang/RuntimeException", msg).ok();
                    }

                    #null_mut
                }
            }
        }
    };

    new_tokens.into()
}

// set the function name used for jni - this way you can use whatever actual function name you want
// used for impl statements. for jmethod, use name attribute instead
#[proc_macro_attribute]
pub fn jname(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

/// Don't generate an implementation for a method in an impl
#[proc_macro_attribute]
pub fn jignore(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

// call as static function instead of instance function
#[proc_macro_attribute]
pub fn jstatic(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

// take the object from the handle allowing it to be dropped
#[proc_macro_attribute]
pub fn jdestroy(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

// set a handle to Self
#[proc_macro_attribute]
pub fn jnew(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

// allow a function to be conditionally compiled in the resulting generated output
// uses same format as #[cfg(target_os)]
// note: you still are required to use #[cfg(target_os)] on the original impl function
#[proc_macro_attribute]
pub fn jtarget(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

// Change the object that's gotten when using a regular instance function
#[proc_macro_attribute]
pub fn jget(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

// Change the object variable that's set from default to another one when using jnew
#[proc_macro_attribute]
pub fn jset(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

// Change the variable which is taken when used with jdestroy
#[proc_macro_attribute]
pub fn jtake(_: TokenStream, item: TokenStream) -> TokenStream {
    // even though this is a no-op, this validates that it is an ItemFn and not something else
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    item_fn.to_token_stream().into()
}

// wrap an entire impl for jni, including all functions inside
#[proc_macro_attribute]
pub fn jclass(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = syn::parse_macro_input!(item as syn::ItemImpl);
    let mut item_impl_mod = item_impl.clone();
    let attrs = syn::parse_macro_input!(attr as syn::AttributeArgs);

    let args = match utils::get_args(attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let mut pkg = args.get("pkg");
    let mut cls = args.get("cls");
    if let Some(_) = pkg {
        if let Some(_) = cls {
            return syn::Error::new(Span::mixed_site(), "Can't use both pkg and cls attributes at same time").to_compile_error().into();
        }
    }
    if let None = pkg {
        if let None = cls {
            return syn::Error::new(Span::mixed_site(), "Must specify either pkg or cls attributes").to_compile_error().into();
        }
    }

    let f: String;
    let c: String;
    if let Some(v) = pkg {
        f = utils::fix_class_path(v, false);
        pkg = Some(&f);
    }
    if let Some(v) = cls {
        c = utils::fix_class_path(v, false);
        cls = Some(&c);
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

    let impl_returns = utils::validate_impl_returns(&item_impl_mod.items, &name);
    let impl_returns = match impl_returns {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let exc = match args.get("exc") {
        Some(v) => utils::fix_class_path(&*v, true),
        None => "java/lang/RuntimeException".to_owned()
    };

    let funcs = utils::generate_impl_functions(&item_impl_mod.items, &impl_returns, namespace, &exc);
    let funcs = match funcs {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    let mut stream = item_impl.to_token_stream();

    for f in funcs {
        stream.extend(f);
    }

    stream.into()
}
