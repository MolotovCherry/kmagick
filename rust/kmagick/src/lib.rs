#![allow(non_snake_case)]

use std::sync::Once;

use jni::JNIEnv;
use jni::objects::{JObject, JString, ReleaseMode};
use jni::sys::{jboolean, jint, jlong, jlongArray, jobjectArray, jsize};
use log::LevelFilter;

use cache::CacheType;
pub use drawing_wand::DrawingWand;
// make available at crate level for macros
use jni_tools::{
    jclass, jignore, jname,
    JNIResult, jstatic, setup_panic, Utils
};
pub use magick_wand::MagickWand;
pub use pixel_wand::PixelWand;
use utils::Result;

#[macro_use]
mod macros;
mod drawing_wand;
mod magick_wand;
mod pixel_wand;
mod utils;
mod cache;
mod errors;

cfg_if::cfg_if! {
    if #[cfg(target_os="android")] {
        use android_logger::Config;
        use log::Level;
    } else {
        use simplelog::*;
    }
}

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

fn init_logger() -> Result<()> {
    #[cfg(not(target_os="android"))]
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

    #[cfg(target_os="android")]
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


struct Magick;

#[jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/MagickException")]
impl Magick {
    #[jstatic]
    fn nativeInit() -> JNIResult<()> {
        init()?;

        magick_rust::magick_wand_genesis();

        log::debug!("Magick::nativeInit() Initialized native environment");

        Ok(())
    }

    #[jstatic]
    fn magickQueryFonts(env: JNIEnv, _: JObject, pattern: JString) -> JNIResult<jobjectArray> {
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
    fn terminate(env: JNIEnv) -> JNIResult<()> {
        // Before terminating, clear cache and take all handles / drop mem, since all internal
        // references will become invalid afterwards. Last thing we need are UB and segfaults
        log::debug!("Magick::terminate(): Clearing all wands");
        cache::clear(env)?;

        magick_rust::magick_wand_terminus();
        log::debug!("Magick::terminate(): Terminated environment");
        Ok(())
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

    // destroy all caches
    #[jstatic]
    fn destroyWands(env: JNIEnv) -> JNIResult<()> {
        // clear the entire wand cache manually
        Ok(cache::clear(env)?)
    }

    // destroy all cache for a specific type
    #[jstatic]
    fn destroyWandType(env: JNIEnv, _: JObject, cache_type: jint) -> JNIResult<()> {
        let cache_type = CacheType::n(cache_type).unwrap();

        match cache_type {
            CacheType::PixelWand => {
                let cache = &*cache::PIXELWAND_CACHE;
                cache::destroy_type::<PixelWand>(env, cache)?;
            }

            CacheType::DrawingWand => {
                let cache = &*cache::DRAWINGWAND_CACHE;
                cache::destroy_type::<DrawingWand>(env, cache)?;
            }

            CacheType::MagickWand => {
                let cache = &*cache::MAGICKWAND_CACHE;
                cache::destroy_type::<MagickWand>(env, cache)?;
            }
        };

        Ok(())
    }

    // destroy a specific cache
    #[jstatic]
    fn destroyWandIds(env: JNIEnv, _: JObject, id_list: jlongArray, cache_type: jint) -> JNIResult<()> {
        let id_list = env.get_long_array_elements(id_list, ReleaseMode::NoCopyBack)?;

        let len = id_list.size()? as usize;
        let slice = unsafe {
            std::slice::from_raw_parts(id_list.as_ptr(), len)
        };

        // kotlin bytecode ULong is actually a J. Perfect! Translates 100% to u64
        let slice = bytemuck::cast_slice::<jlong, u64>(slice);

        let cache_type = CacheType::n(cache_type).unwrap();

        match cache_type {
            CacheType::PixelWand => {
                let cache = &*cache::PIXELWAND_CACHE;
                cache::destroy_ids::<PixelWand>(env, cache, slice)?;
            }

            CacheType::DrawingWand => {
                let cache = &*cache::DRAWINGWAND_CACHE;
                cache::destroy_ids::<DrawingWand>(env, cache, slice)?;
            }

            CacheType::MagickWand => {
                let cache = &*cache::MAGICKWAND_CACHE;
                cache::destroy_ids::<MagickWand>(env, cache, slice)?;
            }
        };

        Ok(())
    }

    #[jstatic]
    fn destroyWandId(env: JNIEnv, _: JObject, id: jlong, cache_type: jint) {
        let id = bytemuck::cast::<jlong, u64>(id);

        let cache_type = CacheType::n(cache_type).unwrap();

        match cache_type {
            CacheType::PixelWand => {
                let cache = &*cache::PIXELWAND_CACHE;
                cache::remove::<PixelWand>(env, cache, id);
            }

            CacheType::DrawingWand => {
                let cache = &*cache::DRAWINGWAND_CACHE;
                cache::remove::<DrawingWand>(env, cache, id);
            }

            CacheType::MagickWand => {
                let cache = &*cache::MAGICKWAND_CACHE;
                cache::remove::<MagickWand>(env, cache, id);
            }
        };
    }

    #[jignore]
    fn isMagickWandInstantiated() -> bool {
        unsafe {
            return match magick_rust::bindings::IsMagickWandInstantiated() {
                magick_rust::bindings::MagickBooleanType_MagickTrue => true,
                magick_rust::bindings::MagickBooleanType_MagickFalse => false,
                _ => false
            }
        }
    }
}
