use std::iter::FromIterator;

use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use syn::{Attribute, Error, FnArg, Pat, PatType, ReturnType, Token, Type, parenthesized, parse::ParseStream, punctuated::Punctuated, spanned::Spanned, token::Comma};

use quote::{ToTokens, TokenStreamExt, quote};
use crate::parser::Binding;

use super::parser::ItemBinding;


// This function was not public in the API, so I had to rip it out
pub fn parse_fn_args(input: ParseStream) -> syn::Result<Punctuated<FnArg, Token![,]>> {
    let mut args = Punctuated::new();
    let mut has_receiver = false;

    // strip out the parenthesis and only use inner input
    let content;
    parenthesized!(content in input);
    let input = content;

    while !input.is_empty() {
        let attrs = input.call(Attribute::parse_outer)?;

        let arg = if let Some(dots) = input.parse::<Option<Token![...]>>()? {
            FnArg::Typed(PatType {
                attrs,
                pat: Box::new(Pat::Verbatim(variadic_to_tokens(&dots))),
                colon_token: Token![:](dots.spans[0]),
                ty: Box::new(Type::Verbatim(variadic_to_tokens(&dots))),
            })
        } else {
            let mut arg: FnArg = input.parse()?;
            match &mut arg {
                FnArg::Receiver(receiver) if has_receiver => {
                    return Err(Error::new(
                        receiver.self_token.span,
                        "unexpected second method receiver",
                    ));
                }
                FnArg::Receiver(receiver) if !args.is_empty() => {
                    return Err(Error::new(
                        receiver.self_token.span,
                        "unexpected method receiver",
                    ));
                }
                FnArg::Receiver(receiver) => {
                    has_receiver = true;
                    receiver.attrs = attrs;
                }
                FnArg::Typed(arg) => arg.attrs = attrs,
            }
            arg
        };
        args.push_value(arg);

        if input.is_empty() {
            break;
        }

        let comma: Token![,] = input.parse()?;
        args.push_punct(comma);
    }

    Ok(args)
}

// This function was not public in the API, so I had to rip it out
fn variadic_to_tokens(dots: &Token![...]) -> TokenStream {
    TokenStream::from_iter(vec![
        TokenTree::Punct({
            let mut dot = Punct::new('.', Spacing::Joint);
            dot.set_span(dots.spans[0]);
            dot
        }),
        TokenTree::Punct({
            let mut dot = Punct::new('.', Spacing::Joint);
            dot.set_span(dots.spans[1]);
            dot
        }),
        TokenTree::Punct({
            let mut dot = Punct::new('.', Spacing::Alone);
            dot.set_span(dots.spans[2]);
            dot
        }),
    ])
}


fn extract_args(
    fn_args: Punctuated<FnArg, Comma>
) -> syn::Result<Vec<(String, String)>> {
    let mut vec_fn_args: Vec<(String, String)> = vec![];
    for fn_arg in fn_args {
        match fn_arg {
            FnArg::Typed(v) => {
                let name: String;
                let ty: String;

                match &*v.pat {
                    Pat::Ident(n) => {
                        name = n.ident.to_string();
                    }

                    Pat::Wild(w) => return Err(syn::Error::new_spanned(w, "Wildcards not supported")),

                    _ => return Err(syn::Error::new_spanned(v, "Unsupported type"))
                }

                match &*v.ty {
                    Type::Path(p) => {
                        ty = p.path.segments.to_token_stream().to_string().replace(" ", "");
                    }

                    Type::Reference(r) => {
                        ty = r.to_token_stream().to_string().replace(" ", "");
                    }

                    Type::Group(g) => {
                        ty = g.to_token_stream().to_string().replace(" ", "");
                    }

                    _ => return Err(syn::Error::new_spanned(&v.ty, "Unsupported type"))
                }

                vec_fn_args.push((name, ty));
            }

            _ => return Err(syn::Error::new(fn_arg.span(), "Self arg unsupported"))
        }
    }

    Ok(vec_fn_args)
}

// name of type (inner type if Result<type>), is_result, is_return
fn get_result_type(ret: ReturnType) -> syn::Result<(String, bool, bool)> {
    match ret {
        ReturnType::Default => {
            return Ok((String::from(""), false, false))
        }

        ReturnType::Type(_, t) => {
            match *t {
                Type::Reference(r) => {
                    return Ok((r.to_token_stream().to_string().replace(" ", ""), false, true));
                }

                Type::Path(p) => {
                    let segment = p.path.segments.last().unwrap();
                    if segment.ident.to_string() != "Result" {
                        let ty_str = p.path.segments.to_token_stream().to_string().replace(" ", "");
                        return Ok((ty_str, false, true))
                    }

                    let args = &p.path.segments.last().unwrap().arguments;

                    match args {
                        syn::PathArguments::AngleBracketed(r) => {
                            let r_args = &r.args;
                            if r_args.len() == 0 {
                                return Err(syn::Error::new_spanned(args, "Angle brackets <> need a type inside"))
                            }

                            match &r_args[0] {
                                syn::GenericArgument::Type(t) => {
                                    match t {
                                        Type::Path(p) => {
                                            let ty_str = p.path.segments.to_token_stream().to_string().replace(" ", "");
                                            return Ok((ty_str, true, true))
                                        }

                                        Type::Tuple(v) => {
                                            if v.elems.len() == 0 {
                                                return Ok((String::from("()"), true, true))
                                            }
                                            return Err(syn::Error::new_spanned(v, "Tuple must be empty"))
                                        }

                                        _ => return Err(syn::Error::new_spanned(r, "Wrong arg type"))
                                    }
                                },

                                _ => return Err(syn::Error::new_spanned(r, "Wrong type"))
                            }
                        },

                        _ => return Err(syn::Error::new_spanned(args, "Expected angle brackets"))
                    }
                }

                Type::Group(g) => {
                    match *g.elem {
                        Type::Path(p) => {
                            let ty_str = p.path.segments.to_token_stream().to_string().replace(" ", "");
                            return Ok((ty_str, false, true))
                        }

                        _ => return Err(syn::Error::new_spanned(g, "Unsupported group type"))
                    }
                }

                _ => return Err(syn::Error::new_spanned(*t, "Illegal return type"))
            }
        }
    }
}


pub fn process_functions(items: ItemBinding) -> syn::Result<TokenStream> {
    let mut tk = TokenStream::new();

    for item in items.items {
        tk.append_all(
            create_function(item)?
        );
    }

    let impl_name = items.impl_token;
    let exc_name = format!("com/cherryleafroad/kmagick/{}Exception", impl_name.to_string());

    Ok(quote! {
        #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc=#exc_name)]
        impl #impl_name {
            #tk
        }
    })
}

pub fn create_function(item: Binding) -> syn::Result<TokenStream> {
    // get list of extracted args
    let extracted = extract_args(item.binding_fn.fn_args)?;
    // get result type + information ; (type, is_result, is_return)
    let (mut result_type, mut is_result, is_return) = get_result_type(item.binding_fn.ret)?;
    let fn_name = item.binding_fn.name;
    let orig_fn_name = item.fn_name;

    let self_kwrd = if item.mutable {
        quote! {
            &mut self,
        }
    } else {
        quote! {
            &self,
        }
    };

    let mut fn_call = TokenStream::new();

    // start off with default args
    let mut main_fn_args = quote! {
        env: jni::JNIEnv, obj: jni::objects::JObject
    };

    let mut binding_fn_args = TokenStream::new();

    let mut setup_code = TokenStream::new();

    // add a forced Ok(()) return at the end
    let mut force_result = false;
    // whether this force result requries a match (suppresses Ok(()))
    let mut force_result_match = false;


    // process pre-setup, params, and procesing code
    let mut string_setup = quote! {
        use jni_tools::Utils;
    };
    let mut handle_setup = quote! {
        use jni_tools::Handle;
    };
    for (name, ty) in extracted {
        let name = Ident::new(&*name, Span::call_site());

        // conversions from jni to native methods
        // shadowing is easier :)
        match &*ty {
            "&str" => {
                force_result = true;
                force_result_match = true;
                result_type = String::from("()");
                is_result = true;

                setup_code.append_all(quote! {
                    #string_setup
                    let #name = &*env.get_jstring(#name)?;
                });

                main_fn_args.append_all(quote! {
                    , #name: jni::objects::JString
                });

                // erase the previous code so we bring headers in only once
                string_setup = TokenStream::new();
            }

            "PixelWand" => {
                force_result = true;
                result_type = String::from("()");
                is_result = true;

                setup_code.append_all(quote! {
                    #handle_setup
                    let r_obj = env.get_handle::<crate::pixel_wand::PixelWand>(obj)?;
                    let #name = &r_obj.instance;
                });

                handle_setup = TokenStream::new();
            }

            // size_t = usize
            // jint == i32
            // conversions fit inside, however, the other way around...
            "size_t" => {
                setup_code.append_all(quote! {
                    let #name = #name as usize;
                });

                main_fn_args.append_all(quote! {
                    , #name: jni::sys::jint
                });
            },

            // ssize_t = isize
            // jsize == jint == i32
            // conversions fit inside, the other way though...
            "ssize_t" => {
                setup_code.append_all(quote! {
                    let #name = #name as isize;
                });

                main_fn_args.append_all(quote! {
                    , #name: jni::sys::jint
                });
            },

            // jfloat == f32 == Quantum == MagickFloatType
            "Quantum" => {
                main_fn_args.append_all(quote! {
                    , #name: jni::sys::jfloat
                });
            }

            // jdouble == f64
            "f64" => {
                main_fn_args.append_all(quote! {
                    , #name: jni::sys::jdouble
                });
            },

            // Any other custom type not represented will be treated as an object
            // and these custom types == c_int == jint == i32
            _ => {
                main_fn_args.append_all(quote! {
                    , #name: jni::sys::jint
                });
            }
        };


        binding_fn_args.append_all(quote! {
            #name,
        });
    }

    // process the function call
    if !is_return {
        fn_call.append_all(quote! {
            self.#fn_name(#binding_fn_args);
        });
    } else {
        let tk: TokenStream;

        if is_result {
            match &*result_type {
                "()" => {
                    tk = quote! {
                        Ok(self.#fn_name(#binding_fn_args)?)
                    };
                }

                "String" => {
                    // if a null pointer was returned, forward it to jni
                    let msg = format!("null ptr returned by {}", fn_name);
                    tk = quote! {
                        let res = match self.#fn_name(#binding_fn_args) {
                            Ok(v) => v,
                            Err(e) => {
                                return if e.starts_with(#msg) {
                                    Ok(std::ptr::null_mut())
                                } else {
                                    Err(
                                        Box::new(
                                            crate::utils::JNIError::RuntimeException(
                                                String::from(e)
                                            )
                                        )
                                    )
                                };
                            }
                        };
                    };
                }

                _ => {
                    tk = quote! {
                        let res = self.#fn_name(#binding_fn_args)?;
                    };
                }
            }
        } else {
            tk = quote!{
                let res = self.#fn_name(#binding_fn_args);
            };
        }

        fn_call.append_all(tk);
    }

    // handle post setup and return sig conversion if it is required
    // ignore "()" since there are no results for it

    let fn_code: TokenStream;
    let mut ret_result = TokenStream::new();

    if is_return && result_type != "()" {
        let ret_type: TokenStream;
        let mut last_stmt: TokenStream;
        let mut post_setup = TokenStream::new();

        match &*result_type {
            "String" => {
                ret_type = quote! {
                    jni::sys::jobject
                };

                last_stmt = quote! {
                    env.new_string(&*res)?.into_inner()
                };
            }

            // values should fit into it.. if not, then there's a bigger problem
            "size_t" | "ssize_t" => {
                is_result = true;

                ret_type = quote! {
                    jni::sys::jsize
                };

                post_setup = quote! {
                    use std::convert::TryFrom;
                };

                last_stmt = quote! {
                    i32::try_from(res)?
                };
            }

            "PixelWand" => {
                is_result = true;

                ret_type = quote! {
                    jni::sys::jobject
                };

                let pixel_wand_cls = "com/cherryleafroad/kmagick/PixelWand";
                let pixel_wand_com = format!("{}$Companion", pixel_wand_cls);
                let pixel_wand_obj = format!("L{};", pixel_wand_cls);
                let pixel_wand_sig = format!("(){}", pixel_wand_obj);
                post_setup.append_all(quote! {
                    #handle_setup
                    let cls = env.find_class(#pixel_wand_com)?;
                    let c_obj = env.new_object(cls, "()V", &[])?;
                    let mid = env.get_method_id(cls, "newInstance", #pixel_wand_sig)?;
                    let n_obj = env.call_method_unchecked(c_obj, mid, jni::signature::JavaType::Object(#pixel_wand_obj.into()), &[])?.l()?;

                    let r_obj = crate::pixel_wand::PixelWand {
                        instance: res
                    };

                    env.set_handle(n_obj, r_obj)?;
                });

                last_stmt = quote! {
                    n_obj.into_inner()
                };
            }

            "Quantum" => {
                ret_type = quote! {
                    jni::sys::jfloat
                };

                // quantum == f32 == jfloat
                last_stmt = quote! {
                    res
                };
            }

            "f64" => {
                ret_type = quote! {
                    jni::sys::jdouble
                };

                // f64 == jdouble
                last_stmt = quote! {
                    res
                };
            }

            // Any other custom type not represented will be treated as an object
            // and these custom types == c_int == jint == i32
            _ => {
                // force a result type since we're using ? below
                is_result = true;

                ret_type = quote! {
                    jni::sys::jobject
                };

                let class = format!("com/cherryleafroad/kmagick/{}$Companion", result_type);
                let mid_sig = format!("(I)Lcom/cherryleafroad/kmagick/{};", result_type);
                let res_type = format!("Lcom/cherryleafroad/kmagick/{};", result_type);
                post_setup.append_all(quote! {
                    let val = jni::objects::JValue::Int(res);
                    let cls = env.find_class(#class)?;
                    let j_obj = env.new_object(cls, "()V", &[])?;
                    let mid = env.get_method_id(cls, "fromNative", #mid_sig)?;
                });

                last_stmt = quote! {
                    env.call_method_unchecked(j_obj, mid, jni::signature::JavaType::Object(#res_type.into()), &[val])?.l()?.into_inner()
                };
            }
        };

        if is_result {
            last_stmt = quote! {
                Ok(#last_stmt)
            };

            ret_result = quote! {
                -> crate::utils::Result<#ret_type>
            };
        } else {
            ret_result = quote! {
                -> #ret_type
            };
        }

        fn_code = quote! {
            fn #orig_fn_name(#self_kwrd #main_fn_args) #ret_result {
                #setup_code

                #fn_call

                #post_setup

                #last_stmt
            }
        };
    } else {
        if result_type == "()" {
            ret_result = quote! {
                -> crate::utils::Result<()>
            };
        }

        let empty_ending = if is_result && force_result && !force_result_match {
            quote! {
                Ok(())
            }
        } else {
            quote!{}
        };

        fn_code = quote! {
            fn #orig_fn_name(#self_kwrd #main_fn_args) #ret_result {
                #setup_code

                #fn_call

                #empty_ending
            }
        };
    }

    Ok(fn_code)
}
