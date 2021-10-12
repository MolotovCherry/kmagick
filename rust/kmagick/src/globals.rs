use lazy_static::lazy_static;
use std::sync::Mutex;
use jni::objects::GlobalRef;
use jni::JNIEnv;
use jni_tools::Handle;
use crate::{
    PixelWand, MagickWand, DrawingWand
};

lazy_static! {
    pub static ref PIXELWAND_CACHE: Mutex<Vec<GlobalRef>> = Mutex::new(vec![]);
    pub static ref DRAWINGWAND_CACHE: Mutex<Vec<GlobalRef>> = Mutex::new(vec![]);
    pub static ref MAGICKWAND_CACHE: Mutex<Vec<GlobalRef>> = Mutex::new(vec![]);
}

macro_rules! TakeObj {
    ($env:ident, $wand:ident, $cache:ident) => {{
        for wand in &*$cache {
            // clear handle to avoid errors if it was removed before
            $env.clear_handle::<$wand>(wand.as_obj())?;
        }
    }}
}

pub fn clear_cache(env: JNIEnv) -> crate::utils::Result<()> {
    let pixel_cache = &mut *PIXELWAND_CACHE.lock()?;
    let magick_cache = &mut *MAGICKWAND_CACHE.lock()?;
    let drawing_cache = &mut *DRAWINGWAND_CACHE.lock()?;

    // first we need to take all the objects out
    TakeObj!(env, PixelWand, pixel_cache);
    TakeObj!(env, DrawingWand, drawing_cache);
    TakeObj!(env, MagickWand, magick_cache);

    // now clear out all instances
    pixel_cache.clear();
    magick_cache.clear();
    drawing_cache.clear();

    Ok(())
}
