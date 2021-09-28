use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, ImplItem, ItemImpl, Meta, NestedMeta, Pat, PathArguments, ReturnType, Type};
use quote::{quote, ToTokens};

pub fn get_args(args: Vec<NestedMeta>) -> syn::Result<HashMap<String, String>> {
    let mut hm: HashMap<String, String> = HashMap::new();

    let allowed_args = vec!["cls", "exc", "pkg", "handler_trait"];

    if args.is_empty() {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), format!("Attributes are required")))
    }

    for arg in &args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.segments.is_empty() {
                    return Err(syn::Error::new_spanned(nv, "Empty segments"));
                }

                let ident = nv.path.segments.first().unwrap().ident.clone();
                if !allowed_args.contains(&&*ident.to_string()) {
                    return Err(syn::Error::new_spanned(&nv.path, format!("`{}` is not a valid attribute", ident.to_string())));
                }

                let val = if let syn::Lit::Str(lit) = &nv.lit {
                    lit.value()
                } else {
                    return Err(syn::Error::new_spanned(&nv.lit, "Expected a string literal"));
                };

                hm.insert(ident.to_string(), val);
            }

            arg => {
                return Err(syn::Error::new_spanned(arg, "Unknown attribute"));
            }
        }
    }

    Ok(hm)
}

pub fn class_to_ident(class: &str, fn_name: &str) -> Ident {
    let name = {
        let cls = class.replace("/", "_").replace(".", "_");
        format!("Java_{}_{}", cls, fn_name)
    };
    
    Ident::new(&name, proc_macro2::Span::call_site())
}

// Ok result is (ReturnType, is_result_type)
pub fn extract_return(ret: &ReturnType, name: &Ident, impl_name: Option<&Ident>) -> syn::Result<(ReturnType, bool)> {
    let allowed_ret = vec![
        "jarray", "jboolean", "jbooleanArray", "jbyte", "jbyteArray",
        "jchar", "jcharArray", "jclass", "jdouble", "jdoubleArray",
        "jfieldID", "jfloat", "jfloatArray", "jint", "jintArray", "jlong",
        "jlongArray", "jmethodID", "jobject", "jobjectArray", "jshort", "jshortArray",
        "jsize", "jstring", "jthrowable", "jweak"
    ];

    match &ret {
        ReturnType::Type(arrow_token, ty) => {
            let _ty = &**ty;
            match _ty {
                syn::Type::Path(ref v) => {
                    if v.path.segments.is_empty() {
                        return Err(syn::Error::new_spanned(v, "Empty segments"));
                    }

                    let segment = v.path.segments.last().unwrap();

                    let ident = &segment.ident;
                    let ident_str = ident.to_string();
                    let name_str = name.to_string();

                    if name_str == "destroy" {
                        return Err(syn::Error::new_spanned(v, "Destroy method cannot have return type"))
                    }

                    if impl_name.is_some() {
                        // restrict return type of Self::new()
                        if ident_str != "Self" && ident_str != impl_name.unwrap().to_string() && name_str == "new" {
                            return Err(syn::Error::new_spanned(v, "Return type must be a `Self` type"))
                        }
                    }

                    if ident_str != "Result" && !allowed_ret.contains(&&*ident_str) {
                        // special case - allow new() functions to return Self for the jniclass implementation
                        if name_str != "new" {
                            return Err(syn::Error::new_spanned(ident, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
                        } else if impl_name.is_some() {
                            if ident_str != "Self" && ident_str != impl_name.unwrap().to_string() {
                                return Err(syn::Error::new_spanned(ident, "Return type must be a Result<> type, primitive j type (jni::sys::*), `Self` type, or empty"))
                            }
                        } else {
                            return Err(syn::Error::new_spanned(ident, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
                        }
                    }

                    let args = &segment.arguments;
                    match args {
                        PathArguments::AngleBracketed(a) => {
                            let a_args = &a.args;
                            if a_args.len() == 0 {
                                return Err(syn::Error::new_spanned(args, "Angle brackets <> need an Ok type"))
                            }

                            let _ok_res = &a_args[0];
                            let ok_res: syn::Type;
                            match _ok_res {
                                syn::GenericArgument::Type(ref v) => {
                                    // validate it's an allowed type
                                    match v {
                                        syn::Type::Path(i_v) => {
                                            let seg = &i_v.path.segments;
                                            if seg.len() == 0 {
                                                return Err(syn::Error::new_spanned(args, "Empty segments"));
                                            }

                                            let ident = &*seg.first().unwrap().ident.to_string();
                                            if !allowed_ret.contains(&ident) {
                                                return Err(syn::Error::new_spanned(v, "Return type must be a primitive j type (jni::sys::*)"))
                                            }

                                            ok_res = v.clone();
                                        }

                                        _ => return Err(syn::Error::new_spanned(&segment, "Wrong type"))
                                    }
                                }

                                _ => {
                                    return Err(syn::Error::new_spanned(args, "Wrong type"))
                                }
                            }

                            // reconstruct return type
                            let typebox = Box::new(ok_res);
                            Ok((ReturnType::Type(*arrow_token, typebox), true))
                        }
                        
                        PathArguments::None => {
                            let new = v.clone();
                            Ok(
                                (ReturnType::Type(*arrow_token, Box::new(Type::Path(new))), false)
                            )
                        }

                        _ => {
                            Err(syn::Error::new_spanned(&segment, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
                        }
                    }

                }

                _ => {
                    if name.to_string() == "destroy" {
                        return Err(syn::Error::new_spanned(_ty, "Destroy method cannot have return type"));
                    }

                    Err(syn::Error::new_spanned(_ty, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
                }
            }
        }

        _ => {
            if impl_name.is_some() && name.to_string() == "new" {
                Err(syn::Error::new_spanned(name, "Impl new() must return `Self` type"))
            } else {
                Ok((ReturnType::Default, false))
            }
        }
    }
}

// returns the first JNIEnv ident (skips &self)
pub fn validate_fn_args(fn_args: &Punctuated<FnArg, Comma>) -> syn::Result<()> {
    let allowed_types_second_param = vec![
        "JObject", "jobject", "JClass", "jclass"
    ];
    let allowed_types = vec![
        "jobject", "jclass", "jthrowable", "jstring", "jarray", "jbooleanArray",
        "jbyteArray", "jcharArray", "jshortArray", "jintArray", "jlongArray",
        "jfloatArray", "jdoubleArray", "jobjectArray", "jweak", "jint", "jlong ",
        "jbyte ", "jboolean", "jchar", "jshort", "jfloat", "jdouble", "jsize", 
        "jfieldID", "jmethodID",
        "JByteBuffer", "JClass", "JFieldID", "JList", "JMap", "JMethodID",
        "JObject", "JStaticFieldID", "JStaticMethodID", "JString", "JThrowable",
        "JValue"
    ];

    let mut pos = 1;

    for arg in fn_args {
        match arg {
            FnArg::Typed(v) => {                
                if let syn::Type::Path(ref v) = *v.ty {
                    let seg = &v.path.segments;
                    if seg.len() == 0 {
                        return Err(syn::Error::new(Span::call_site(), "Empty segments"));
                    }

                    let ty = seg.last().unwrap();
                    if pos == 1 {
                        if ty.ident.to_string() != "JNIEnv" {
                            return Err(syn::Error::new_spanned(v, "Param must be JNIEnv"));
                        }
                    } else if pos == 2 {
                        if !allowed_types_second_param.contains(&&*ty.ident.to_string()) {
                            return Err(syn::Error::new_spanned(v, "Param must be JObject or JClass"));
                        }
                    } else {
                        if !allowed_types.contains(&&*ty.ident.to_string()) {
                            return Err(syn::Error::new_spanned(v, "Param must be a j-type"));
                        }
                    }
                }

                pos += 1;
            }

            // self arg, just ignore that
            _ => {}
        }
    }

    Ok(())
}

pub fn extract_two_params(fn_args: &Punctuated<FnArg, Comma>, name: &Ident) -> syn::Result<(Ident, Ident)> {
    let mut idents: Vec<&Ident> = vec![];

    for (i, arg) in fn_args.iter().enumerate() {
        match arg {
            FnArg::Typed(v) => {
                if let Pat::Ident(v) = &*v.pat {
                    idents.push(&v.ident);
                    if i == 2 {
                        break;
                    }
                }
            }

            // self arg, just ignore that
            _ => {}
        }
    }

    let tk: Box<dyn ToTokens> = if idents.len() > 0 {
        Box::new(fn_args)
    } else {
        Box::new(name)
    };

    if idents.len() < 2 {
        return Err(syn::Error::new_spanned(tk, "Missing minimum amount of required args (2; JNIEnv and JObject/JClass)"));
    }

    Ok((idents[0].clone(), idents[1].clone()))
}

pub fn fn_call(fn_args: &Punctuated<FnArg, Comma>, fn_name: &Ident) -> syn::Result<TokenStream> {
    let ident_params = get_fn_args(fn_args);
    let args = ident_params.join(", ");
    syn::parse_str(&format!("{}({})", fn_name.to_string(), args))
}

pub fn filter_out_ignored(item_impl: &mut ItemImpl) {
    item_impl.items.retain(|i| { 
        if let ImplItem::Method(m) = i {
            for attr in &m.attrs {
                let s = &attr.path.segments;
                for seg in s {
                    let ident = seg.ident.to_string();
                    if ident == "jniignore" {
                        return false;
                    }
                }
            }

            return true;
        }
        
        return true;
    });
}

pub fn validate_impl_args(items: &Vec<ImplItem>) -> syn::Result<()> {
    for item in items {
        if let ImplItem::Method(m) = item {
            validate_fn_args(&m.sig.inputs)?;
        }
    }

    Ok(())
}

pub fn extract_impl_name(self_type: &Type) -> syn::Result<Ident> {
    let type_ident: &Ident;
    match self_type {
        Type::Path(t) => {
            let segments = &t.path.segments;
            if segments.len() == 0 {
                return Err(syn::Error::new_spanned(self_type, "Segments empty"))
            }

            let last = t.path.segments.last().unwrap();
            type_ident = &last.ident;
        }

        _ => return Err(syn::Error::new_spanned(self_type, "Missed match")),
    }

    Ok(type_ident.to_owned())
}

pub fn validate_impl_returns(items: &Vec<ImplItem>, name: &Ident) -> syn::Result<Vec<(ReturnType, bool)>> {
    let mut impl_returns = vec![];
    for item in items {
        if let ImplItem::Method(m) = item {
            impl_returns.push(extract_return(&m.sig.output, &m.sig.ident, Some(name))?);
        }
    }

    Ok(impl_returns)
}

pub fn impl_extract_two_params(items: &Vec<ImplItem>) -> syn::Result<Vec<(Ident, Ident)>> {
    let mut impl_idents = vec![];

    for item in items {
        if let ImplItem::Method(m) = item {
            let two_params = extract_two_params(&m.sig.inputs, &m.sig.ident)?;
            impl_idents.push(two_params);
        }
    }

    Ok(impl_idents)
}

pub fn get_fn_args(input: &Punctuated<FnArg, Comma>) -> Vec<String> {
    let mut ident_params: Vec<String> = vec![];

    for arg in input {
        match arg {
            FnArg::Typed(v) => {
                if let Pat::Ident(v) = &*v.pat {
                    ident_params.push(v.ident.to_string());
                }
            }

            // self arg, just ignore that
            _ => {}
        }
    }

    ident_params
}

pub fn impl_fn_args(input: &Punctuated<FnArg, Comma>) -> Punctuated<FnArg, Comma> {
    let mut new_punc: Punctuated<FnArg, Comma> = Punctuated::new();

    for arg in input {
        match arg {
            FnArg::Typed(_) => {
                new_punc.push(arg.clone())
            }

            // ignore self param
            FnArg::Receiver(_) => ()
        }
    }

    new_punc
}

pub fn impl_is_fn_mut(input: &Punctuated<FnArg, Comma>) -> bool {
    for arg in input {
        match arg {
            FnArg::Typed(_) => (),

            // self param
            FnArg::Receiver(v) => {
                return v.mutability.is_some()
            }
        }
    }

    false
}

pub fn generate_impl_functions(
    items: &Vec<ImplItem>,
    returns: &Vec<(ReturnType, bool)>,
    env_idents: &Vec<(Ident, Ident)>,
    namespace: (&String, bool, &Ident),
    exc: &str,
    handler_trait: Option<&String>
) -> syn::Result<Vec<TokenStream>> {
    let mut funcs: Vec<TokenStream> = vec![];

    for (i, _fn) in items.iter().enumerate() {
        match _fn {
            ImplItem::Method(m) => {
                let fn_name = &m.sig.ident;
                let env_ident = &env_idents[i].0;
                let second_ident = &env_idents[i].1;
                let ret_type = &returns[i].0;
                let is_result = returns[i].1;

                let fn_inputs = impl_fn_args(&m.sig.inputs);
                let fn_is_mut = impl_is_fn_mut(&m.sig.inputs);

                let fn_call = fn_call(&m.sig.inputs, fn_name)?;

                let ns = namespace.0;
                let is_pkg = namespace.1;
                let impl_name = namespace.2;
                let impl_name_str = impl_name.to_string();

                let diag = format!("{}::{}()", impl_name_str, fn_name);

                let is_returning = match ret_type {
                    ReturnType::Default => false,
                    ReturnType::Type(_, _) => true
                };

                let class = if is_pkg {
                    format!("{}_{}", ns, impl_name)
                } else {
                    ns.to_owned()
                };

                let java_name = class_to_ident(&class, &fn_name.to_string());

                let handler_trait: TokenStream = match handler_trait {
                    Some(v) => syn::parse_str(v),
                    None => syn::parse_str("crate::env::Utils")
                }?;

                // special case for new fn
                let stream: TokenStream;
                if fn_name == "new" {

                    stream = quote! {
                        #[no_mangle]
                        pub extern "C" fn #java_name(#fn_inputs) {
                            use #handler_trait;

                            let panic_res = ::std::panic::catch_unwind(|| {
                                let r_obj = #impl_name::#fn_call;
                                let res = #env_ident.set_handle(#class, #second_ident, r_obj);

                                match res {
                                    Ok(_) => (),
                                    Err(e) => {
                                        #env_ident.throw_new(#exc, format!("Failed to set handle for `{}` : {}", #diag, e.to_string())).ok();
                                    }
                                }
                            });

                            match panic_res {
                                Ok(_) => (),
                                Err(e) => {
                                    #env_ident.throw_new("java/lang/RuntimeException", &format!("`{}` panicked", #diag)).ok();
                                }
                            }
                        }
                    };
                } else if fn_name == "destroy" {

                    let mut_kwrd = if fn_is_mut {
                        quote! { mut }
                    } else {
                        TokenStream::new()
                    };

                    stream = quote! {
                        #[no_mangle]
                        pub extern "C" fn #java_name(#fn_inputs) {
                            use #handler_trait;

                            let panic_res = ::std::panic::catch_unwind(|| {
                                let res = #env_ident.take_handle::<#impl_name>(#class, #second_ident);

                                let #mut_kwrd r_obj = match res {
                                    Ok(v) => v,
                                    Err(e) => {
                                        #env_ident.throw_new(#exc, format!("Failed to take handle for `{}` : {}", #diag, e.to_string())).ok();
                                        return;
                                    }
                                };

                                r_obj.#fn_call;
                            });

                            match panic_res {
                                Ok(_) => (),
                                Err(e) => {
                                    #env_ident.throw_new("java/lang/RuntimeException", &format!("`{}` panicked", #diag)).ok();
                                }
                            }
                        }
                    };

                } else {
                    
                    let ret_no_result = if is_returning && !is_result {
                        quote! { return }
                    } else {
                        TokenStream::new()
                    };

                    let null_mut = if is_returning {
                        quote! {
                            ::std::ptr::null_mut()
                        }
                    } else {
                        TokenStream::new()
                    };

                    let ok_or_null = if is_returning {
                        quote! {
                            ::std::ptr::null_mut()
                        }
                    } else {
                        quote! {
                            ()
                        }
                    };

                    let mut_kwrd = if fn_is_mut {
                        quote! { mut }
                    } else {
                        TokenStream::new()
                    };

                    //
                    // matching for result types
                    //
                    let res_binding = if is_result {
                        quote! {
                            let c_res =
                        }
                    } else {
                        TokenStream::new()
                    };

                    let match_res = if is_result {
                        quote! {
                            match c_res {
                                Ok(v) => return v,
                                Err(e) => {
                                    #env_ident.throw_new(#exc, format!("`{}` threw an exception : {}", #diag, e.to_string())).ok();
                                    return ::std::ptr::null_mut();
                                }
                            }
                        }
                    } else {
                        TokenStream::new()
                    };
                    //
                    // end matching for result types

                    stream = quote! {
                        #[no_mangle]
                        pub extern "C" fn #java_name(#fn_inputs) #ret_type {
                            use #handler_trait;

                            let panic_res = ::std::panic::catch_unwind(|| {
                                let res = #env_ident.get_handle::<#impl_name>(#class, #second_ident);

                                let #mut_kwrd r_obj = match res {
                                    Ok(v) => v,
                                    Err(e) => {
                                        #env_ident.throw_new(#exc, format!("Failed to get handle for `{}` : {}", #diag, e.to_string())).ok();
                                        return #null_mut;
                                    }
                                };

                                #res_binding #ret_no_result r_obj.#fn_call;

                                #match_res
                            });

                            match panic_res {
                                Ok(_) => #ok_or_null,
                                Err(e) => {
                                    #env_ident.throw_new("java/lang/RuntimeException", &format!("`{}` panicked", #diag)).ok();
                                    #null_mut
                                }
                            }
                        }
                    };
                }

                funcs.push(stream);
            }

            _ => return Err(syn::Error::new_spanned(_fn, "Illegal type"))
        }
    }

    Ok(funcs)
}
