use jni::{
    JNIEnv
};
use log::error;
use magick_rust::MagickWand;
use std::ffi::{
    CString, CStr
};
use jni::objects::{JString, JObject};

use crate::env;


