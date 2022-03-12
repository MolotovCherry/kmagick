pub(super) fn generate_impl_functions(
    items: &Vec<ImplItem>,
    returns: &Vec<(ReturnType, String, bool)>,
    namespace: (&String, bool, &Ident),
    exc: &str
) -> syn::Result<Vec<TokenStream>> {
    let mut funcs: Vec<TokenStream> = vec![];

    let second_types = impl_extract_second_types(items)?;

    let mut i = 0;
    for _fn in items.iter() {
        match _fn {
            ImplItem::Method(m) => {
                let fn_name = &m.sig.ident;
                let empty_fn = is_empty_block(&m.block);

                let binding_name = get_rename_attr(&m.sig.ident, &m.attrs)?;

                let target = get_cfg_target(&m.attrs);

                let ret_type = &returns[i].0;
                let is_result = returns[i].2;

                let attrs = top_attrs(&m.attrs);

                let fn_inputs = impl_fn_args(&m.sig.inputs)?;
                let fn_is_mut = impl_is_fn_mut(&m.sig.inputs);

                let fn_call_args = impl_fn_fill_args(&m.sig.inputs, &fn_inputs)?;

                let inputs = match &second_types[i] {
                    Some(v) => {
                        let mut tk = quote!{
                            , obj: #v
                        };

                        let res = fn_inputs.iter().map(|(v1, v2)| { quote! { , #v1: #v2 } }).collect::<Vec<TokenStream>>();

                        tk.extend(res);

                        tk
                    }

                    None => {
                        quote! {
                            , obj: jni::objects::JObject
                        }
                    }
                };

                let ns = namespace.0;
                let is_pkg = namespace.1;
                let impl_name = namespace.2;
                let impl_name_str = impl_name.to_string();

                let diag = format!("{}::{}()", impl_name_str, fn_name);

                let is_returning = match ret_type {
                    ReturnType::Default => false,
                    ReturnType::Type(..) => true
                };

                let class = if is_pkg {
                    format!("{}_{}", ns, impl_name)
                } else {
                    ns.to_owned()
                };

                let get_set_take = get_set_take_attrs(&m.attrs);
                let set_varname = if get_set_take.1.is_some() {
                    get_set_take.1.unwrap().parse::<TokenStream>()?
                } else {
                    quote! { obj }
                };

                let get_varname = if get_set_take.0.is_some() {
                    get_set_take.0.unwrap().parse::<TokenStream>()?
                } else {
                    quote! { obj }
                };

                let take_varname = if get_set_take.2.is_some() {
                    get_set_take.2.unwrap().parse::<TokenStream>()?
                } else {
                    quote! { obj }
                };

                let java_name = class_to_ident(&class, &binding_name);

                //
                // special changing syntax
                //
                let null_ret = if is_returning {
                    get_null_return_obj(&returns[i].1)
                } else {
                    TokenStream::new()
                };


                //
                //

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
                                env.throw_new(#exc, msg).ok();

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
                if attrs.contains(&"jnew".to_string()) {

                    let mat_res = if is_result {
                        quote! {
                            let r_mat = #impl_name::#fn_name(#fn_call_args);
                            let r_obj = match r_mat {
                                Ok(v) => v,
                                Err(e) => {
                                    let msg = format!("Failed to create new obj for `{}` : {}", #diag, e.to_string());
                                    log::error!("{}", msg);
                                    log::debug!("Error details: {:?}", e);
                                    env.throw_new(#exc, msg).ok();
                                    return;
                                }
                            };
                        }
                    } else {
                        quote! {
                            let r_obj = #impl_name::#fn_name(#fn_call_args);
                        }
                    };

                    stream = quote! {
                        #target
                        #[no_mangle]
                        pub extern "system" fn #java_name(env: jni::JNIEnv #inputs) {
                            use jni_tools::Handle;

                            let p_res = std::panic::catch_unwind(|| {
                                #mat_res
                                let res = env.set_handle(#set_varname, r_obj);

                                match res {
                                    Ok(_) => (),
                                    Err(e) => {
                                        let msg = format!("Failed to set handle for `{}` : {}", #diag, e.to_string());
                                        log::error!("{}", msg);
                                        log::debug!("Error details: {:?}", e);
                                        env.throw_new(#exc, msg).ok();
                                    }
                                }
                            });

                            match p_res {
                                Ok(_) => (),
                                Err(e) => {
                                    let msg = &format!("`{}` panicked", #diag);
                                    log::error!("{}", msg);
                                    env.throw_new("java/lang/RuntimeException", msg).ok();
                                }
                            }
                        }
                    };
                } else if attrs.contains(&"jstatic".to_string()) {

                    stream = quote! {
                        #target
                        #[no_mangle]
                        pub extern "system" fn #java_name(env: jni::JNIEnv #inputs) #ret_type {
                            let p_res = std::panic::catch_unwind(|| {
                                #res_binding #impl_name::#fn_name(#fn_call_args)#res_semicolon

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

                } else if attrs.contains(&"jdestroy".to_string()) {

                    let mut_kwrd = if fn_is_mut {
                        quote! { mut }
                    } else {
                        TokenStream::new()
                    };

                    let fn_call = if empty_fn {
                        TokenStream::new()
                    } else {
                        quote! {
                            r_obj.#fn_name(#fn_call_args);
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
                        pub extern "system" fn #java_name(env: jni::JNIEnv #inputs) {
                            use jni_tools::Handle;

                            let p_res = std::panic::catch_unwind(|| {
                                let res = env.clear_handle::<#impl_name>(#take_varname);

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
                                        env.throw_new(#exc, msg).ok();
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
                                    env.throw_new("java/lang/RuntimeException", msg).ok();
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
                        pub extern "system" fn #java_name(env: jni::JNIEnv #inputs) #ret_type {
                            use jni_tools::Handle;

                            let p_res = std::panic::catch_unwind(|| {
                                let res = env.get_handle::<#impl_name>(#get_varname);

                                let #mut_kwrd r_obj = match res {
                                    Ok(v) => v,
                                    Err(e) => {
                                        let msg = format!("Failed to get handle for `{}` : {}", #diag, e.to_string());
                                        log::error!("{}", msg);
                                        log::debug!("Error details: {:?}", e);
                                        env.throw_new(#exc, msg).ok();

                                        return #null_ret;
                                    }
                                };

                                #res_binding r_obj.#fn_name(#fn_call_args)#res_semicolon

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
                i += 1;
            }

            _ => ()//return Err(syn::Error::new_spanned(_fn, "Illegal type"))
        }
    }

    Ok(funcs)
}
