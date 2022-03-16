use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use fxhash::FxHashMap;
use jni::JNIEnv;
use jni::objects::GlobalRef;
use lazy_static::lazy_static;

use jni_tools::Handle;

use crate::{
    DrawingWand, MagickWand, PixelWand
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
            log::trace!("Destroyed {} id {}", stringify!($wand), wand.id);
        }
    }}
}

pub fn clear(env: JNIEnv) -> crate::Result<()> {
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
// for an entire u64's worth, it is guaranteed to not have any collisions
// the id will wraparound after that
// TODO: doubt that'll ever be a problem, but if it is, make an issue report so it can be redesigned
pub fn insert(cache: &Mutex<FxHashMap<u64, GlobalRef>>, value: GlobalRef, name: &str) -> crate::Result<u64> {
    static ID_COUNT: AtomicU64 = AtomicU64::new(0);

    let cache = &mut *cache.lock().expect("Poisoned lock");
    let id = ID_COUNT.fetch_add(1, Ordering::Relaxed);

    cache.insert(id, value);

    log::trace!("Inserted {name} id {id}");

    Ok(id)
}

// Remove entry from the cache
pub fn remove(cache: &Mutex<FxHashMap<u64, GlobalRef>>, id: u64, name: &str) {
    let cache = &mut *cache.lock().expect("Poisoned lock");
    if let Some(_) = cache.remove(&id) {
        log::trace!("Destroyed {name} id {id}");
    } else {
        // it's concievable a race condition can occurr where another thread might try to remove
        // from the cache when another thread terminated it, thereby causing this to be already gone
        log::trace!("{name} id {id} already removed from cache");
    }

}
