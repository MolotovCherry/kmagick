use proc_macro2::Span;
use syn::{FnArg, Ident, ReturnType, Token, parse::{Parse, ParseStream}, punctuated::Punctuated, token::{Comma, parsing}};

// new token type <==
#[allow(unused)]
struct DoubleFatLeftArrow {
    pub spans: [Span; 3]
}

impl Parse for DoubleFatLeftArrow {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            spans: parsing::punct(input, "<<=")?,
        })
    }
}


#[derive(Debug)]
pub struct BindingFn {
    pub name: Ident,
    pub fn_args: Punctuated<FnArg, Comma>,
    pub ret: ReturnType
}

#[derive(Debug)]
pub struct Binding {
    pub fn_name: Ident,
    pub mutable: bool,
    pub binding_fn: BindingFn
}

impl Parse for Binding {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // optional mut keyword
        let mutable: Option<Token![mut]> = input.parse()?;
        // final_name
        let fn_name: Ident = input.parse()?;
        // <==
        input.parse::<DoubleFatLeftArrow>()?;
        // binding_name
        let binding_fn_name: Ident = input.parse()?;
        // (arg: type)
        let binding_fn_args = super::utils::parse_fn_args(input)?;
        // -> foobar
        let binding_fn_ret = input.parse::<ReturnType>()?;

        let binding_fn_res = BindingFn {
            name: binding_fn_name,
            fn_args: binding_fn_args,
            ret: binding_fn_ret
        };

        Ok(Self {
            fn_name,
            mutable: mutable.is_some(),
            binding_fn: binding_fn_res
        })
    }
}

#[derive(Debug)]
pub struct ItemBinding {
    pub impl_token: Ident,
    pub items: Punctuated<Binding, Token![,]>
}

impl Parse for ItemBinding {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let impl_token: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        // now to process all the fn definitions
        // jniFnName(arg: type) -> return <<= binding_fn_name(jniFnNameInputArg: type) -> arg: JString,

        let mut items: Punctuated<Binding, Token![,]> = Punctuated::new();
        loop {
            // extra comma at very end
            if input.is_empty() {
                break;
            }

            items.push_value(input.parse()?);

            // no extra comma at very end
            if input.is_empty() {
                break;
            }

            items.push_punct(input.parse()?);
        }

        Ok(Self {
            impl_token,
            items
        })
    }
}
