#![allow(non_snake_case)]
#![feature(hash_drain_filter)]
#![allow(dead_code)]
#![feature(panic_info_message)]

#[macro_use]
mod macros;
mod drawing_wand;
mod magick_wand;
mod pixel_wand;
mod utils;

use jni_tools::{
    Utils, setup_panic, jclass,
    jignore, jname, jstatic
};
use utils::Result;
use jni::{
    JNIEnv
};
use jni::sys::{jint, jobjectArray, jsize, jboolean};
use jni::objects::{JObject, JString};
use magick_rust;

use log::{LevelFilter, info};

#[cfg(target_os="android")]
use android_logger::Config;
#[cfg(target_os="android")]
use log::Level;
#[cfg(not(target_os="android"))]
use simplelog::*;

use std::sync::Once;


static INIT: Once = Once::new();

fn init() -> Result<()> {
    let mut result = None;
    INIT.call_once(|| {
        // hack to move the Result<> out since
        // this closure won't allow us to return
        result = Some(init_logger());

        // empty panic handler
        setup_panic!();
    });

    // isn't it guaranteed to be Some?
    // actually, no. If init() get's called again, it will == None
    // so let's ignore an unwrap() otherwise it'll panic
    match result {
        Some(v) => Ok(v?),
        None => Ok(())
    }
}

#[cfg(not(target_os="android"))]
fn init_logger() -> Result<()> {
    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Trace,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto
            ),
        ]
    )?;

    if cfg!(debug_assertions) {
        log::set_max_level(LevelFilter::Debug);
    } else {
        log::set_max_level(LevelFilter::Info);
    }

    Ok(())
}

#[cfg(target_os="android")]
fn init_logger() -> Result<()> {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace)
            .with_tag("MAGICK")
    );

    if cfg!(debug_assertions) {
        log::set_max_level(LevelFilter::Debug);
    } else {
        log::set_max_level(LevelFilter::Info);
    }

    Ok(())
}

struct Magick { }

#[jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad.kmagick/MagickException")]
impl Magick {
    #[jstatic]
    fn nativeInit() -> Result<()> {
        init()?;

        magick_rust::magick_wand_genesis();

        info!("Magick::nativeInit() Initialized native environment");

        Ok(())
    }

    #[jstatic]
    fn magickQueryFonts(env: JNIEnv, _: JObject, pattern: JString) -> Result<jobjectArray> {
        let pat: String = env.get_jstring(pattern)?;

        let fonts = magick_rust::magick_query_fonts(&*pat)?;

        let arr = env.new_object_array(fonts.len() as jsize, "java/lang/String", JObject::null())?;
        for (i, font) in fonts.iter().enumerate() {
            let value = env.new_string(font)?;
            env.set_object_array_element(arr, i as jsize, value)?;
        }

        Ok(arr)
    }

    #[jstatic]
    fn terminate() {
        magick_rust::magick_wand_terminus();
        info!("Magick::terminate() Terminated environment");
    }

    #[jstatic]
    #[jname(name="nativeSetLogLevel")]
    fn setLogLevel(_: JNIEnv, _: JObject, level: jint) {
        let level = match level {
            x if x == LevelFilter::Off as i32 => LevelFilter::Off,
            x if x == LevelFilter::Error as i32 => LevelFilter::Error,
            x if x == LevelFilter::Warn as i32 => LevelFilter::Warn,
            x if x == LevelFilter::Info as i32 => LevelFilter::Info,
            x if x == LevelFilter::Debug as i32 => LevelFilter::Debug,
            x if x == LevelFilter::Trace as i32 => LevelFilter::Trace,
            _ => LevelFilter::Off
        };

        log::set_max_level(level);
    }

    #[jstatic]
    fn isInitialized() -> jboolean {
        let res = Magick::isMagickWandInstantiated();
        match res {
            true => 1 as jboolean,
            false => 0 as jboolean
        }
    }

    #[jignore]
    fn isMagickWandInstantiated() -> bool {
        unsafe {
            match magick_rust::bindings::IsMagickWandInstantiated() {
                magick_rust::bindings::MagickBooleanType_MagickTrue => return true,
                magick_rust::bindings::MagickBooleanType_MagickFalse => return false,
                _ => return false
            }
        }
    }
}
