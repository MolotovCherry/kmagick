use jni::{
    JNIEnv
};
use log::error;
use magick_rust::MagickWand;
use std::error::Error;
use std::ffi::{
    CString, CStr
};
use jni::objects::{JString, JObject};

use crate::env;


pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
