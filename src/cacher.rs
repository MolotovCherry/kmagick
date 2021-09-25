use std::error::Error;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;

use jni::objects::{JFieldID, JStaticFieldID};
use jni::signature::JavaType;
use jni::sys::jobject;
use jni::{
    objects::{GlobalRef, JClass, JMethodID, JObject, JStaticMethodID, JValue},
    JNIEnv,
};
use lazy_static::lazy_static;
use log::error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    pub static ref CLASS_CACHE: Mutex<HashMap<String, GlobalRef>> = Mutex::new(HashMap::new());
    pub static ref METHOD_ID_CACHE: Mutex<HashMap<String, SendPtr<JMethodID<'static>>>> = Mutex::new(HashMap::new());
    pub static ref STATIC_METHOD_ID_CACHE: Mutex<HashMap<String, SendPtr<JStaticMethodID<'static>>>> = Mutex::new(HashMap::new());
    pub static ref FIELD_ID_CACHE: Mutex<HashMap<String, SendPtr<JFieldID<'static>>>> = Mutex::new(HashMap::new());
    pub static ref STATIC_FIELD_ID_CACHE: Mutex<HashMap<String, SendPtr<JStaticFieldID<'static>>>> = Mutex::new(HashMap::new());
}

// A wrapper to allow send+sync for mutex on JMethod types
pub struct SendPtr<T>(T);
unsafe impl<T> Send for SendPtr<T> {}
unsafe impl<T> Sync for SendPtr<T> {}

#[derive(Debug)]
struct CacherError {
    details: String
}


impl CacherError {
    fn new(msg: &str) -> CacherError {
        CacherError{details: msg.to_string()}
    }
}

impl std::fmt::Display for CacherError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CacherError {
    fn description(&self) -> &str {
        &self.details
    }
}


pub fn clear_cache() {
    let cls_cache = &mut *CLASS_CACHE.lock().unwrap();
    let mid_cache = &mut *METHOD_ID_CACHE.lock().unwrap();
    let smid_cache = &mut *STATIC_METHOD_ID_CACHE.lock().unwrap();
    let fid_cache = &mut *FIELD_ID_CACHE.lock().unwrap();
    let sfid_cache = &mut *STATIC_FIELD_ID_CACHE.lock().unwrap(); 
    // all references inside will auto drop afterwards
    fid_cache.clear();
    sfid_cache.clear();
    mid_cache.clear();
    smid_cache.clear();
    cls_cache.clear();

}

pub fn get_cls<'a>(env: &'a JNIEnv, cls: &str) -> Result<JClass<'a>>
{
    let cache = &mut *CLASS_CACHE.lock().unwrap();
    match cache.get(cls) {
        Some(gref) => Ok(JClass::from(*gref.as_obj())),
        None => {
            let class: JClass = env
                .find_class(cls)?;

            cache.insert(
                String::from(cls),
                env.new_global_ref(class)?,
            );
            Ok(class)
        }
    }
}

pub fn get_smid<'a>(env: &'a JNIEnv, cls: &str, method: &str, sig: &str) -> Result<JStaticMethodID<'a>>
{
    let cache = &mut *STATIC_METHOD_ID_CACHE.lock()?;
    let identifier = &*format!("{}.{}{}", cls, method, sig);

    match cache.get(identifier) {
        Some(smid) => Ok(smid.0),

        None => {
            let class = get_cls(env, cls)?;
            let smid = env.get_static_method_id(class, method, sig)?;
            let snmid = JStaticMethodID::from(smid.into_inner());
            cache.insert(String::from(identifier), SendPtr(snmid));
            Ok(smid)
        }
    }
}

pub fn get_mid<'a>(env: &'a JNIEnv, cls: &str, method: &str, sig: &str) -> Result<JMethodID<'a>>
{
    let cache = &mut *METHOD_ID_CACHE.lock()?;
    let identifier = &*format!("{}.{}{}", cls, method, sig);

    match cache.get(identifier) {
        Some(mid) => Ok(mid.0),

        None => {
            let class = get_cls(env, cls)?;
            // bypass lifetime restriction by transmutation
            let mid = env.get_method_id(class, method, sig)?;
            let nmid = JMethodID::from(mid.into_inner());
            cache.insert(String::from(identifier), SendPtr(nmid));
            Ok(mid)
        }
    }
}

pub fn get_field_id<'a>(env: &'a JNIEnv, cls: &str, field: &str, sig: &str) -> Result<JFieldID<'a>>
{
    let cache = &mut *FIELD_ID_CACHE.lock()?;
    let identifier = &*format!("{}.{}.{}", cls, field, sig);

    match cache.get(identifier) {
        Some(fid) => {
            Ok(fid.0)
        }

        None => {
            let class = get_cls(env, cls)?;
            let field_id = env.get_field_id(class, field, sig)?;
            let nfield_id = JFieldID::from(field_id.into_inner());
            cache.insert(String::from(identifier), SendPtr(nfield_id));

            Ok(field_id)
        }
    }
}

// wrapper to cache and get field
pub fn get_field<'a>(env: &'a JNIEnv, cls: &str, obj: &'a JObject, field: &str, ty: &str) -> Result<JValue<'a>>
{
    let field_id = get_field_id(env, cls, field, ty)?;
    let parsed = JavaType::from_str(ty)?;

    Ok(env.get_field_unchecked(*obj, field_id, parsed)?)
}

pub fn get_static_field_id<'a>(env: &'a JNIEnv, cls: &str, field: &str, sig: &str) -> Result<JStaticFieldID<'a>>
{
    let cache = &mut *STATIC_FIELD_ID_CACHE.lock()?;
    let parsed = JavaType::from_str(sig.as_ref())?;

    let identifier = &*format!("{}.{}{}", cls, field, sig);

    let class = get_cls(env, cls)?;

    match cache.get(identifier) {
        Some(sfid) => {
            Ok(sfid.0)
        }

        None => {
            let field_id = env.get_static_field_id(class, field, sig)?;

            return Ok(field_id);
        }
    }
}

// wrapper to cache and get static field
pub fn get_static_field<'a>(env: &'a JNIEnv, cls: &str, obj: &'a JObject, field: &str, ty: &str) -> Result<JValue<'a>>
{
    let class = get_cls(env, cls)?;
    let field_id = get_static_field_id(env, cls, field, ty)?;
    let parsed = JavaType::from_str(ty.as_ref())?;

    Ok(env.get_static_field_unchecked(class, field_id, parsed)?)
}

pub fn set_field(env: &JNIEnv, cls: &str, obj: &JObject, field: &str, sig: &str, value: JValue) -> Result<()> {
    let field_id = get_field_id(env, cls, field, sig)?;
    env.set_field_unchecked(*obj, field_id, value)?;
    Ok(())
}

// These had to be modified to work with kotlin
pub fn set_rust_field<R>(env: &JNIEnv, cls: &str, obj: &JObject, field: &str, rust_object: R) -> Result<()>
where
    R: Send + 'static
{
    let _ = env.lock_obj(*obj)?;

    // Check to see if we've already set this value. If it's not null, that
    // means that we're going to leak memory if it gets overwritten.
    let handle_field = get_field(env, cls, obj, field, "Ljava/lang/Long;")?.l()?;
    if !handle_field.is_null() {
        error!("cacher::set_rust_field:: field {} already set", field.to_owned());
        return Err(Box::new(jni::errors::Error::FieldAlreadySet(field.to_owned())));
    }

    let mbox = Box::new(::std::sync::Mutex::new(rust_object));
    let ptr: *mut Mutex<R> = Box::into_raw(mbox);

    let class = get_cls(env, "java/lang/Long")?;
    let jlong = env.new_object(class, "(J)V", &[(ptr as jni::sys::jlong).into()])?;

    set_field(env, cls, obj, field, "Ljava/lang/Long;", JValue::from(jlong))?;
    Ok(())
}

// These had to be modified to work with kotlin
pub fn get_rust_field<R>(env: &JNIEnv, cls: &str, obj: &JObject, field: &str) -> Result<MutexGuard<'static, R>>
    where
        R: Send + 'static,
{
    let _ = env.lock_obj(*obj)?;

    let j_obj = get_field(env, cls, obj, field, "Ljava/lang/Long;")?.l()?;
    let ptr = get_field(env, "java/lang/Long", &j_obj, "value", "J")?.j()? as *mut Mutex<R>;

    if j_obj.is_null() {
        error!("cacher::get_rust_field:: field {} is null", field.to_owned());
        return Err(
            Box::new(CacherError::new(&*format!("field {} was null", field)))
        );
    }

    unsafe {
        // dereferencing is safe, because we checked it for null
        Ok((*ptr).lock().unwrap())
    }
}

// take back the rust object from jvm
// kotlin edition
pub fn take_rust_field<'a, R>(env: &JNIEnv, cls: &str, obj: &JObject, field: &str) -> Result<R>
    where
        R: Send + 'static,
{
    let mbox = {
        let _ = env.lock_obj(*obj)?;

        let j_obj = get_field(env, cls, obj, field, "Ljava/lang/Long;")?.l()?;
        let ptr = get_field(env, "java/lang/Long", &j_obj, "value", "J")?.j()? as *mut Mutex<R>;

        if ptr.is_null() {
            error!("cacher::take_rust_field:: field {} is null", field.to_owned());
            return Err(
                Box::new(CacherError::new(&*format!("field {} was null", field)))
            );
        }

        let mbox = unsafe { Box::from_raw(ptr) };

        // attempt to acquire the lock. This prevents us from consuming the
        // mutex if there's an outstanding lock. No one else will be able to
        // get a new one as long as we're in the guarded scope.
        drop(mbox.try_lock().unwrap());

        set_field(env, cls, obj, field, "Ljava/lang/Long;", JValue::from(::std::ptr::null_mut() as jobject))?;

        mbox
    };

    Ok(mbox.into_inner().unwrap())
}
