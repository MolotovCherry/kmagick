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

use jni_tools::{Cacher, Utils};
use utils::Result;
use jni::sys::{jobjectArray, jsize};
use jni_macros::{jni_class, jni_static};
use magick_rust;

use log::{LevelFilter, info};

#[cfg(target_os="android")]
use android_logger::Config;
#[cfg(target_os="android")]
use log::Level;
#[cfg(not(target_os="android"))]
use simplelog::*;

use std::sync::Once;
use jni::{
    JNIEnv
};
use jni::objects::{JObject, JString};
use jni_tools::setup_panic;


static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        init_logger();

        // empty panic handler
        setup_panic!();
    });
}


#[cfg(not(target_os="android"))]
fn init_logger() {
    CombinedLogger::init(
        vec![
            TermLogger::new(
                debug_cond!(LevelFilter::Debug, LevelFilter::Info),
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto
            ),
        ]
    ).unwrap();
}

#[cfg(target_os="android")]
fn init_logger() {
    android_logger::init_once(
        Config::default()
            .with_min_level(debug_cond!(Level::Debug, Level::Info))
            .with_tag("MAGICK")
    );
}

struct Magick { }

#[jni_class(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad.kmagick/MagickException")]
impl Magick {
    #[jni_static]
    fn nativeInit() {
        init();
        magick_rust::magick_wand_genesis();

        info!("Magick::nativeInit() Initialized native environment");
    }

    #[jni_static]
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

    #[jni_static]
    fn nativeTerminate(env: JNIEnv) {
        magick_rust::magick_wand_terminus();
        env.clear_cache();

        info!("Magick::nativeTerminate() Terminated environment");
    }
}
