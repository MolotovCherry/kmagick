use std::collections::HashSet;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{PathArguments, ReturnType};
use syn::spanned::Spanned;
use super::super::ParsedAttr;

/// Ok result is (return_ident, is_result, is_return, ReturnType)
pub(super) fn parse_return(ret: &ReturnType, impl_name: &Option<Ident>, attrs: &HashSet<ParsedAttr>) -> syn::Result<(Option<Ident>, bool, bool, ReturnType)> {
    let mut allowed_ret = vec![
        "jarray", "jboolean", "jbooleanArray", "jbyte", "jbyteArray",
        "jchar", "jcharArray", "jclass", "jdouble", "jdoubleArray",
        "jfieldID", "jfloat", "jfloatArray", "jint", "jintArray", "jlong",
        "jlongArray", "jmethodID", "jobject", "jobjectArray", "jshort", "jshortArray",
        "jsize", "jstring", "jthrowable", "jweak", "JNIResult"
    ];

    let is_impl = impl_name.is_some();
    let impl_ident = if is_impl {
        Some(impl_name.clone().unwrap().clone())
    } else {
        None
    };

    // efficiency
    let is_jnew = attrs.contains("jnew");
    let is_jdestroy = attrs.contains("jdestroy");


    // The only valid return types for jnew are Self and ident_name
    let impl_name = impl_name.clone().unwrap().to_string();
    if is_impl && is_jnew {
        allowed_ret = vec![
            &*impl_name,
            "Self",
            "JNIResult"
        ];
    }

    // variables for later
    #[allow(unused_assignments)]
    let mut inner_ty = None;
    #[allow(unused_assignments)]
    let mut full_ty= TokenStream::new();
    #[allow(unused_assignments)]
    let mut is_return = false;
    #[allow(unused_assignments)]
    let mut is_result = false;
    #[allow(unused_assignments)]
    let mut raw_return = ReturnType::Default;

    match &ret {
        ReturnType::Type(_, ty) => {
            let ty = &**ty;
            match ty {
                //
                // Has a return type
                //
                syn::Type::Path(v) => {
                    let segment = v.path.segments.last().unwrap();

                    // The last type on the segment stream, e.g. the JNIResult in `jni::JNIResult`
                    inner_ty = Some(&segment.ident);
                    full_ty = v.path.segments.to_token_stream();
                    is_return = true;
                    raw_return = ReturnType::Type(syn::Token![->](ty.span()), Box::new(ty.clone()));

                    if is_jdestroy {
                        return Err(syn::Error::new_spanned(v, "Destroy method cannot have return type"))
                    }

                    // basic sanity validation
                    if !allowed_ret.contains(&&*inner_ty.unwrap().to_string()) {
                        return if is_impl {
                            Err(syn::Error::new_spanned(inner_ty.unwrap(), "Return type must be a JNIResult<Self> type, `Self` type, or empty"))
                        } else {
                            Err(syn::Error::new_spanned(inner_ty.unwrap(), "Return type must be a JNIResult<> type, primitive j type (jni::sys::*), or empty"))
                        }
                    }

                    //
                    // Check if there's generics <>
                    //
                    let args = &segment.arguments;
                    match args {
                        //
                        // angled brackets <>
                        //
                        PathArguments::AngleBracketed(a) => {
                            let a_args = &a.args;
                            // Come on now, you actually need something inside them!
                            if a_args.len() == 0 {
                                return Err(syn::Error::new_spanned(args, "Angle brackets <> need a primitive j type (jni::sys::*) or ()"))
                            }

                            let _ok_res = &a_args[0];
                            // check first result argument in <>
                            match _ok_res {
                                //
                                // Normal type argument
                                //
                                syn::GenericArgument::Type(v) => {
                                    // validate it's an allowed type
                                    match v {
                                        //
                                        // Normal type argument inside <>
                                        //
                                        syn::Type::Path(i_v) => {
                                            inner_ty = Some(&i_v.path.segments.last().unwrap().ident);
                                            is_result = true;

                                            if is_jnew {
                                                if !allowed_ret.contains(&&*inner_ty.unwrap().to_string()) {
                                                    return Err(syn::Error::new_spanned(v, "Return type must be a Self type"))
                                                }
                                            } else if !allowed_ret.contains(&&*inner_ty.unwrap().to_string()) {
                                                return Err(syn::Error::new_spanned(v, "Return type must be a primitive j type (jni::sys::*) or ()"))
                                            }
                                        }

                                        //
                                        // Another empty tuple () or (type) inside the <>
                                        //
                                        syn::Type::Tuple(t) => {
                                            if is_jnew {
                                                return Err(syn::Error::new_spanned(_ok_res, "Return type must be a Self type"));
                                            }

                                            // this is an empty () ok type
                                            // so just return no return type then
                                            if t.elems.len() == 0 {
                                                is_result = true;
                                                inner_ty = None;
                                            } else {
                                                return Err(syn::Error::new_spanned(_ok_res, "Must be an empty ok () type"))
                                            }
                                        }

                                        //
                                        // Some other invalid type inside <>
                                        //
                                        _ => {
                                            if is_jnew {
                                                return Err(syn::Error::new_spanned(_ok_res, "Return type must be a Self type"));
                                            }

                                            return Err(syn::Error::new_spanned(_ok_res, "Return type in brackets must be primitive j type (jni::sys::*) or ()"))
                                        }
                                    }
                                }

                                //
                                // Some other invalid type
                                //
                                _ => {
                                    if is_jnew {
                                        return Err(syn::Error::new_spanned(args, "Return type must be a Self type"));
                                    }

                                    return Err(syn::Error::new_spanned(args, "Return type must be a primitive j type (jni::sys::*) or ()"))
                                }
                            }
                        }

                        //
                        // no args ..?
                        // this gets called for needing to return jfloat, jboolean, etc types
                        // return type was actually missing
                        //
                        PathArguments::None => {
                            // leave inner_ty at default
                            is_result = true;
                            raw_return = syn::parse2::<ReturnType>(
                                quote![-> #full_ty]
                            )?;
                        }

                        //
                        // Parenthesized generic type, wrong type
                        //
                        _ => {
                            return Err(syn::Error::new_spanned(&segment, "Return type must be a JNIResult<> type, primitive j type (jni::sys::*), or empty"))
                        }
                    }
                }

                //
                // Tuple () or (type)
                //
                syn::Type::Tuple(t) => {
                    if is_jnew {
                        return Err(syn::Error::new_spanned(ty, "Return type must be a Self type"));
                    }

                    // this is an empty () ok type
                    // so just return no return type then
                    if t.elems.len() == 0 {
                        inner_ty = None;
                        is_return = false;
                        is_result = false;
                        raw_return = ReturnType::Default;
                    } else {
                        return Err(syn::Error::new_spanned(ty, "Must be an empty ok () type"))
                    }
                }

                //
                // Generic errors for everything else
                //
                _ => {
                    if is_jdestroy {
                        return Err(syn::Error::new_spanned(ty, "Destroy method cannot have return type"));
                    }

                    return Err(syn::Error::new_spanned(ty, "Return type must be a JNIResult<> type, primitive j type (jni::sys::*), or empty"))
                }
            }
        }

        //
        // No return type
        //
        _ => {
            if is_impl && is_jnew {
                return Err(syn::Error::new_spanned(impl_ident.unwrap(), "Impl new() must return `Self` type"))
            } else {
                inner_ty = None;
                is_return = false;
                is_result = false;
                raw_return = ReturnType::Default;
            }
        }
    }

    Ok((inner_ty.map(|f| f.clone()), is_result, is_return, raw_return))
}
