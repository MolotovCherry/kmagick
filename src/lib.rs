#![allow(non_snake_case)]
#![feature(hash_drain_filter)]

mod drawing_wand;
mod magick_wand;
mod pixel_wand;
mod utils;
#[macro_use]
mod macros;
mod env;

use jni::sys::{jobject, jobjectArray};
use magick_rust;

use log::{
    LevelFilter, info
};

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

use crate::env::Cacher;

//use utils::get_jstring;


static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        init_logger();
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

#[no_mangle]
pub extern fn Java_com_cherryleafroad_kmagick_Magick_nativeInit(_: JNIEnv, _: JObject) {
    init();
    magick_rust::magick_wand_genesis();
    info!("Magick::nativeInit() Initialized native environment");
}

#[no_mangle]
pub extern fn Java_com_cherryleafroad_kmagick_Magick_nativeTerminate(env: JNIEnv, _: JObject) {
    info!("Magick::nativeTerminate() Terminating environment");
    magick_rust::magick_wand_terminus();
    env.clear_cache();
}

/*#[no_mangle]
pub extern fn Java_com_cherryleafroad_kmagick_Magick_magickQueryFonts(env: JNIEnv, _: JObject, pattern: JString) -> jobjectArray {
    let pat = get_jstring(env, pattern);

    if let Some(v) = check_magick_exc!(env, magick_rust::magick_query_fonts(&*pat)) {
        let arr = check_magick_exc!(
            env.new_object_array(v.len(), "java/lang/String", initial_element)
        );
        
    } else {
        std::ptr::null_mut()
    }
}*/
