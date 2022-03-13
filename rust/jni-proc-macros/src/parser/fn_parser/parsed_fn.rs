use std::collections::HashSet;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
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
    pub vis: Visibility,
    pub attrs: HashSet<ParsedAttr>,
    /// these args are from the actual fn. we must adhere to sending these exact one's over
    /// (argname, argname_span)
    pub fn_args: Vec<(TokenStream, Span)>,
    /// these args are what we must use to generate the binding function's header
    /// (argname, type)
    pub binding_fn_args: Vec<(TokenStream, TokenStream)>,
    pub self_is_mut: bool,
    pub has_self: bool,
    pub is_impl_fn: bool,
    pub is_empty: bool,
    /// return type is a result type or not?
    pub is_result: bool,
    pub is_returning: bool,
    result_type: String,
    pub null_ret_type: TokenStream,
    method: MethodType,
    /// the raw returntype of the function
    pub ret_type: ReturnType
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
    pub fn parse_fn(input: &proc_macro::TokenStream) -> syn::Result<Option<Self>> {
        let item_fn = syn::parse::<ItemFn>(input.clone())?;
        Self::parse(
            &item_fn,
            MethodType::ItemFn(item_fn),
            None
        )
    }

    pub fn parse_impl_fn(input: ImplItemMethod, impl_name: Ident) -> syn::Result<Option<Self>> {
        Self::parse(
            &input,
            MethodType::ImplItemMethod(input),
            Some(impl_name)
        )
    }

    /// returns None if annotated with jignore, it's a no-op
    fn parse<T: GenericFn>(item_fn: &T, method: MethodType, impl_name: Option<Ident>) -> syn::Result<Option<Self>> {
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
        } else if attrs.contains("jstatic") {
            is_static = Some(true);
        } else if attrs.contains("jname") {
            bind_name = Some(
                Ident::new(
                    &*attrs.get_attr_key_s("jname", "name").unwrap(),
                    item_fn.sig().ident.span()
                )
            );
        }

        // use normal name if jname is missing
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
        let (fn_args, has_self, self_is_mut) = fn_arg_parser(item_fn.sig().inputs.into_iter().collect())?;

        //
        // validate input types are correct
        //
        validate_types(
            fn_args.iter().map(|(_, (ty, _))| ty.clone()).collect(),
            item_fn.is_impl(),
            is_static.is_some()
        )?;


        // now we must validate and potentially fix the differences in calling the fn's
        // we are allowed to have 0 arguments to the impl/method fns (but if we do have args,
        // the first ones MUST be env and jniobject/jclass (validated above))

        // (name, type)
        let mut binding_fn_args: Vec<(TokenStream, TokenStream)> = fn_args.iter().map(|((name, _), (ty, _))| (name.clone(), ty.clone())).collect();
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


        //
        // Return type processing
        //
        let (result_type, is_result, is_returning, raw_return) = parse_return(item_fn.output(), impl_name, &attrs)?;
        let result_type = result_type.unwrap_or("".to_string());
        let null_ret_type = Self::get_null_ret_type(&result_type);
        //  End return type processing
        //

        let vis = item_fn.vis().clone();

        let is_empty = item_fn.block().stmts.is_empty();

        Ok(Some(Self {
            bind_name,
            orig_name,
            vis,
            attrs,
            fn_args: fn_args.iter().map(|a| (a.0.0.clone(), a.0.1)).collect(),
            binding_fn_args,
            self_is_mut,
            has_self,
            is_impl_fn: item_fn.is_impl(),
            is_empty,
            result_type,
            is_result,
            is_returning,
            null_ret_type,
            method,
            ret_type: raw_return
        }))
    }

    pub fn call_attr<F>(&self, name: &str, f: F)
        where F: FnMut(&ParsedAttr)
    {
        if self.attrs.contains(name) {
            f(self.attrs.get(name).unwrap());
        }
    }

    /// get the args for the generated jni binding function
    /// to fill the function header with
    pub fn get_binding_args(&self) -> TokenStream {
        let mut tk = TokenStream::new();

        for (name, ty) in self.binding_fn_args {
            tk.extend(
                quote![
                    #name: ty,
                ]
            );
        }

        tk
    }

    /// get the args to call the internal function with
    /// these args may not always line up with the binding args
    pub fn get_calling_args(&self) -> TokenStream {
        let mut tk = TokenStream::new();

        for (name, _) in self.fn_args {
            tk.extend(
                quote![
                    #name,
                ]
            );
        }

        tk
    }

    pub fn is_empty_fn(&self) -> bool {
        self.is_empty
    }

    pub fn is_impl_fn(&self) -> bool {
        self.is_impl_fn
    }

    /// check if self is &self mut
    pub fn is_mut(&self) -> bool {
        self.self_is_mut
    }

    pub fn has_self(&self) -> bool {
        self.has_self
    }

    /// check if function is static
    pub fn is_static(&self) -> bool {
        self.attrs.contains("jstatic")
    }

    /// check if function is ignore
    pub fn is_ignore(&self) -> bool {
        self.attrs.contains("jignore")
    }

    /// check if function is destroy
    pub fn is_destroy(&self) -> bool {
        self.attrs.contains("jdestroy")
    }

    /// check if function is new
    pub fn is_new(&self) -> bool {
        self.attrs.contains("jnew")
    }

    /// check if function is take
    pub fn is_take(&self) -> bool {
        self.attrs.contains("jtake")
    }

    /// check if function is set
    pub fn is_set(&self) -> bool {
        self.attrs.contains("jset")
    }

    /// check if function is get
    pub fn is_get(&self) -> bool {
        self.attrs.contains("jget")
    }

    /// check if function is target
    pub fn is_target(&self) -> bool {
        self.attrs.contains("jtarget")
    }

    /// check if function is name
    pub fn is_name(&self) -> bool {
        self.attrs.contains("jname")
    }

    fn get_null_ret_type(ret_type: &str) -> TokenStream {
        let mut tks = TokenStream::new();

        let res_type = ret_type.to_token_stream();
        match ret_type {
            // object types
            "jobject" | "jclass" | "jthrowable" | "jstring" | "jarray" |
            "jbooleanArray" | "jbyteArray" | "jcharArray" | "jshortArray" |
            "jintArray" | "jlongArray" | "jfloatArray" | "jdoubleArray" |
            "jobjectArray" | "jweak" | "jfieldID" | "jmethodID" => {
                tks.extend(quote! { std::ptr::null_mut() })
            },

            // numeric types
            "jint" | "jlong" | "jbyte" | "jboolean" | "jchar" | "jshort" |
            "jsize" => {
                tks.extend(quote! { 0 as jni::sys::#res_type })
            },

            "jfloat" | "jdouble" => {
                tks.extend(quote! { 0.0 as jni::sys::#res_type })
            },

            _ => ()
        }

        tks
    }
}
