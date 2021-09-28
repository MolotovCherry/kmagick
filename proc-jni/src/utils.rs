use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{FnArg, ImplItem, ItemImpl, Meta, NestedMeta, Pat, PathArguments, ReturnType, Type};

pub fn get_args(args: Vec<NestedMeta>) -> syn::Result<HashMap<String, String>> {
    let mut hm: HashMap<String, String> = HashMap::new();

    let allowed_args = vec!["cls", "exc", "pkg"];

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
pub fn extract_return(ret: &ReturnType) -> syn::Result<(ReturnType, bool)> {
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
                    if ident.to_string() != "Result" && !allowed_ret.contains(&&*ident.to_string()) {
                        return Err(syn::Error::new_spanned(ident, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
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

                _ => Err(syn::Error::new_spanned(_ty, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
            }
        }

        _ => Ok((ReturnType::Default, false))
    }
}

// returns the first JNIEnv ident (skips &self)
pub fn validate_fn_args(fn_args: &Punctuated<FnArg, Comma>, name: &Ident) -> syn::Result<Ident> {
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
    let mut ident: Ident = name.clone();

    if fn_args.len() < 2 {
        let span = if fn_args.len() == 0 {
            name.span()
        } else {
            fn_args.span()
        };

        return Err(syn::Error::new(span, "Missing minimum required number of params (2)"));
    }

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

                if let Pat::Ident(v) = &*v.pat {
                    if pos == 1 {
                        ident = v.ident.clone();
                    }
                } else {
                    return Err(syn::Error::new_spanned(&v.pat, "Not an ident"));
                }

                pos += 1;
            }

            // self arg, just ignore that
            _ => {}
        }
    }

    Ok(ident)
}

pub fn fn_call(fn_args: &Punctuated<FnArg, Comma>, fn_name: &Ident) -> syn::Result<TokenStream> {
    let mut ident_params: Vec<String> = vec![];

    for arg in fn_args {
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

    let args = ident_params.join(", ");
    syn::parse_str(&format!("{}({});", fn_name.to_string(), args))
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

pub fn validate_impl_args(items: &Vec<ImplItem>) -> syn::Result<Vec<Ident>> {
    let mut env_idents = vec![];
    for item in items {
        if let ImplItem::Method(m) = item {
            env_idents.push(validate_fn_args(&m.sig.inputs, &m.sig.ident)?);
        }
    }

    Ok(env_idents)
}

pub fn validate_impl_returns(items: &Vec<ImplItem>) -> syn::Result<Vec<(ReturnType, bool)>> {
    let mut impl_returns = vec![];
    for item in items {
        if let ImplItem::Method(m) = item {
            impl_returns.push(extract_return(&m.sig.output)?);
        }
    }

    Ok(impl_returns)
}
