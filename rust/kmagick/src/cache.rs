use lazy_static::lazy_static;
use std::sync::Mutex;
use jni::objects::GlobalRef;
use jni::JNIEnv;
use jni_tools::Handle;
use fxhash::FxHashMap;
use rand::Rng;
use crate::{
    PixelWand, MagickWand, DrawingWand
};

lazy_static! {
    pub static ref PIXELWAND_CACHE: Mutex<FxHashMap<u64, GlobalRef>> = Mutex::new(FxHashMap::default());
    pub static ref DRAWINGWAND_CACHE: Mutex<FxHashMap<u64, GlobalRef>> = Mutex::new(FxHashMap::default());
    pub static ref MAGICKWAND_CACHE: Mutex<FxHashMap<u64, GlobalRef>> = Mutex::new(FxHashMap::default());
}

macro_rules! TakeObj {
    ($env:ident, $wand:ident, $cache:ident) => {{
        for wand in $cache.values() {
            // clear handle and let object drop to prevent invalid references to an already deleted obj
            let wand = $env.take_handle::<$wand>(wand.as_obj())?;
            log::debug!("Destroyed dangling {} id {}", stringify!($wand), wand.id);
        }
    }}
}

pub fn clear(env: JNIEnv) -> crate::utils::Result<()> {
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

// returns the id used for the key
pub fn insert(cache: &mut FxHashMap<u64, GlobalRef>, value: GlobalRef) -> u64 {
    let mut rng = rand::thread_rng();
    let mut id: u64;

    // randomly generate an id as many times as needed until the key doesn't exist
    // of course, 99.999% of the time it'll break on the first iteration
    loop {
        id = rng.gen();
        if !cache.contains_key(&id) {
            cache.insert(id, value);
            break;
        }

        log::debug!("Found a key collision : {}", id);
    }

    id
}
