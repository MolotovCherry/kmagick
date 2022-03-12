use std::collections::HashMap;
use proc_macro2::Ident;
use syn::{AttributeArgs, LitStr};
use syn::spanned::Spanned;

pub(super) fn attr_verifier(attrs: AttributeArgs, values: &HashMap<Ident, LitStr>, name: &str) -> syn::Result<()> {
    //
    // validate correct arguments were passed to attr
    // (allowed_args, required_args)
    //
    let allowed_args = match name {
        "jmethod" => (
            vec!["cls", "exc"],
            vec!["cls", "exc"]
        ),
        "jclass" => (
            vec!["pkg", "exc"],
            vec!["pkg", "exc"]
        ),
        "jtarget" => (
            vec!["target_os"],
            vec!["target_os"]
        ),
        "jname" => (
            vec!["name"],
            vec!["name"]
        ),
        _ => (vec![], vec![])
    };

    if !allowed_args.0.is_empty() {
        if attrs.is_empty() {
            return Err(syn::Error::new(proc_macro2::Span::mixed_site(), format!("Attributes are required")))
        }

        // validate all keys to make sure they're okay to use
        for key in values.keys() {
            let ks = &*key.to_string();
            if !allowed_args.0.contains(&ks) {
                return Err(
                    syn::Error::new(
                        ks.span(),
                        format!("Invalid key; valid options are: {}", allowed_args.0.join(", "))
                    )
                )
            }
        }
    }
    
    Ok(())
}