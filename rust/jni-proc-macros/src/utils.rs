use proc_macro2::Span;
use syn::LitStr;

pub fn fix_class_path(cls: &LitStr, slashes: bool) -> LitStr {
    // if not slashes, then underscores
    let cls = cls.value();
    let res = if slashes {
        cls.replace(".", "/").replace("_", "/")
    } else {
        cls.replace("/", "_").replace(".", "_")
    };

    LitStr::new(&res, Span::mixed_site())
}
