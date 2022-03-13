use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::LitStr;

use crate::parser::ParsedImpl;

pub(super) fn generate_impl_functions(
    item_impl: ParsedImpl,
    exc: LitStr
) -> syn::Result<Vec<TokenStream>> {
    let mut funcs: Vec<TokenStream> = vec![];

    let impl_name = item_impl.name;

    for _fn in item_impl.functions {
        let mut target = TokenStream::new();
        _fn.call_attr("jtarget", |f| {
            target.extend(f.to_cfg_tokens());
        });

        // jget set take
        let mut get = _fn.obj_name.clone();
        if _fn.attrs.contains("jget") {
            _fn.call_attr("jget", |f| {
                let tk = f.get("from").unwrap().value().to_token_stream();
                get = tk;
            });
        }

        let mut take = _fn.obj_name.clone();
        if _fn.attrs.contains("jtake") {
            _fn.call_attr("jtake", |f| {
                let tk = f.get("from").unwrap().value().to_token_stream();
                take = tk;
            });
        }

        let mut set = _fn.obj_name.clone();
        if _fn.attrs.contains("jset") {
            _fn.call_attr("jset", |f| {
                let tk = f.get("to").unwrap().value().to_token_stream();
                set = tk;
            });
        }

        let call_args = _fn.calling_fn_args;
        let binding_args = _fn.binding_fn_args;
        let fn_name = _fn.orig_name;
        let empty_fn = _fn.is_empty;
        let attrs = _fn.attrs;
        let ret_type = _fn.ret_type;
        let is_result = _fn.is_result;
        let fn_is_mut = _fn.self_is_mut;
        let is_returning = _fn.is_returning;
        let java_name = _fn.java_binding_fn_name;
        let env = _fn.env_name;

        let diag = format!("{}::{}()", impl_name, fn_name);

        //
        // special changing syntax
        //
        let null_ret = _fn.null_ret_type;

        let v_or_underscore = if _fn.is_returning {
            quote! { v }
        } else {
            quote! { _ }
        };

        let v_or_unit = if _fn.is_returning {
            quote! { v }
        } else {
            quote! { () }
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
                    Ok(#v_or_underscore) => #v_or_unit,
                    Err(e) => {
                        let msg = format!("`{}` threw an exception : {}", #diag, e.to_string());
                        log::error!("{}", msg);
                        log::debug!("Error details: {:?}", e);
                        #env.throw_new(#exc, msg).ok();

                        #null_ret
                    }
                }
            }
        } else {
            TokenStream::new()
        };
        //
        // end matching for result types
        //

        let res_semicolon = if is_returning {
            if is_result {
                quote! { ; }
            } else {
                TokenStream::new()
            }
        } else {
            quote! { ; }
        };


        // special case for new fn
        let stream: TokenStream;
        if attrs.contains("jnew") {
            let mat_res = if is_result {
                quote! {
                    let r_mat = #impl_name::#fn_name(#call_args);
                    let r_obj = match r_mat {
                        Ok(v) => v,
                        Err(e) => {
                            let msg = format!("Failed to create new obj for `{}` : {}", #diag, e.to_string());
                            log::error!("{}", msg);
                            log::debug!("Error details: {:?}", e);
                            #env.throw_new(#exc, msg).ok();
                            return;
                        }
                    };
                }
            } else {
                quote! {
                    let r_obj = #impl_name::#fn_name(#call_args);
                }
            };

            stream = quote! {
                #target
                #[no_mangle]
                pub extern "system" fn #java_name(#binding_args) {
                    use jni_tools::Handle;

                    let p_res = std::panic::catch_unwind(|| {
                        #mat_res
                        let res = #env.set_handle(#set, r_obj);

                        match res {
                            Ok(_) => (),
                            Err(e) => {
                                let msg = format!("Failed to set handle for `{}` : {}", #diag, e.to_string());
                                log::error!("{}", msg);
                                log::debug!("Error details: {:?}", e);
                                #env.throw_new(#exc, msg).ok();
                            }
                        }
                    });

                    match p_res {
                        Ok(_) => (),
                        Err(e) => {
                            let msg = &format!("`{}` panicked", #diag);
                            log::error!("{}", msg);
                            #env.throw_new("java/lang/RuntimeException", msg).ok();
                        }
                    }
                }
            };
        } else if attrs.contains("jstatic") {

            stream = quote! {
                #target
                #[no_mangle]
                pub extern "system" fn #java_name(#binding_args) #ret_type {
                    let p_res = std::panic::catch_unwind(|| {
                        #res_binding #impl_name::#fn_name(#call_args)#res_semicolon

                        #match_res
                    });

                    match p_res {
                        Ok(#v_or_underscore) => #v_or_unit,
                        Err(e) => {
                            let msg = &format!("`{}` panicked", #diag);
                            log::error!("{}", msg);
                            #env.throw_new("java/lang/RuntimeException", msg).ok();

                            #null_ret
                        }
                    }
                }
            };

        } else if attrs.contains("jdestroy") {

            let mut_kwrd = if fn_is_mut {
                quote! { mut }
            } else {
                TokenStream::new()
            };

            let fn_call = if empty_fn {
                TokenStream::new()
            } else {
                quote! {
                    r_obj.#fn_name(#call_args);
                }
            };

            let fn_call_res_binding = if empty_fn {
                TokenStream::new()
            } else {
                quote! {
                    let #mut_kwrd r_obj =
                }
            };

            let fn_call_sem = if empty_fn {
                TokenStream::new()
            } else {
                quote! { ; }
            };

            stream = quote! {
                #target
                #[no_mangle]
                pub extern "system" fn #java_name(#binding_args) {
                    use jni_tools::Handle;

                    let p_res = std::panic::catch_unwind(|| {
                        let res = #env.clear_handle::<#impl_name>(#take);

                        #fn_call_res_binding match res {
                            Ok(v) => {
                                match v {
                                    Some(r) => r,
                                    None => {
                                        // at this point the object reference is gone,
                                        // because we cleared the cache and cleared the references,
                                        // although the destructor was still called for jvm alive objects
                                        // so a no-op is sufficient
                                        return;
                                    }
                                }
                            },
                            Err(e) => {
                                let msg = format!("Failed to clear handle for `{}` : {}", #diag, e.to_string());
                                log::error!("{}", msg);
                                log::debug!("Error details: {:?}", e);
                                #env.throw_new(#exc, msg).ok();
                                return;
                            }
                        }#fn_call_sem

                        #fn_call
                    });

                    match p_res {
                        Ok(_) => (),
                        Err(e) => {
                            let msg = &format!("`{}` panicked", #diag);
                            log::error!("{}", msg);
                            #env.throw_new("java/lang/RuntimeException", msg).ok();
                        }
                    }
                }
            };

        } else {

            let mut_kwrd = if fn_is_mut {
                quote! { mut }
            } else {
                TokenStream::new()
            };

            stream = quote! {
                #target
                #[no_mangle]
                pub extern "system" fn #java_name(#binding_args) #ret_type {
                    use jni_tools::Handle;

                    let p_res = std::panic::catch_unwind(|| {
                        let res = #env.get_handle::<#impl_name>(#get);

                        let #mut_kwrd r_obj = match res {
                            Ok(v) => v,
                            Err(e) => {
                                let msg = format!("Failed to get handle for `{}` : {}", #diag, e.to_string());
                                log::error!("{}", msg);
                                log::debug!("Error details: {:?}", e);
                                #env.throw_new(#exc, msg).ok();

                                return #null_ret;
                            }
                        };

                        #res_binding r_obj.#fn_name(#call_args)#res_semicolon

                        #match_res
                    });

                    match p_res {
                        Ok(#v_or_underscore) => #v_or_unit,
                        Err(e) => {
                            let msg = &format!("`{}` panicked", #diag);
                            log::error!("{}", msg);
                            env.throw_new("java/lang/RuntimeException", msg).ok();

                            #null_ret
                        }
                    }
                }
            };
        }

        funcs.push(stream);
    }

    Ok(funcs)
}
