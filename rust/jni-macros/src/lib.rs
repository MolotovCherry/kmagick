use std::error::Error;

pub use jni_proc_macros::*;

pub type JNIResult<T> = std::result::Result<T, Box<dyn Error>>;
