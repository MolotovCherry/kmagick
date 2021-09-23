#![allow(non_snake_case)]

mod drawing_wand;
mod magick_wand;
mod pixel_wand;
mod utils;
mod cacher;

use jni::sys::jobject;
use magick_rust::{
    magick_wand_genesis, magick_wand_terminus,
    magick_query_fonts
};

use log::{
    Level, LevelFilter, error, warn, info
};

#[cfg(target_os="android")]
use android_logger::Config;
#[cfg(not(target_os="android"))]
use simplelog::*;

use std::sync::Once;
use jni::{
    JNIEnv
};
use jni::objects::{JObject, JString};

use utils::get_jstring;


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
pub extern fn Java_com_cherryleafroad_jmagick_Magick_nativeInit(_: JNIEnv, _: JObject) {
    init();
    magick_wand_genesis();
    info!("Magick::nativeInit() Initialized native environment");
}

#[no_mangle]
pub extern fn Java_com_cherryleafroad_jmagick_Magick_nativeTerminate(env: JNIEnv, _: JObject) {
    info!("Magick::nativeTerminate() Terminating environment");
    magick_wand_terminus();
    cacher::clear_cache();
}

/*#[no_mangle]
pub extern fn Java_com_cherryleafroad_jmagick_Magick_magickQueryFonts(env: JNIEnv, _: JObject, pattern: JString) -> jobject {
    let pat = get_jstring(env, pattern);

    throw_magick_exc!(env, "foo");
}*/
