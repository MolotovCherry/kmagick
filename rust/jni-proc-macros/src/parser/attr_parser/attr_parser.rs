use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, AttributeArgs, LitStr, NestedMeta, Token};
use syn::punctuated::Punctuated;
use crate::parser::attr_parser::attr_processor::process_attrs;
use crate::parser::attr_parser::attr_verifier::attr_verifier;


pub struct ParsedAttr {
    pub name: Ident,
    pub values: HashMap<Ident, LitStr>,
    jtarget_ts: TokenStream
}

// convert the whole attribute into a token stream
impl ToTokens for ParsedAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let inner = &self.get_inner();

        if self.is_empty() {
            tokens.extend(quote! {
                #[#name]
            });
        } else {
            tokens.extend(quote! {
                #[#name(#inner)]
            });
        }
    }
}

// "foo" == ParsedAttr (nice and easy)
impl PartialEq<&str> for ParsedAttr {
    fn eq(&self, other: &&str) -> bool {
        self.name == other
    }
}

impl PartialEq<ParsedAttr> for ParsedAttr {
    fn eq(&self, other: &ParsedAttr) -> bool {
        self.name == other.name
    }
}
impl Eq for ParsedAttr {}

impl Hash for ParsedAttr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// so we can use methods with string values to compare
// e.g. hashset.contains("jname")
impl Borrow<str> for ParsedAttr {
    fn borrow(&self) -> &str {
        &self.name.to_string()
    }
}

pub trait AttrGet {
    fn get_attr_key_s(&self, name: &str, key: &str) -> Option<String>;
    fn get_attr_key(&self, name: &str, key: &str) -> Option<&LitStr>;
}

// convenience method to get the attr key of a specific attr in a hashset
impl AttrGet for HashSet<ParsedAttr> {
    fn get_attr_key_s(&self, name: &str, key: &str) -> Option<String> {
        // whoa uhhh, getting kotlin vibes here with the ?.
        Some(self.get(name)?.get_s(key)?)
    }

    fn get_attr_key(&self, name: &str, key: &str) -> Option<&LitStr> {
        // whoa uhhh, getting kotlin vibes here with the ?.
        Some(self.get(name)?.get(key)?)
    }
}


impl ParsedAttr {
    /// get inner args
    fn get_inner(&self) -> TokenStream {
        let mut inner = TokenStream::new();

        if self.jtarget_ts.is_empty() {
            // normal attrs as usual
            for (key, val) in self.values {
                inner.extend(
                    quote! {
                        #key = #val,
                    }
                );
            }
        } else {
            // special: not(target_os="foo") ; only for jtarget
            inner.extend(self.jtarget_ts.clone());
        }

        inner
    }

    /// is empty values
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// check if key is in hashmap
    pub fn contains(&self, name: &str) -> bool {
        self.values.keys().any(|k| k == name)
    }

    /// get the value for key
    pub fn get(&self, name: &str) -> Option<&LitStr> {
        let ident = self.values.keys().find(|k| *k == name)?;
        self.values.get(ident)
    }

    /// get the underlying key, except as a String
    pub fn get_s(&self, name: &str) -> Option<String> {
        Some(self.get(name)?.value())
    }

    /// check if attr is a certain one, if so, run closure with value
    pub fn check_run<F: FnMut(&LitStr)>(&self, name: &str, f: F) {
        if self.name == name {
            f(self.get(name).unwrap());
        }
    }

    /// get entire entry
    pub fn get_entry(&self, name: &str) -> Option<(&Ident, &LitStr)> {
        let ident = self.values.keys().find(|k| *k == name)?;
        self.values.get_key_value(ident)
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub(crate) fn parse_attribute(name: &Ident, attrs: &Attribute) -> syn::Result<Self> {
        // sometimes a #[name] may include parens #[name()] or #[name(value="target")].
        // That is to say, the tokenstream is () or (value="target").
        // This only happens with other attributes, not the main attributes passed directly into the proc macro fn.
        // the thing is, for the input to AttributeArgs to process, it needs the inner `value="target"`,
        // and parens cause failure, so let's strip the parens from the tokenstream if it exists
        let attrs = attrs.parse_args::<TokenStream>().unwrap();
        Self::parse(name, &attrs.into())
    }

    pub fn parse(name: &Ident, attrs: &proc_macro::TokenStream) -> syn::Result<Self> {
        // parse into AttributeArgs
        let attrs: AttributeArgs = match syn::parse::Parser::parse(
            Punctuated::<NestedMeta, Token![,]>::parse_terminated,
            attrs.clone(),
        ) {
            | Ok(it) => it.into_iter().collect(),
            | Err(e) => return Err(e),
        };

        //
        // Process attrs
        //
        let mut values = HashMap::new();
        let mut jtarget_ts = TokenStream::new();
        process_attrs(&mut values, &mut jtarget_ts, &attrs)?;

        //
        // validate correct arguments were passed to attr
        //
        attr_verifier(attrs, &values, &*name.to_string())?;

        Ok(Self {
            name: name.clone(),
            values,
            jtarget_ts
        })
    }

    // if attr is jtarget, will convert it into cfg block tokens
    pub fn to_cfg_tokens(&self) -> TokenStream {
        let mut ts = TokenStream::new();

        if self.name == "jtarget" {
            let inner = &self.jtarget_ts;
            ts.extend(
                quote! {
                    #[cfg(#inner)]
                }
            );
        }

        ts
    }
}
