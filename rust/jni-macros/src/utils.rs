use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, Block, FnArg, ImplItem, ItemImpl, Meta, NestedMeta, Pat, PathArguments, ReturnType, Type};
use quote::{ToTokens, format_ident, quote};
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn get_args(args: Vec<NestedMeta>) -> syn::Result<HashMap<String, String>> {
    let mut hm: HashMap<String, String> = HashMap::new();

    let allowed_args = vec!["cls", "exc", "pkg", "name"];
    let ignored_args = vec!["target_os"];

    if args.is_empty() {
        return Err(syn::Error::new(proc_macro2::Span::mixed_site(), format!("Attributes are required")))
    }

    for arg in &args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.segments.is_empty() {
                    return Err(syn::Error::new_spanned(nv, "Empty segments"));
                }

                let ident = nv.path.segments.first().unwrap().ident.clone();

                // ignore this one
                if ignored_args.contains(&&*ident.to_string()) {
                    continue;
                }

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

pub fn get_cfg_target(attributes: &Vec<Attribute>) -> TokenStream {
    let mut tk = TokenStream::new();

    let mut found_cfg = false;
    for attr in attributes {
        for seg in &attr.path.segments {
            if seg.ident.to_string() == "jtarget" {
                found_cfg = true;
            }
        }

        if found_cfg {
            let cfg_tokens = &attr.tokens;
            tk = quote! {
                #[cfg#cfg_tokens]
            };
        }

        found_cfg = false;
    }

    tk
}

pub fn class_to_ident(class: &str, fn_name: &TokenStream) -> TokenStream {
    let class = class.replace("/", "_").replace(".", "_").replace("\"", "");
    let fn_name = fn_name.to_string().replace("\"", "");

    format_ident!("Java_{}_{}", class, fn_name.to_string()).to_token_stream()
}

pub fn fix_class_path(class: &String, slashes: bool) -> String {
    // if not slashes, then underscores
    if !slashes {
        class.replace("/", "_").replace(".", "_")
    } else {
        class.replace(".", "/").replace("_", "/")
    }
}

pub fn get_null_return_obj(ident: &str) -> TokenStream {
    let mut tks = TokenStream::new();

    let null_mut = quote! { std::ptr::null_mut() };
    match ident {
        // object types
        "jobject" => tks.extend(null_mut),
        "jclass" => tks.extend(null_mut),
        "jthrowable" => tks.extend(null_mut),
        "jstring" => tks.extend(null_mut),
        "jarray" => tks.extend(null_mut),
        "jbooleanArray" => tks.extend(null_mut),
        "jbyteArray" => tks.extend(null_mut),
        "jcharArray" => tks.extend(null_mut),
        "jshortArray" => tks.extend(null_mut),
        "jintArray" => tks.extend(null_mut),
        "jlongArray" => tks.extend(null_mut),
        "jfloatArray" => tks.extend(null_mut),
        "jdoubleArray" => tks.extend(null_mut),
        "jobjectArray" => tks.extend(null_mut),
        "jweak" => tks.extend(null_mut),
        "jfieldID" => tks.extend(null_mut),
        "jmethodID" => tks.extend(null_mut),

        // numeric types
        "jint" => tks.extend(quote! { 0 as jni::sys::jint }),
        "jlong" => tks.extend(quote! { 0 as jni::sys::jlong }),
        "jbyte" => tks.extend(quote! { 0 as jni::sys::jbyte }),
        "jboolean" => tks.extend(quote! { 0 as jni::sys::jboolean }),
        "jchar" => tks.extend(quote! { 0 as jni::sys::jchar }),
        "jshort" => tks.extend(quote! { 0 as jni::sys::jshort }),
        "jfloat" => tks.extend(quote! { 0.0 as jni::sys::jfloat }),
        "jdouble" => tks.extend(quote! { 0.0 as jni::sys::jdouble }),
        "jsize" => tks.extend(quote! { 0 as jni::sys::jsize }),

        _ => ()
    }

    tks
}

// Ok result is (ReturnType, is_result_type)
pub fn extract_return(ret: &ReturnType, name: &Ident, impl_name: Option<&Ident>, attributes: &Vec<String>) -> syn::Result<(ReturnType, String, bool)> {
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

                    if attributes.contains(&"jdestroy".to_string()) {
                        return Err(syn::Error::new_spanned(v, "Destroy method cannot have return type"))
                    }

                    if impl_name.is_some() {
                        // restrict return type of Self::new()
                        if ident != "Result" && ident != "Self" && ident != &*impl_name.unwrap().to_string() && attributes.contains(&"jnew".to_string()) {
                            return Err(syn::Error::new_spanned(v, "Return type must be a `Self` type or Result<Self> type"))
                        }
                    }

                    if ident != "Result" && !allowed_ret.contains(&&*ident.to_string()) {
                        // special case - allow new() functions to return Self for the jniclass implementation
                        if !attributes.contains(&"jnew".to_string()) {
                            return Err(syn::Error::new_spanned(ident, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
                        } else if impl_name.is_some() {
                            if ident != "Self" && ident != &*impl_name.unwrap().to_string() {
                                return Err(syn::Error::new_spanned(ident, "Return type must be a Result<Self> type, `Self` type, or empty"))
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
                                return Err(syn::Error::new_spanned(args, "Angle brackets <> need a primitive j type (jni::sys::*) or ()"))
                            }

                            let _ok_res = &a_args[0];
                            let ok_res: syn::Type;
                            let ident: String;
                            match _ok_res {
                                syn::GenericArgument::Type(ref v) => {
                                    // validate it's an allowed type
                                    match v {
                                        syn::Type::Path(i_v) => {
                                            let seg = &i_v.path.segments;
                                            if seg.len() == 0 {
                                                return Err(syn::Error::new_spanned(args, "Empty segments"));
                                            }

                                            ident = seg.last().unwrap().ident.to_string();
                                            if attributes.contains(&"jnew".to_string()) {
                                                if ident != "Self" && ident != impl_name.unwrap().to_string() {
                                                    return Err(syn::Error::new_spanned(v, "Return type must be a Self type"))
                                                }
                                            } else if !allowed_ret.contains(&&*ident) {
                                                return Err(syn::Error::new_spanned(v, "Return type must be a primitive j type (jni::sys::*) or ()"))
                                            }

                                            ok_res = v.clone();
                                        }

                                        syn::Type::Tuple(t) => {
                                            if attributes.contains(&"jnew".to_string()) {
                                                return Err(syn::Error::new_spanned(_ok_res, "Return type must be a Self type"));
                                            }

                                            // this is an empty () ok type
                                            // so just return no return type then
                                            if t.elems.len() == 0 {
                                                return Ok((ReturnType::Default, "".to_string(), true));
                                            }
                                            return Err(syn::Error::new_spanned(_ok_res, "Must be an empty ok () type"))
                                        }

                                        _ => {
                                            if attributes.contains(&"jnew".to_string()) {
                                                return Err(syn::Error::new_spanned(_ok_res, "Return type must be a Self type"));
                                            }

                                            return Err(syn::Error::new_spanned(_ok_res, "Return type in brackets must be primitive j type (jni::sys::*) or ()"))
                                        }
                                    }
                                }

                                _ => {
                                    if attributes.contains(&"jnew".to_string()) {
                                            return Err(syn::Error::new_spanned(args, "Return type must be a Self type"));
                                    }

                                    return Err(syn::Error::new_spanned(args, "Return type must be a primitive j type (jni::sys::*) or ()"))
                                }
                            }

                            // reconstruct return type
                            let typebox = Box::new(ok_res);
                            Ok((ReturnType::Type(*arrow_token, typebox), ident, true))
                        }

                        PathArguments::None => {
                            let new = v.clone();
                            Ok(
                                (ReturnType::Type(*arrow_token, Box::new(Type::Path(new))), ident.to_string(), false)
                            )
                        }

                        _ => {
                            Err(syn::Error::new_spanned(&segment, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
                        }
                    }

                }

                syn::Type::Tuple(t) => {
                    if attributes.contains(&"jnew".to_string()) {
                        return Err(syn::Error::new_spanned(_ty, "Return type must be a Self type"));
                    }

                    // this is an empty () ok type
                    // so just return no return type then
                    if t.elems.len() == 0 {
                        return Ok((ReturnType::Default, "".to_string(), false));
                    }
                    return Err(syn::Error::new_spanned(_ty, "Must be an empty ok () type"))
                }

                _ => {
                    if attributes.contains(&"jdestroy".to_string()) {
                        return Err(syn::Error::new_spanned(_ty, "Destroy method cannot have return type"));
                    }

                    Err(syn::Error::new_spanned(_ty, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
                }
            }
        }

        _ => {
            if impl_name.is_some() && attributes.contains(&"jnew".to_string()) {
                Err(syn::Error::new_spanned(name, "Impl new() must return `Self` type"))
            } else {
                Ok((ReturnType::Default, "".to_string(), false))
            }
        }
    }
}

// returns the first JNIEnv ident (skips &self)
pub fn validate_fn_args(fn_args: &Punctuated<FnArg, Comma>, is_impl: bool, attrs: &Vec<String>) -> syn::Result<()> {
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
                        return Err(syn::Error::new(Span::mixed_site(), "Empty segments"));
                    }

                    let ty = seg.last().unwrap();
                    if pos == 1 {
                        if ty.ident.to_string() != "JNIEnv" {
                            return Err(syn::Error::new_spanned(v, "Param must be JNIEnv"));
                        }
                    } else if pos == 2 {
                        if !allowed_types_second_param.contains(&&*ty.ident.to_string()) {
                            return Err(syn::Error::new_spanned(v, "Param must be JObject or JClass (methods only)"));
                        }

                        if is_impl && ty.ident.to_string().to_lowercase() == "jclass" {
                            if !attrs.contains(&"jstatic".to_string()) {
                                return Err(syn::Error::new_spanned(v, "JClass is not allowed in second position on impl methods"));
                            }
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

pub fn extract_second_type<'a>(fn_args: &Punctuated<FnArg, Comma>) -> Option<TokenStream> {
    let mut i = 0usize;
    for arg in fn_args {
        match arg {
            FnArg::Typed(v) => {

                if i == 1 {
                    if let Type::Path(r) = &*v.ty {
                        if r.path.segments.len() > 0 {
                            return Some(r.path.segments.to_token_stream())
                        }
                    }
                } else if i > 1 {
                    break;
                }

                i += 1;
            }

            // self arg, just ignore that
            _ => {}
        }
    }

    None
}

pub fn filter_out_ignored(item_impl: &mut ItemImpl) {
    item_impl.items.retain(|i| {
        if let ImplItem::Method(m) = i {
            for attr in &m.attrs {
                let s = &attr.path.segments;
                for seg in s {
                    let ident = seg.ident.to_string();
                    if ident == "jignore" {
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
            let attrs = top_attrs(&m.attrs);
            validate_fn_args(&m.sig.inputs, true, &attrs)?;
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

        Type::Group(g) => {
            match &*g.elem {
                Type::Path(p) => {
                    let segments = &p.path.segments;
                    if segments.len() == 0 {
                        return Err(syn::Error::new_spanned(self_type, "Segments empty"))
                    }

                    let last = p.path.segments.last().unwrap();
                    type_ident = &last.ident;
                }

                _ => return Err(syn::Error::new_spanned(self_type, "Missed match")),
            }
        }

        _ => return Err(syn::Error::new_spanned(self_type, "Missed match")),
    }

    Ok(type_ident.to_owned())
}

pub fn validate_impl_returns(items: &Vec<ImplItem>, name: &Ident) -> syn::Result<Vec<(ReturnType, String, bool)>> {
    let mut impl_returns = vec![];
    for item in items {
        if let ImplItem::Method(m) = item {
            impl_returns.push(extract_return(&m.sig.output, &m.sig.ident, Some(name), &top_attrs(&m.attrs))?);
        }
    }

    Ok(impl_returns)
}

pub fn impl_extract_second_types(items: &Vec<ImplItem>) -> syn::Result<Vec<Option<TokenStream>>> {
    let mut impl_idents = vec![];

    for item in items {
        if let ImplItem::Method(m) = item {
            let second_type = extract_second_type(&m.sig.inputs);
            impl_idents.push(second_type);
        }
    }

    Ok(impl_idents)
}

pub fn impl_fn_args(input: &Punctuated<FnArg, Comma>) -> syn::Result<Vec<(TokenStream, TokenStream)>> {
    let mut new_punc: Vec<(TokenStream, TokenStream)> = vec![];

    let mut i = 0usize;
    for arg in input {
        match arg {
            FnArg::Typed(v) => {
                i += 1;

                // only process after the first 2
                if i >= 3 {
                    let mut ty = TokenStream::new();
                    if let Type::Path(d) = &*v.ty {
                        let seg = &d.path.segments;
                        if seg.len() == 0{
                            return Err(syn::Error::new_spanned(v, "Segments empty"))
                        }
                        ty.extend(seg.to_token_stream());
                    }

                    if let Pat::Ident(b) = &*v.pat {
                        let ident = b.ident.to_token_stream();

                        new_punc.push(
                            (ident, ty)
                        )
                    } else if let Pat::Wild(_) = &*v.pat {
                        let id = (0..10)
                            .map(|_| {
                                let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                                CHARSET[idx] as char
                            })
                            .collect::<String>();

                        let id = id.parse::<TokenStream>()?;

                        new_punc.push(
                            (id, ty)
                        );
                    }
                }
            }

            // ignore self param
            FnArg::Receiver(_) => ()
        }
    }

    Ok(new_punc)
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

pub fn fn_full_args(args: &Punctuated<FnArg, Comma>) -> syn::Result<(TokenStream, TokenStream)> {
    let mut fn_sig_vec = vec![];
    let mut fn_call_vec = vec![];

    let mut num = 0usize;
    for arg in args {
        match arg {
            // covers both typed and wildcards
            FnArg::Typed(b) => {
                let mut ty = TokenStream::new();
                let mut ident = TokenStream::new();
                if let Type::Path(d) = &*b.ty {
                    let seg = &d.path.segments;
                    if seg.len() == 0 {
                        return Err(syn::Error::new_spanned(b, "Segments empty"))
                    }

                    ty.extend(seg.to_token_stream());
                }

                if let Pat::Ident(b) = &*b.pat {
                    ident = b.to_token_stream();
                }

                if let Pat::Wild(_) = &*b.pat {
                    let id = (0..10)
                        .map(|_| {
                            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                            CHARSET[idx] as char
                        })
                        .collect::<String>();

                    ident = id.parse::<TokenStream>()?;
                }

                if num == 0 {
                    //fn_sig.extend(syn::parse_str::<TokenStream>("env: jni::JNIEnv"));
                    fn_call_vec.push(quote! { env });
                } else if num == 1 {
                    fn_sig_vec.push(quote! { , obj: #ty });
                    fn_call_vec.push(quote! { , obj });
                } else {
                    fn_sig_vec.push(quote! { , #ident: #ty });
                    fn_call_vec.push(quote! { , #ident });
                }

                num += 1;
            }

            // ignore self
            FnArg::Receiver(_) =>  ()
        }
    }

    if fn_sig_vec.len() == 0 {
        fn_sig_vec.push(quote! { , obj: jni::objects::JObject });
    }

    let mut fn_sig_tokens = TokenStream::new();
    fn_sig_tokens.extend(fn_sig_vec);
    let mut fn_call_tokens = TokenStream::new();
    fn_call_tokens.extend(fn_call_vec);

    Ok((fn_call_tokens, fn_sig_tokens))
}

pub fn impl_fn_fill_args(args: &Punctuated<FnArg, Comma>, rest: &Vec<(TokenStream, TokenStream)>) -> syn::Result<TokenStream> {
    let mut tk = TokenStream::new();

    let mut num = 0usize;
    for arg in args {
        match arg {
            // covers both typed and wildcards
            FnArg::Typed(_) => {
                num += 1;
            }

            // ignore self
            FnArg::Receiver(_) =>  ()
        }
    }

    if num >= 1 {
        tk.extend(syn::parse_str::<TokenStream>("env"));
    }
    if num >= 2 {
        tk.extend(syn::parse_str::<TokenStream>(", obj"));
    }
    if num >= 3 {
        tk.extend(
            rest.iter().map(|(v, _)| { quote! { , #v } }).collect::<TokenStream>()
        );
    }

    Ok(tk)
}

pub fn top_attrs(attributes: &Vec<Attribute>) -> Vec<String> {
    let mut attrs = vec![];

    for attr in attributes {
        for seg in &attr.path.segments {
            attrs.push(seg.ident.to_string());
        }
    }

    attrs
}

pub fn get_rename_attr(ident: &Ident, attributes: &Vec<Attribute>) -> syn::Result<TokenStream> {
    let mut name = ident.to_string();

    let mut is_rename = false;
    for attr in attributes {
        for seg in &attr.path.segments {
            if seg.ident.to_string() == "jname" {
                is_rename = true;
            }
        }

        if is_rename {
            for token in attr.tokens.clone() {
                if let TokenTree::Group(g) = token {
                    for t in g.stream() {
                        if let TokenTree::Literal(l) = t {
                            name = l.to_string().replace("\"", "");
                        }
                    }
                }
            }
        }

        is_rename = false;
    }

    let stream = name.parse::<TokenStream>()?;
    Ok(quote! { #stream })
}


pub fn get_set_take_attrs(attributes: &Vec<Attribute>) -> (Option<String>, Option<String>, Option<String>) {
    let mut jget_option = None;
    let mut jget = false;
    let mut jset_option = None;
    let mut jset = false;
    let mut jtake_option = None;
    let mut jtake = false;
    for attr in attributes {
        if attr.path.segments.len() == 0 {
            continue;
        }

        let last = attr.path.segments.last().unwrap();
        if last.ident.to_string() == "jget" {
            jget = true;
        } else if last.ident.to_string() == "jset" {
            jset = true;
        } else if last.ident.to_string() == "jtake" {
            jtake = true;
        }

        let mut passed = false;
        for token in attr.tokens.clone() {
            if let TokenTree::Group(g) = token {
                for t in g.stream() {
                    if let TokenTree::Ident(i) = &t {
                        if i.to_string() == "from" && (jget || jtake) {
                            passed = true;
                        } else if i.to_string() == "to" && jset {
                            passed = true;
                        }
                    }

                    if passed {
                        if let TokenTree::Literal(l) = &t {
                            let value = Some(l.to_string().replace("\"", ""));

                            if jget {
                                jget_option = value;
                                break;
                            } else if jset {
                                jset_option = value;
                                break;
                            } else if jtake {
                                jtake_option = value;
                                break;
                            }
                        }
                    }
                }
            }
        }

        jget = false;
        jset = false;
        jtake = false;
    }

    (jget_option, jset_option, jtake_option)
}

pub fn is_empty_block(block: &Block) -> bool {
    block.stmts.len() == 0
}

pub fn generate_impl_functions(
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
                        pub extern "system" fn #java_name(env: jni::JNIEnv#inputs) {
                            use jni_tools::Handle;

                            let p_res = std::panic::catch_unwind(|| {
                                #mat_res
                                let res = env.set_handle(#set_varname, r_obj);

                                match res {
                                    Ok(_) => (),
                                    Err(e) => {
                                        let msg = format!("Failed to set handle for `{}` : {}", #diag, e.to_string());
                                        log::error!("{}", msg);
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
                        pub extern "system" fn #java_name(env: jni::JNIEnv#inputs) #ret_type {
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

                    let empty_underscore = if empty_fn {
                        quote! { _ }
                    } else {
                        quote! { v }
                    };

                    let ok_v = if empty_fn {
                        quote! { () }
                    } else {
                        quote! { v }
                    };

                    stream = quote! {
                        #target
                        #[no_mangle]
                        pub extern "system" fn #java_name(env: jni::JNIEnv#inputs) {
                            use jni_tools::Handle;

                            let p_res = std::panic::catch_unwind(|| {
                                let res = env.take_handle::<#impl_name>(#take_varname);

                                #fn_call_res_binding match res {
                                    Ok(#empty_underscore) => #ok_v,
                                    Err(e) => {
                                        let msg = format!("Failed to take handle for `{}` : {}", #diag, e.to_string());
                                        log::error!("{}", msg);
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
                        pub extern "system" fn #java_name(env: jni::JNIEnv#inputs) #ret_type {
                            use jni_tools::Handle;

                            let p_res = std::panic::catch_unwind(|| {
                                let res = env.get_handle::<#impl_name>(#get_varname);

                                let #mut_kwrd r_obj = match res {
                                    Ok(v) => v,
                                    Err(e) => {
                                        let msg = format!("Failed to get handle for `{}` : {}", #diag, e.to_string());
                                        log::error!("{}", msg);
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
