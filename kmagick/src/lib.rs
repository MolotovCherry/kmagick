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

use jni_tools::Cacher;
use utils::Result;
use jni::sys::jobjectArray;
use jni_macros::jniclass;
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
use std::panic;


static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        init_logger();

        panic::set_hook(Box::new(|_| { }));
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

struct Magick {
    is_init: bool
}

#[jniclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad.kmagick/MagickException")]
impl Magick {
    fn new() -> Self {
        Self {
            is_init: false
        }
    }

    fn nativeInit(&mut self) {
        init();
        magick_rust::magick_wand_genesis();
        self.is_init = true;
        
        
        info!("Magick::nativeInit() Initialized native environment");
    }

    fn nativeTerminate(&self, env: JNIEnv) {
        info!("Magick::nativeTerminate() Terminating environment");
        info!("I got a new var : {}", self.is_init);
        magick_rust::magick_wand_terminus();
        env.clear_cache();
    }

    fn magickQueryFonts(&self, env: JNIEnv, _: JObject, pattern: JString) -> Result<jobjectArray> {
        let pat: String = env.get_string(pattern).unwrap().into();

        let fonts = magick_rust::magick_query_fonts(&*pat)?;
        
        //let arr = env.new_object_array(v.len(), "java/lang/String", initial_element);

        Ok(std::ptr::null_mut())
    }

    fn destroy(&self) {
        println!("destroy");
    }
}
