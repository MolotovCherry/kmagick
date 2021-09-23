use jni::{
    JNIEnv
};
use std::ffi::{
    CString, CStr
};
use jni::objects::{JString, JObject};

/// If in debug mode, sends first param to function. If in release mode, sends 2nd param to function
#[macro_export]
macro_rules! debug_cond {
    ($a:expr, $b:expr) => {
        if cfg!(debug_assertions) {
            $a
        } else {
            $b
        }
    };
}

pub fn get_jstring(env: JNIEnv, string: JString) -> String {
    unsafe {
        let conv = CString::from(
            CStr::from_ptr(
                env.get_string(string).unwrap().as_ptr()
            )
        );

        conv.into_string().expect("unable to get jstring")
    }
}

#[macro_export]
macro_rules! throw_magick_exc {
    ($env:ident, $m:expr) => {
        {
            let cls = ::cacher::get_cls($env, "com/cherryleafroad/jmagick/MagickException");
            $env.throw_new(cls, $m).expect("failed to throw exception");
        }
    }
}

#[macro_export]
macro_rules! throw_magickwand_exc {
    ($env:ident, $m:expr) => {
        {
            let cls = ::cacher::get_cls($env, "com/cherryleafroad/jmagick/MagickWandException");
            $env.throw_new(cls, $m).expect("failed to throw exception");
        }
    }
}

#[macro_export]
macro_rules! throw_pixelwand_exc {
    ($env:ident, $m:expr) => {
        {
            let cls =  ::cacher::get_cls($env, "com/cherryleafroad/jmagick/PixelWandException").;
            $env.throw_new(cls, $m).expect("failed to throw exception");
        }
    }
}

#[macro_export]
macro_rules! throw_drawingwand_exc {
    ($env:ident, $m:expr) => {
        {
            let cls = ::cacher::get_cls($env, "com/cherryleafroad/jmagick/DrawingWandException");
            $env.throw_new(cls, $m).expect("failed to throw exception");
        }
    }
}

/// Takes the object, gets the Long handle field, and converts back into our Rust type
pub fn get_handle<T>(env: JNIEnv, obj: JObject) -> &'static mut T {
    let val = env.get_field(obj, "HANDLE", "Ljava/lang/Long;").unwrap();
    let l = val.j().unwrap() as usize as *mut T;
    unsafe { &mut *l }
}
