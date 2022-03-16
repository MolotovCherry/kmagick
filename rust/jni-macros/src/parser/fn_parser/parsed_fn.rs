#![allow(unused_assignments)]

use std::collections::HashSet;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{ImplItemMethod, ItemFn, ReturnType, Visibility};
use crate::parser::{AttrGet, GenericFn, ParsedAttr};
use crate::parser::fn_parser::fn_parser::fn_arg_parser;
use crate::parser::fn_parser::validate_args::validate_types;

use super::return_parser::parse_return;


enum MethodType {
    ImplItemMethod(ImplItemMethod),
    ItemFn(ItemFn)
}

pub struct ParsedFn {
    /// name to use in generated function
    pub bind_name: Ident,
    /// original function call name
    pub orig_name: Ident,
    // binding jni call name
    pub java_binding_fn_name: TokenStream,
    pub vis: Visibility,
    pub attrs: HashSet<ParsedAttr>,
    /// these args are from the actual fn. we must adhere to sending these exact one's over
    /// (argname, argname_span)
    pub fn_args: Vec<TokenStream>,
    /// these args are what we must use to generate the binding function's header
    /// (argname, type)
    pub binding_fn_args: TokenStream,
    pub calling_fn_args: TokenStream,
    pub self_is_mut: bool,
    pub has_self: bool,
    pub is_impl_fn: bool,
    pub is_empty: bool,
    /// return type is a result type or not?
    pub is_result: bool,
    pub is_returning: bool,
    pub null_ret_type: TokenStream,
    method: MethodType,
    /// the raw returntype of the function
    pub ret_type: ReturnType,
    pub env_name: TokenStream,
    pub obj_name: TokenStream
}

impl ToTokens for ParsedFn {
    // return original function in tokens
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.method {
            MethodType::ImplItemMethod(m) => tokens.extend(m.to_token_stream()),
            MethodType::ItemFn(f) => tokens.extend(f.to_token_stream())
        }
    }
}

impl ParsedFn {
    /// returns None if annotated with jignore, it's a no-op
    pub fn parse_fn(input: &proc_macro::TokenStream, attrs: &proc_macro::TokenStream) -> syn::Result<Option<Self>> {
        let item_fn = syn::parse::<ItemFn>(input.clone())?;
        Self::parse(
            &item_fn,
            MethodType::ItemFn(item_fn.clone()),
            None,
            attrs
        )
    }

    pub fn parse_impl_fn(input: ImplItemMethod, impl_name: Ident, attrs: &proc_macro::TokenStream) -> syn::Result<Option<Self>> {
        Self::parse(
            &input,
            MethodType::ImplItemMethod(input.clone()),
            Some(impl_name),
            attrs
        )
    }

    /// returns None if annotated with jignore, it's a no-op
    fn parse<T: GenericFn>(item_fn: &T, method: MethodType, impl_name: Option<Ident>, main_attr: &proc_macro::TokenStream) -> syn::Result<Option<Self>> {
        let mut attrs: HashSet<ParsedAttr> = HashSet::new();
        for attr in item_fn.attrs() {
            attrs.insert(
                ParsedAttr::parse_attribute(
                    &attr.path.segments.last().unwrap().ident,
                    &attr
                )?
            );
        }

        // return None if this is jignore
        let mut is_static = None;
        // handle jname attribute for renaming fn
        let mut bind_name = None;
        if attrs.contains("jignore") {
            return Ok(None)
        }
        if attrs.contains("jstatic") {
            is_static = Some(true);
        }
        if attrs.contains("jname") {
            bind_name = Some(
                Ident::new(
                    &*attrs.get_attr_key_s("jname", "name").unwrap(),
                    item_fn.sig().ident.span()
                )
            );
        }

        let attr_ty = if impl_name.is_some() {
            Ident::new("jclass", Span::mixed_site())
        } else {
            Ident::new("jmethod", Span::mixed_site())
        };
        let main_attr = ParsedAttr::parse(&attr_ty, main_attr)?;

        // use normal name if jname is missing
        if let None = bind_name {
            // this is okay because attributes verified this can only exist as Some for jmethod
            // otherwise it'll be None (overwritten next)
            bind_name = main_attr.get_i("name");
        }

        // otherwise the name is the same
        if let None = bind_name {
            bind_name = Some(item_fn.sig().ident.clone());
        }
        // shadow name since it will always be Some()
        let bind_name = bind_name.unwrap();
        //  remember original name of fn for calling
        let orig_name = item_fn.sig().ident.clone();

        //
        // process args
        //
        let (fn_args, has_self, self_is_mut) = fn_arg_parser(item_fn.sig().inputs.clone().into_iter().collect())?;

        //
        // validate input types are correct
        //
        validate_types(
            fn_args.iter().map(|(_, _, ty)| ty).collect(),
            item_fn.is_impl(),
            is_static.is_some()
        )?;


        // now we must validate and potentially fix the differences in calling the fn's
        // we are allowed to have 0 arguments to the impl/method fns (but if we do have args,
        // the first ones MUST be env and jniobject/jclass (validated above))

        // (name, type)
        let mut binding_fn_args = fn_args.iter().map(|f| (f.0.clone(), f.1.clone())).collect();
        match fn_args.len() {
            // nothing, huh?
            0 => {
                binding_fn_args = vec![
                    (quote![env], quote![jni::JNIEnv]),
                    (quote![obj], quote![jni::objects::JObject])
                ];
            }

            1 => {
                binding_fn_args.push((quote![obj], quote![jni::objects::JObject]));
            }

            // all default args will be entered already
            _ => ()
        }

        // extract names, cause we will need it
        let env_name = binding_fn_args[0].0.clone();
        let obj_name = binding_fn_args[1].0.clone();

        // final procesing for the args
        let binding_fn_args = Self::get_binding_args(binding_fn_args);
        let calling_fn_args = Self::get_calling_args(fn_args.iter().map(|f| &f.0).collect());

        //
        // Return type processing
        //
        let (result_type, is_result, is_returning, raw_return) = parse_return(item_fn.output(), &impl_name, &attrs)?;
        let result_type = result_type.unwrap_or(Ident::new("_", Span::mixed_site()));
        let null_ret_type = Self::get_null_ret_type(&result_type.to_string(), result_type, is_returning);
        //  End return type processing
        //

        let vis = item_fn.vis().clone();

        let is_empty = item_fn.block().stmts.is_empty();

        // generate the official java binding name
        // TODO: unfortunately, this naming will not always hold true in jni, as it sometimes names things differently
        // there's potential for this to cause subtle bugs with jni methods seemingly "missing"
        //    cls can be on eit
        let clss = if main_attr.contains("cls") {
            // get it from the cls attribute
            main_attr.get_s("cls").unwrap()
        } else {
            format!("{}_{}", main_attr.get_s("pkg").unwrap(), impl_name.unwrap())
        };
        let class = clss.replace("/", "_").replace(".", "_").replace("\"", "");
        let java_binding_fn_name = format_ident!("Java_{}_{}", class, bind_name).to_token_stream();

        Ok(Some(Self {
            bind_name,
            orig_name,
            java_binding_fn_name,
            vis,
            attrs,
            fn_args: fn_args.iter().map(|f| f.0.clone()).collect(),
            binding_fn_args,
            calling_fn_args,
            self_is_mut,
            has_self,
            is_impl_fn: item_fn.is_impl(),
            is_empty,
            is_result,
            is_returning,
            null_ret_type,
            method,
            ret_type: raw_return,
            env_name,
            obj_name
        }))
    }

    pub fn get_attr(&self, name: &str) -> Option<&ParsedAttr>
    {
        if self.attrs.contains(name) {
            return Some(self.attrs.get(name).unwrap());
        }

        None
    }

    /// get the args for the generated jni binding function
    /// to fill the function header with
    fn get_binding_args(binding_fn_args: Vec<(TokenStream, TokenStream)>) -> TokenStream {
        let mut tk = TokenStream::new();

        for (name, ty) in binding_fn_args {
            tk.extend(
                quote![
                    #name: #ty,
                ]
            );
        }

        tk
    }

    /// get the args to call the internal function with
    /// these args may not always line up with the binding args
    fn get_calling_args(fn_args: Vec<&TokenStream>) -> TokenStream {
        let mut tk = TokenStream::new();

        for name in fn_args {
            tk.extend(
                quote![
                    #name,
                ]
            );
        }

        tk
    }

    // (null_mut, return type)
    fn get_null_ret_type(ret_type: &str, ret_ident: Ident, is_returning: bool) -> TokenStream {
        let mut tks = TokenStream::new();

        if is_returning {
            match ret_type {
                // object types
                "jobject" | "jclass" | "jthrowable" | "jstring" | "jarray" |
                "jbooleanArray" | "jbyteArray" | "jcharArray" | "jshortArray" |
                "jintArray" | "jlongArray" | "jfloatArray" | "jdoubleArray" |
                "jobjectArray" | "jweak" | "jfieldID" | "jmethodID" => {
                    tks.extend(
                        quote! { std::ptr::null_mut() }
                    )
                },

                // numeric types
                "jint" | "jlong" | "jbyte" | "jboolean" | "jchar" | "jshort" |
                "jsize" => {
                    tks.extend(
                        quote! { 0 as jni::sys::#ret_ident }
                    )
                },

                "jfloat" | "jdouble" => {
                    tks.extend(
                        quote! { 0.0 as jni::sys::#ret_ident }
                    )
                },

                _ => ()
            }
        }

        tks
    }
}
