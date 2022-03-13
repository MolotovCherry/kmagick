use proc_macro2::TokenStream;
use quote::ToTokens;
use rand::Rng;
use syn::{FnArg, Pat, Type};
use syn::spanned::Spanned;


const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// (((name, name_span), (ty, ty_span)), has_self, self_is_mut)
pub(super) fn fn_arg_parser(inputs: Vec<FnArg>) -> syn::Result<(Vec<(TokenStream, TokenStream)>, bool, bool)> {
    let mut fn_args = vec![];
    let mut self_is_mut = false;
    let mut has_self = false;
    for arg in inputs {
        match arg {
            // self arg ; don't push self arg, cause we only care about args after this
            FnArg::Receiver(r) => {
                has_self = true;
                self_is_mut = r.mutability.is_some();
            }

            // normal arg
            FnArg::Typed(t) => {
                let (name, ty);

                match *t.pat {
                    Pat::Ident(i) => {
                        name = i.ident.to_token_stream();
                    }

                    // when encountering an _ arg, we need a real arg name to send it in the generated fn
                    // so generate a random id for the arg
                    Pat::Wild(_) => {
                        name = (0..10)
                            .map(|_| {
                                let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                                CHARSET[idx] as char
                            })
                            .collect::<String>().to_token_stream();
                    }

                    n => return Err(syn::Error::new(n.span(), "Invalid arg name"))
                }

                match *t.ty {
                    Type::Path(p) => {
                        ty = p.path.segments.to_token_stream();
                    }

                    t => return Err(syn::Error::new(t.span(), "Invalid arg type"))
                }

                // (arg name, arg name span),
                // (arg type, arg type span)
                fn_args.push(
                    (name, ty)
                );
            }
        }
    }

    Ok((fn_args, has_self, self_is_mut))
}
