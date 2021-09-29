use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, FnArg, ImplItem, ItemImpl, Meta, NestedMeta, Pat, PathArguments, ReturnType, Type};
use quote::quote;
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn get_args(args: Vec<NestedMeta>) -> syn::Result<HashMap<String, String>> {
    let mut hm: HashMap<String, String> = HashMap::new();

    let allowed_args = vec!["cls", "exc", "pkg", "name"];

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

pub fn fix_class_path(class: &String, slashes: bool) -> String {
    // if not slashes, then underscores
    if !slashes {
        class.replace("/", "_").replace(".", "_")
    } else {
        class.replace(".", "/").replace("_", "/")
    }
}

// Ok result is (ReturnType, is_result_type)
pub fn extract_return(ret: &ReturnType, name: &Ident, impl_name: Option<&Ident>, attributes: &Vec<String>) -> syn::Result<(ReturnType, bool)> {
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

                    if attributes.contains(&"jni_destroy".to_string()) {
                        return Err(syn::Error::new_spanned(v, "Destroy method cannot have return type"))
                    }

                    if impl_name.is_some() {
                        // restrict return type of Self::new()
                        if ident_str != "Self" && ident_str != impl_name.unwrap().to_string() && attributes.contains(&"jni_new".to_string()) {
                            return Err(syn::Error::new_spanned(v, "Return type must be a `Self` type"))
                        }
                    }

                    if ident_str != "Result" && !allowed_ret.contains(&&*ident_str) {
                        // special case - allow new() functions to return Self for the jniclass implementation
                        if !attributes.contains(&"jni_new".to_string()) {
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
                    if attributes.contains(&"jni_destroy".to_string()) {
                        return Err(syn::Error::new_spanned(_ty, "Destroy method cannot have return type"));
                    }

                    Err(syn::Error::new_spanned(_ty, "Return type must be a Result<> type, primitive j type (jni::sys::*), or empty"))
                }
            }
        }

        _ => {
            if impl_name.is_some() && attributes.contains(&"jni_new".to_string()) {
                Err(syn::Error::new_spanned(name, "Impl new() must return `Self` type"))
            } else {
                Ok((ReturnType::Default, false))
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
                        return Err(syn::Error::new(Span::call_site(), "Empty segments"));
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
                            if !attrs.contains(&"jni_static".to_string()) {
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

pub fn extract_second_type<'a>(fn_args: &Punctuated<FnArg, Comma>) -> Option<Ident> {
    let mut i = 0usize;
    for arg in fn_args {
        match arg {
            FnArg::Typed(v) => {

                if i == 1 {
                    if let Type::Path(r) = &*v.ty {
                        if let Some(s) = r.path.segments.last() {
                            return Some(s.ident.clone());
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
                    if ident == "jni_ignore" {
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

        _ => return Err(syn::Error::new_spanned(self_type, "Missed match")),
    }

    Ok(type_ident.to_owned())
}

pub fn validate_impl_returns(items: &Vec<ImplItem>, name: &Ident) -> syn::Result<Vec<(ReturnType, bool)>> {
    let mut impl_returns = vec![];
    for item in items {
        if let ImplItem::Method(m) = item {
            impl_returns.push(extract_return(&m.sig.output, &m.sig.ident, Some(name), &top_attrs(&m.attrs))?);
        }
    }

    Ok(impl_returns)
}

pub fn impl_extract_second_types(items: &Vec<ImplItem>) -> syn::Result<Vec<Option<Ident>>> {
    let mut impl_idents = vec![];

    for item in items {
        if let ImplItem::Method(m) = item {
            let second_type = extract_second_type(&m.sig.inputs);
            impl_idents.push(second_type);
        }
    }

    Ok(impl_idents)
}

pub fn impl_fn_args(input: &Punctuated<FnArg, Comma>) -> syn::Result<Vec<(String, String)>> {
    let mut new_punc: Vec<(String, String)> = vec![];

    let mut i = 0usize;
    for arg in input {
        match arg {
            FnArg::Typed(v) => {
                i += 1;

                // only process after the first 2
                if i >= 3 {
                    let mut ty: String = "".to_string();
                    if let Type::Path(d) = &*v.ty {
                        let seg = &d.path.segments;
                        if seg.len() == 0{
                            return Err(syn::Error::new_spanned(v, "Segments empty"))
                        }
                        ty = seg.last().unwrap().ident.to_string()
                    }

                    if let Pat::Ident(b) = &*v.pat {
                        let ident = b.ident.to_string();

                        new_punc.push((ident, ty.clone()))
                    }

                    if let Pat::Wild(_) = &*v.pat {
                        let id: String;

                        id = (0..10)
                            .map(|_| {
                                let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                                CHARSET[idx] as char
                            })
                            .collect();
                        
                        new_punc.push(
                            (id, ty.clone())
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
                let mut ty = String::new();
                let mut ident = String::new();
                if let Type::Path(d) = &*b.ty {
                    let seg = &d.path.segments;
                    if seg.len() == 0 {
                        return Err(syn::Error::new_spanned(b, "Segments empty"))
                    }
                    ty = seg.last().unwrap().ident.to_string();
                }

                if let Pat::Ident(b) = &*b.pat {
                    ident = b.ident.to_string();
                }

                if let Pat::Wild(_) = &*b.pat {
                    ident = (0..10)
                        .map(|_| {
                            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                            CHARSET[idx] as char
                        })
                        .collect();
                }

                if num == 0 {
                    //fn_sig.extend(syn::parse_str::<TokenStream>("env: JNIEnv"));
                    fn_call_vec.push("env".to_string());
                } else if num == 1 {
                    fn_sig_vec.push(format!(", obj: {}", ty));
                    fn_call_vec.push("obj".to_string());
                } else {
                    fn_sig_vec.push(format!("{}: {}", ident, ty));
                    fn_call_vec.push(ident);
                }

                num += 1;
            }

            // ignore self
            FnArg::Receiver(_) =>  ()
        }
    }

    if fn_sig_vec.len() == 0 {
        fn_sig_vec.push(", obj: JObject".to_string());
    }

    let fn_sig_str = fn_sig_vec.join(", ");
    let fn_call_str = fn_call_vec.join(", ");

    let fn_sig: TokenStream = syn::parse_str(&fn_sig_str)?;
    let fn_call: TokenStream = syn::parse_str(&fn_call_str)?;

    Ok((fn_call, fn_sig))
}

pub fn impl_fn_fill_args(args: &Punctuated<FnArg, Comma>, rest: &Vec<(String, String)>) -> syn::Result<TokenStream> {
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

    tk.extend(
        syn::parse_str::<TokenStream>(&rest.iter().map(|v| { format!(", {}", v.0) }).collect::<String>())?
    );

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

pub fn rename_attr(ident: &Ident, attributes: &Vec<Attribute>) -> Ident {
    let mut name = ident.to_string();

    let mut is_rename = false;
    for attr in attributes {
        for seg in &attr.path.segments {
            if seg.ident.to_string() == "jni_name" {
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
    }

    Ident::new(&name, ident.span())
}


pub fn generate_impl_functions(
    items: &Vec<ImplItem>,
    returns: &Vec<(ReturnType, bool)>,
    namespace: (&String, bool, &Ident),
    exc: &str
) -> syn::Result<Vec<TokenStream>> {
    let mut funcs: Vec<TokenStream> = vec![];

    let second_types = impl_extract_second_types(items)?;

    for (i, _fn) in items.iter().enumerate() {
        match _fn {
            ImplItem::Method(m) => {
                let fn_name = rename_attr(&m.sig.ident, &m.attrs);

                let ret_type = &returns[i].0;
                let is_result = returns[i].1;

                let attrs = top_attrs(&m.attrs);

                let fn_inputs = impl_fn_args(&m.sig.inputs)?;
                let fn_is_mut = impl_is_fn_mut(&m.sig.inputs);

                let fn_call_args = impl_fn_fill_args(&m.sig.inputs, &fn_inputs)?;

                let inputs = match &second_types[i] {
                    Some(v) => {
                        let mut tk = quote!{
                            , obj: #v
                        };

                        let res: TokenStream = syn::parse_str(&fn_inputs.iter().map(|v| { format!(", {}: {}", v.0, v.1) }).collect::<String>())?;
                        tk.extend(res);

                        tk
                    }

                    None => {
                        quote! {
                            , obj: JObject
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
                    ReturnType::Type(_, _) => true
                };

                let class = if is_pkg {
                    format!("{}_{}", ns, impl_name)
                } else {
                    ns.to_owned()
                };

                let handle_cls = fix_class_path(&class, true);

                let java_name = class_to_ident(&class, &fn_name.to_string());
                
                //
                // special changing syntax
                //
                let null_mut = if is_returning {
                    quote! {
                        ::std::ptr::null_mut()
                    }
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
                            Ok(v) => v,
                            Err(e) => {
                                env.throw_new(#exc, format!("`{}` threw an exception : {}", #diag, e.to_string())).ok();
                                ::std::ptr::null_mut()
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


                // special case for new fn
                let stream: TokenStream;
                if attrs.contains(&"jni_new".to_string()) {

                    stream = quote! {
                        #[no_mangle]
                        pub extern "system" fn #java_name(env: JNIEnv#inputs) {
                            use ::jni_tools::Handle;
                            use ::jni_tools::Cacher;

                            let p_res = ::std::panic::catch_unwind(|| {
                                let r_obj = #impl_name::#fn_name(#fn_call_args);
                                let res = env.set_handle(#handle_cls, obj, r_obj);

                                match res {
                                    Ok(_) => (),
                                    Err(e) => {
                                        env.throw_new(#exc, format!("Failed to set handle for `{}` : {}", #diag, e.to_string())).ok();
                                    }
                                }
                            });

                            match p_res {
                                Ok(_) => (),
                                Err(e) => {
                                    env.throw_new("java/lang/RuntimeException", &format!("`{}` panicked", #diag)).ok();
                                }
                            }
                        }
                    };
                } else if attrs.contains(&"jni_static".to_string()) {

                    stream = quote! {
                        #[no_mangle]
                        pub extern "system" fn #java_name(env: JNIEnv#inputs) #ret_type {
                            let p_res = ::std::panic::catch_unwind(|| {
                                #res_binding #impl_name::#fn_name(#fn_call_args)#res_semicolon

                                #match_res
                            });

                            match p_res {
                                Ok(#v_or_underscore) => #v_or_unit,
                                Err(e) => {
                                    env.throw_new("java/lang/RuntimeException", &format!("`{}` panicked", #diag)).ok();
                                    #null_mut
                                }
                            }
                        }
                    };
                
                } else if attrs.contains(&"jni_destroy".to_string()) {

                    let mut_kwrd = if fn_is_mut {
                        quote! { mut }
                    } else {
                        TokenStream::new()
                    };

                    stream = quote! {
                        #[no_mangle]
                        pub extern "system" fn #java_name(env: JNIEnv#inputs) {
                            use ::jni_tools::Handle;
                            use ::jni_tools::Cacher;

                            let p_res = ::std::panic::catch_unwind(|| {
                                let res = env.take_handle::<#impl_name>(#handle_cls, obj);

                                let #mut_kwrd r_obj = match res {
                                    Ok(v) => v,
                                    Err(e) => {
                                        env.throw_new(#exc, format!("Failed to take handle for `{}` : {}", #diag, e.to_string())).ok();
                                        return;
                                    }
                                };

                                r_obj.#fn_name(#fn_call_args);
                            });

                            match p_res {
                                Ok(_) => (),
                                Err(e) => {
                                    env.throw_new("java/lang/RuntimeException", &format!("`{}` panicked", #diag)).ok();
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
                        #[no_mangle]
                        pub extern "system" fn #java_name(env: JNIEnv#inputs) #ret_type {
                            use ::jni_tools::Handle;
                            use ::jni_tools::Cacher;

                            let p_res = ::std::panic::catch_unwind(|| {
                                let res = env.get_handle::<#impl_name>(#handle_cls, obj);

                                let #mut_kwrd r_obj = match res {
                                    Ok(v) => v,
                                    Err(e) => {
                                        
                                        env.throw_new(#exc, format!("Failed to get handle for `{}` : {}", #diag, e.to_string())).ok();
                                        return #null_mut;
                                    }
                                };

                                #res_binding r_obj.#fn_name(#fn_call_args)#res_semicolon

                                #match_res
                            });

                            match p_res {
                                Ok(#v_or_underscore) => #v_or_unit,
                                Err(e) => {
                                    env.throw_new("java/lang/RuntimeException", &format!("`{}` panicked", #diag)).ok();
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
