use proc_macro::TokenStream;

#[macro_use]
mod macros;
mod utils;
mod parser;
mod jmethod;
mod jclass;


/// wrap a function for jni
/// Usage: #[jmethod(cls="some/java/cls", exc="some/exception/Cls")]
/// exc is not required (runtime exception will be used instead)
/// cls is required
#[proc_macro_attribute]
pub fn jmethod(attr: TokenStream, item: TokenStream) -> TokenStream {
    jmethod::jmethod_internal(attr, item).into()
}

/// wrap an entire impl for jni, including all functions inside
/// Usage: #[jclass(pkg="some/java/pkg", exc="some/exception/Cls")]
/// pkg or cls is required, exc is not required (runtime exception will be used instead)
/// pkg will use the name of the impl as the class name
/// cls will override that with a custom cls name
#[proc_macro_attribute]
pub fn jclass(attr: TokenStream, item: TokenStream) -> TokenStream {
    jclass::jclass_internal(attr, item).into()
}

// set the function name used for jni - this way you can use whatever actual function name you want
// used for impl statements. for jmethod, use name attribute instead
#[proc_macro_attribute]
pub fn jname(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

/// Don't generate an implementation for a method in an impl
#[proc_macro_attribute]
pub fn jignore(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

// call as static function instead of instance function
#[proc_macro_attribute]
pub fn jstatic(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

// take the object from the handle allowing it to be dropped
#[proc_macro_attribute]
pub fn jdestroy(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

// set a handle to Self
#[proc_macro_attribute]
pub fn jnew(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

// Change the object that's gotten when using a regular instance function
#[proc_macro_attribute]
pub fn jget(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

// Change the object variable that's set from default to another one when using jnew
#[proc_macro_attribute]
pub fn jset(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

// Change the variable which is taken when used with jdestroy
#[proc_macro_attribute]
pub fn jtake(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}
