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
            vec!["cls"]
        ),
        "jclass" => (
            vec!["pkg", "cls", "exc"],
            vec![]
        ),
        "jtarget" => (
            vec!["target_os"],
            vec!["target_os"]
        ),
        "jname" => (
            vec!["name"],
            vec!["name"]
        ),
        "jget" | "jtake" => (
            vec!["from"],
            vec!["from"]
        ),
        "jset" => (
            vec!["to"],
            vec!["to"]
        ),
        _ => (vec![], vec![])
    };

    if !allowed_args.0.is_empty() {
        if attrs.is_empty() {
            return Err(syn::Error::new(proc_macro2::Span::mixed_site(), format!("Attributes are required; valid options are: {}", allowed_args.0.join(", "))))
        }

        // validate all keys to make sure they're allowed
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

        // make sure all required args exist
        let string_vec = values.keys().map(|f| f.to_string()).collect::<Vec<_>>();
        for req in allowed_args.1 {
            if !string_vec.contains(&String::from(req)) {
                return Err(syn::Error::new(proc_macro2::Span::mixed_site(), format!("Key {req} required")))
            }
        }
    }

    Ok(())
}
