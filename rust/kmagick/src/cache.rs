use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use enumn::N;
use fxhash::FxHashMap;
use jni::JNIEnv;
use jni::objects::GlobalRef;
use lazy_static::lazy_static;

use jni_tools::Handle;

use crate::{
    DrawingWand, MagickWand, PixelWand
};
use crate::utils::WandId;

lazy_static! {
    pub static ref PIXELWAND_CACHE: Mutex<FxHashMap<u64, GlobalRef>> = Mutex::new(FxHashMap::default());
    pub static ref DRAWINGWAND_CACHE: Mutex<FxHashMap<u64, GlobalRef>> = Mutex::new(FxHashMap::default());
    pub static ref MAGICKWAND_CACHE: Mutex<FxHashMap<u64, GlobalRef>> = Mutex::new(FxHashMap::default());
}

#[derive(N)]
pub enum CacheType {
    PixelWand,
    DrawingWand,
    MagickWand
}

macro_rules! TakeObjs {
    ($env:ident, $wand:ident, $cache:ident) => {{
        for wand in $cache.values() {
            // clear handle and let object drop to prevent invalid references to an already deleted obj
            let wand = $env.take_handle::<$wand>(wand.as_obj())?;
            log::trace!("Destroyed {} id {}", stringify!($wand), wand.id());
        }
    }}
}

macro_rules! TakeObj {
    ($env:ident, $wand:ident, $cache:ident, $id:ident) => {{
        // clear handle and let object drop to prevent invalid references to an already deleted obj
        let wand = $cache.get(&$id);
        if let Some(w) = wand {
            let w = $env.take_handle::<$wand>(w.as_obj()).unwrap();
            log::trace!("Destroyed {} id {}", stringify!($wand), w.id());
        }
    }}
}

pub fn clear(env: JNIEnv) -> crate::Result<()> {
    let pixel_cache = &mut *PIXELWAND_CACHE.lock()?;
    let magick_cache = &mut *MAGICKWAND_CACHE.lock()?;
    let drawing_cache = &mut *DRAWINGWAND_CACHE.lock()?;

    // first we need to take all the objects out
    TakeObjs!(env, PixelWand, pixel_cache);
    TakeObjs!(env, DrawingWand, drawing_cache);
    TakeObjs!(env, MagickWand, magick_cache);

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

pub fn destroy_type<W>(env: JNIEnv, cache: &'static Mutex<FxHashMap<u64, GlobalRef>>) -> crate::Result<()>
    where W: Send + WandId + 'static
{
    let cache = &mut *cache.lock()?;

    TakeObjs!(env, W, cache);

    cache.clear();
    Ok(())
}

pub fn destroy_ids<W>(env: JNIEnv, cache: &'static Mutex<FxHashMap<u64, GlobalRef>>, ids: &[u64]) -> crate::Result<()>
    where W: Send + WandId + 'static
{
    let cache = &mut *cache.lock()?;

    for id in ids {
        TakeObj!(env, W, cache, id);
        let _ =  cache.remove(id);
    }

    Ok(())
}

// Remove entry from the cache
pub fn remove<W>(env: JNIEnv, cache: &'static Mutex<FxHashMap<u64, GlobalRef>>, id: u64)
    where W: Send + WandId + 'static
{
    let cache = &mut *cache.lock().expect("Poisoned lock");
    TakeObj!(env, W, cache, id);
    let _ = cache.remove(&id);
}
