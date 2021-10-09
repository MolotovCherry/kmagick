use proc_macro::TokenStream;

mod utils;
mod parser;

#[proc_macro]
pub fn magick_bindings(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as parser::ItemBinding);

    let res = utils::process_functions(parsed);
    let res = match res {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into()
    };

    res.into()
}
