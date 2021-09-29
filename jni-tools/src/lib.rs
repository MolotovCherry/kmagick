use std::error::Error;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;

use jni::objects::{JFieldID, JStaticFieldID, JString};
use jni::signature::JavaType;
use jni::{
    objects::{GlobalRef, JClass, JMethodID, JObject, JStaticMethodID, JValue},
    JNIEnv,
};
use lazy_static::lazy_static;
use log::error;

pub mod macros;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

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

#[allow(non_snake_case)]
pub mod CacheClasses {
    pub const LONG: &'static str = "java/lang/Long";
}


pub trait Cacher<'a, O> {
    fn clear_cache(&self);
    fn cache_find_class(&self, cls: &str) -> Result<JClass<'a>>;
    fn cache_get_object_class(&self, cls: &str, obj: O) -> Result<JClass<'a>>;
    fn cache_get_static_method_id(&self, cls: &str, method: &str, sig: &str) -> Result<JStaticMethodID<'a>>;
    fn cache_get_method_id(&self, cls: &str, obj: O, method: &str, sig: &str) -> Result<JMethodID<'a>>;
    fn cache_get_field_id(&self, cls: &str, obj: O, field: &str, sig: &str) -> Result<JFieldID<'a>>;
    fn cache_get_field(&self, cls: &str, obj: O, field: &str, ty: &str) -> Result<JValue<'a>>;
    fn cache_get_static_field_id(&self, cls: &str, field: &str, sig: &str) -> Result<JStaticFieldID<'a>>;
    fn cache_get_static_field(&self, cls: &str, field: &str, ty: &str) -> Result<JValue<'a>>;
    fn cache_set_field(&self, cls: &str, obj: O, field: &str, sig: &str, value: JValue) -> Result<()>;
}

impl<'a> Cacher<'a, JObject<'a>> for JNIEnv<'a> {
    fn clear_cache(&self) {
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

    fn cache_find_class(&self, cls: &str) -> Result<JClass<'a>>
    {
        let cache = &mut *CLASS_CACHE.lock().unwrap();
        match cache.get(cls) {
            Some(gref) => Ok(JClass::from(*gref.as_obj())),
            None => {
                let class: JClass = self.find_class(cls)?;

                cache.insert(
                    String::from(cls),
                    self.new_global_ref(class)?
                );
                Ok(class)
            }
        }
    }

    // unfortunately it'll only work once per object...
    fn cache_get_object_class(&self, cls: &str, obj: JObject) -> Result<JClass<'a>> {
        let cache = &mut *CLASS_CACHE.lock().unwrap();

        match cache.get(cls) {
            Some(gref) => Ok(JClass::from(*gref.as_obj())),
            None => {
                let class = self.get_object_class(obj)?;

                cache.insert(
                    String::from(cls),
                    self.new_global_ref(class)?
                );
                Ok(class)
            }
        }
    }

    fn cache_get_static_method_id(&self, cls: &str, method: &str, sig: &str) -> Result<JStaticMethodID<'a>>
    {
        let cache = &mut *STATIC_METHOD_ID_CACHE.lock()?;
        let identifier = &*format!("{}.{}{}", cls, method, sig);

        match cache.get(identifier) {
            Some(smid) => Ok(smid.0),

            None => {
                let class = self.cache_find_class(cls)?;
                let smid = self.get_static_method_id(class, method, sig)?;
                let snmid = JStaticMethodID::from(smid.into_inner());
                cache.insert(String::from(identifier), SendPtr(snmid));
                Ok(smid)
            }
        }
    }

    fn cache_get_method_id(&self, cls: &str, obj: JObject, method: &str, sig: &str) -> Result<JMethodID<'a>>
    {
        let cache = &mut *METHOD_ID_CACHE.lock()?;
        let identifier = &*format!("{}.{}{}", cls, method, sig);

        match cache.get(identifier) {
            Some(mid) => Ok(mid.0),

            None => {
                let class = self.cache_get_object_class(cls, obj)?;
                // bypass lifetime restriction by transmutation
                let mid = self.get_method_id(class, method, sig)?;
                let nmid = JMethodID::from(mid.into_inner());
                cache.insert(String::from(identifier), SendPtr(nmid));
                Ok(mid)
            }
        }
    }

    fn cache_get_field_id(&self, cls: &str, obj: JObject, field: &str, sig: &str) -> Result<JFieldID<'a>>
    {
        let cache = &mut *FIELD_ID_CACHE.lock()?;
        let identifier = &*format!("{}.{}.{}", cls, field, sig);

        match cache.get(identifier) {
            Some(fid) => Ok(fid.0),

            None => {
                let class = self.cache_get_object_class(cls, obj)?;
                let field_id = self.get_field_id(class, field, sig)?;
                let nfield_id = JFieldID::from(field_id.into_inner());
                cache.insert(String::from(identifier), SendPtr(nfield_id));

                Ok(field_id)
            }
        }
    }

    // wrapper to cache and get field
    fn cache_get_field(&self, cls: &str, obj: JObject<'a>, field: &str, ty: &str) -> Result<JValue<'a>>
    {
        let field_id = self.cache_get_field_id(cls, obj, field, ty)?;
        let parsed = JavaType::from_str(ty)?;

        Ok(self.get_field_unchecked(obj, field_id, parsed)?)
    }

    fn cache_get_static_field_id(&self, cls: &str, field: &str, sig: &str) -> Result<JStaticFieldID<'a>>
    {
        let cache = &mut *STATIC_FIELD_ID_CACHE.lock()?;

        let identifier = &*format!("{}.{}{}", cls, field, sig);

        let class = self.cache_find_class(cls)?;

        match cache.get(identifier) {
            Some(sfid) => Ok(sfid.0),

            None => {
                let field_id = self.get_static_field_id(class, field, sig)?;
                let nfield_id = JStaticFieldID::from(field_id.into_inner());
                cache.insert(
                    String::from(identifier),
                    SendPtr(nfield_id)
                );

                return Ok(field_id);
            }
        }
    }

    // wrapper to cache and get static field
    fn cache_get_static_field(&self, cls: &str, field: &str, ty: &str) -> Result<JValue<'a>>
    {
        let class = self.find_class(cls)?;
        let field_id = self.cache_get_static_field_id(cls, field, ty)?;
        let parsed = JavaType::from_str(ty.as_ref())?;

        Ok(self.get_static_field_unchecked(class, field_id, parsed)?)
    }

    fn cache_set_field(&self, cls: &str, obj: JObject, field: &str, sig: &str, value: JValue) -> Result<()> {
        let field_id = self.cache_get_field_id(cls, obj, field, sig)?;
        self.set_field_unchecked(obj, field_id, value)?;
        Ok(())
    }
}

pub trait Kotlin {
    fn get_rust_field_kt<R>(&self, cls: &str, obj: JObject, field: &str) -> Result<MutexGuard<'static, R>>
        where
            R: Send + 'static;
    fn set_rust_field_kt<R>(&self, cls: &str, obj: JObject, field: &str, rust_object: R) -> Result<()>
        where
            R: Send + 'static;
    fn take_rust_field_kt<'a, R>(&self, cls: &str, obj: JObject, field: &str) -> Result<R>
        where
            R: Send + 'static;
}

impl<'a> Kotlin for JNIEnv<'a> {
    fn get_rust_field_kt<R>(&self, cls: &str, obj: JObject, field: &str) -> Result<MutexGuard<'static, R>>
        where
            R: Send + 'static
    {
        let _ = self.lock_obj(*obj)?;

        let j_obj = self.cache_get_field(cls, obj, field, "Ljava/lang/Long;")?.l()?;
        let ptr = self.cache_get_field(CacheClasses::LONG, j_obj, "value", "J")?.j()? as *mut Mutex<R>;

        if j_obj.is_null() {
            error!("env::get_rust_field:: field {} is null", field.to_owned());
            return Err(
                Box::new(CacherError::new(&*format!("field {} was null", field)))
            );
        }

        unsafe {
            // dereferencing is safe, because we checked it for null
            Ok((*ptr).lock().unwrap())
        }
    }

    /// Your field MUST be declared as `private var foo: Long? = null`
    fn set_rust_field_kt<R>(&self, cls: &str, obj: JObject, field: &str, rust_object: R) -> Result<()>
        where
            R: Send + 'static
    {
        let _ = self.lock_obj(*obj)?;

        // Check to see if we've already set this value. If it's not null, that
        // means that we're going to leak memory if it gets overwritten.
        let handle_field = self.cache_get_field(cls, obj, field, "Ljava/lang/Long;")?.l()?;
        if !handle_field.is_null() {
            error!("env::set_rust_field:: field {} already set", field.to_owned());
            return Err(Box::new(jni::errors::Error::FieldAlreadySet(field.to_owned())));
        }

        let mbox = Box::new(::std::sync::Mutex::new(rust_object));
        let ptr: *mut Mutex<R> = Box::into_raw(mbox);

        let class = self.cache_find_class("java/lang/Long")?;
        let jlong = self.new_object(class, "(J)V", &[(ptr as jni::sys::jlong).into()])?;

        self.cache_set_field(cls, obj, field, "Ljava/lang/Long;", JValue::from(jlong))?;
        Ok(())
    }

    fn take_rust_field_kt<R>(&self, cls: &str, obj: JObject, field: &str) -> Result<R>
        where
            R: Send + 'static
    {
        let mbox = {
            let _ = self.lock_obj(*obj)?;

            let j_obj = self.cache_get_field(cls, obj, field, "Ljava/lang/Long;")?.l()?;
            let ptr = self.cache_get_field(CacheClasses::LONG, j_obj, "value", "J")?.j()? as *mut Mutex<R>;

            if ptr.is_null() {
                error!("env::take_rust_field:: field {} is null", field.to_owned());
                return Err(
                    Box::new(CacherError::new(&*format!("field {} was null", field)))
                );
            }

            let mbox = unsafe { Box::from_raw(ptr) };

            // attempt to acquire the lock. This prevents us from consuming the
            // mutex if there's an outstanding lock. No one else will be able to
            // get a new one as long as we're in the guarded scope.
            drop(mbox.try_lock().unwrap());

            self.cache_set_field(cls, obj, field, "Ljava/lang/Long;", JValue::from(JObject::null()))?;

            mbox
        };

        Ok(mbox.into_inner().unwrap())
    }
}

pub trait Utils {
    fn get_jstring(&self, val: JString) -> Result<String>;
}

impl<'a> Utils for JNIEnv<'a> {
    // silently fails if there was an error, but also throws MagickException
    fn get_jstring(&self, val: JString) -> Result<String> {
        Ok(Into::<String>::into(self.get_string(val)?))
    }
}

pub trait Handle {
    fn get_handle<R>(&self, cls: &str, obj: JObject) -> Result<MutexGuard<R>>
        where
            R: Send + 'static;
    fn set_handle<R>(&self, cls: &str, obj: JObject, rust_obj: R) -> Result<()>
        where
            R: Send + 'static;
    fn take_handle<R>(&self, cls: &str, obj: JObject) -> Result<R>
        where
            R: Send + 'static;
}

impl<'a> Handle for JNIEnv<'a> {
    fn get_handle<R>(&self, cls: &str, obj: JObject) -> Result<MutexGuard<R>>
        where
            R: Send + 'static
    {
        Ok(self.get_rust_field_kt::<R>(cls, obj, "handle")?)
    }

    fn set_handle<R>(&self, cls: &str, obj: JObject, rust_object: R) -> Result<()>
        where
            R: Send + 'static
    {
        Ok(self.set_rust_field_kt(cls, obj, "handle", rust_object)?)
    }

    fn take_handle<R>(&self, cls: &str, obj: JObject) -> Result<R>
        where
            R: Send + 'static
    {
        Ok(self.take_rust_field_kt::<R>(cls, obj, "handle")?)
    }
}
