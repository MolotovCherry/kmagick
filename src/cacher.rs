use std::error::Error;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};
use std::{collections::HashMap, fmt::Display};

use jni::objects::{JFieldID, JStaticFieldID};
use jni::signature::{JavaType, Primitive};
use jni::{
    objects::{GlobalRef, JClass, JMethodID, JObject, JStaticMethodID, JValue},
    strings::JNIString,
    JNIEnv,
};
use lazy_static::lazy_static;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    pub static ref CLASS_CACHE: Mutex<HashMap<String, GlobalRef>> = Mutex::new(HashMap::new());
    pub static ref OBJECT_CLASS_CACHE: Mutex<HashMap<String, GlobalRef>> = Mutex::new(HashMap::new());
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
    let obj_cls_cache = &mut *OBJECT_CLASS_CACHE.lock().unwrap();
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
    obj_cls_cache.clear();

}

pub fn get_cls<'a, C>(env: &'a JNIEnv, cls: &C) -> Result<JClass<'a>>
    where
        C: AsRef<str>
{
    let cache = &mut *CLASS_CACHE.lock().unwrap();
    match cache.get(cls.as_ref()) {
        Some(gref) => Ok(JClass::from(*gref.as_obj())),
        None => {
            let class: JClass = env
                .find_class(cls)
                .expect(&*format!("couldn't find class {}", cls.as_ref()));

            cache.insert(
                String::from(cls.as_ref()),
                env.new_global_ref(class).expect("couldn't get global ref"),
            );
            Ok(class)
        }
    }
}

pub fn get_obj_cls<'a, C>(env: &'a JNIEnv, obj: &JObject, cls: &C) -> Result<JClass<'a>>
    where
        C: AsRef<str>
{
    let cache = &mut *OBJECT_CLASS_CACHE.lock().unwrap();
    match cache.get(cls.as_ref()) {
        Some(gref) => Ok(JClass::from(*gref.as_obj())),
        None => {
            let class: JClass = env
                .get_object_class(*obj)
                .expect("couldn't get object class");
            cache.insert(
                String::from(cls.as_ref()),
                env.new_global_ref(class).expect("couldn't get global ref"),
            );
            Ok(class)
        }
    }
}

pub fn get_smid<'a, C, M, S>(env: &'a JNIEnv, cls: &C, method: &M, sig: &S) -> Result<JStaticMethodID<'a>>
    where
        C: AsRef<str>,
        M: AsRef<str>,
        S: AsRef<str>
{
    let cache = &mut *STATIC_METHOD_ID_CACHE.lock()?;
    let identifier = &*format!("{}.{}{}", cls.as_ref(), method.as_ref(), sig.as_ref());

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

pub fn get_mid<'a, C, M, S>(env: &'a JNIEnv, obj: &JObject, cls: &C, method: &M, sig: &S) -> Result<JMethodID<'a>>
    where 
        C: AsRef<str>,
        M: AsRef<str>,
        S: AsRef<str>
{
    let cache = &mut *METHOD_ID_CACHE.lock()?;
    let identifier = &*format!("{}.{}{}", cls.as_ref(), method.as_ref(), sig.as_ref());

    match cache.get(identifier) {
        Some(mid) => Ok(mid.0),

        None => {
            let class = get_obj_cls(env, obj, cls)?;
            // bypass lifetime restriction by transmutation
            let mid = env.get_method_id(class, method, sig)?;
            let nmid = JMethodID::from(mid.into_inner());
            cache.insert(String::from(identifier), SendPtr(nmid));
            Ok(mid)
        }
    }
}

pub fn get_field_id<'a, C, F, D>(env: &'a JNIEnv, cls: &C, obj: &JObject, field: &F, sig: &D) -> Result<JFieldID<'a>>
    where
        C: AsRef<str> + Display,
        F: AsRef<str> + Display,
        D: AsRef<str> + Display
{
    let cache = &mut *FIELD_ID_CACHE.lock()?;
    let identifier = &*format!("{}.{}{}", cls, field, sig);

    match cache.get(identifier) {
        Some(fid) => {
            Ok(fid.0)
        }

        None => {
            let class = get_obj_cls(env, obj, cls)?;
            let field_id = env.get_field_id(class, field, sig)?;

            Ok(field_id)
        }
    }
}

// wrapper to cache and get field
pub fn get_field<'a, C, F, T>(env: &'a JNIEnv, cls: &C, obj: &'a JObject, field: &F, ty: &T) -> Result<JValue<'a>>
    where
        C: AsRef<str> + Display,
        F: AsRef<str> + Display,
        T: AsRef<str> + Into<JNIString> + Display
{
    let field_id = get_field_id(env, cls, obj, field, ty)?;
    let parsed = JavaType::from_str(ty.as_ref())?;

    Ok(env.get_field_unchecked(*obj, field_id, parsed)?)
}

// wrapper to cache and get static field
pub fn get_static_field<'a, C, F, T>(env: &'a JNIEnv, cls: &C, obj: &'a JObject, field: &F, ty: &T) -> Result<JValue<'a>>
    where
        C: AsRef<str> + Display,
        F: AsRef<str> + Display,
        T: AsRef<str> + Into<JNIString> + Display
{
    let cache = &mut *STATIC_FIELD_ID_CACHE.lock()?;
    let parsed = JavaType::from_str(ty.as_ref())?;

    let identifier = &*format!("{}.{}{}", cls, field, ty);

    let class = get_cls(env, cls)?;

    match cache.get(identifier) {
        Some(sfid) => {
            Ok(env.get_static_field_unchecked(class, sfid.0, parsed)?)
        }

        None => {
            let field_id = env.get_static_field_id(class, field, ty)?;

            Ok(env.get_static_field_unchecked(class, field_id, parsed)?)
        }
    }
}

pub fn set_rust_field<C, S, R, T>(env: &JNIEnv, cls: &C, obj: &JObject, field: &S, ty: &T, rust_object: R) -> Result<()>
where
    C: AsRef<str> + Display,
    S: AsRef<str> + Display,
    R: Send + 'static,
    T: AsRef<str> + Display
{
    let guard = env.lock_obj(*obj)?;

    // Check to see if we've already set this value. If it's not null, that
    // means that we're going to leak memory if it gets overwritten.
    let field_ptr = get_field(env, cls, obj, field, ty)?
        .j()? as *mut Mutex<R>;
    if !field_ptr.is_null() {
        return Err(Box::new(jni::errors::Error::FieldAlreadySet(field.as_ref().to_owned())));
    }

    let mbox = Box::new(::std::sync::Mutex::new(rust_object));
    let ptr: *mut Mutex<R> = Box::into_raw(mbox);


    let field_id = get_field_id(env, cls, obj, field, ty)?;
    env.set_field_unchecked(*obj, field_id, (ptr as jni::sys::jlong).into())?;
    Ok(())
}

pub fn get_rust_field<C, S, R>(env: &JNIEnv, cls: &C, obj: &JObject, field: &S) -> Result<MutexGuard<'static, R>>
    where
        C: AsRef<str> + Display,
        S: Into<JNIString> + AsRef<str> + Display,
        R: Send + 'static,
{
    let guard = env.lock_obj(*obj)?;

    let ptr = get_field(env, cls, obj, field, &"J")?.j()? as *mut Mutex<R>;

    if ptr.is_null() {
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
pub fn take_rust_field<'a, C, S, R>(env: &JNIEnv, cls: &C, obj: &JObject, field: &S) -> Result<R>
    where
        C: AsRef<str> + Display,
        S: AsRef<str> + Display,
        R: Send + 'static,
{
    let field_id = get_field_id(env, cls, obj, field, &"J")?;

    let mbox = {
        let guard = env.lock_obj(*obj)?;

        let ptr = env
            .get_field_unchecked(*obj, field_id, JavaType::Primitive(Primitive::Long))?
            .j()? as *mut Mutex<R>;

        if ptr.is_null() {
            return Err(
                Box::new(CacherError::new(&*format!("field {} was null", field)))
            );
        }

        let mbox = unsafe { Box::from_raw(ptr) };

        // attempt to acquire the lock. This prevents us from consuming the
        // mutex if there's an outstanding lock. No one else will be able to
        // get a new one as long as we're in the guarded scope.
        drop(mbox.try_lock().unwrap());

        env.set_field_unchecked(
            *obj,
            field_id,
            (::std::ptr::null_mut::<()>() as jni::sys::jlong).into(),
        )?;

        mbox
    };

    Ok(mbox.into_inner().unwrap())
}
