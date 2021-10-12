use std::error::Error;
use std::sync::{Mutex, MutexGuard};

use jni::objects::JString;
use jni::sys::jobject;
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use log::error;
use thiserror::Error;

pub mod macros;
pub use jni_macros::*;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;


#[allow(non_snake_case)]
pub mod Settings {
    pub const LONG: &'static str = "java/lang/Long";
    pub const LONG_SIG: &'static str = "Ljava/lang/Long;";
    pub const HANDLE: &'static str = "handle";
}


#[derive(Error, Debug)]
enum HandleError {
    #[error("Field `{0}` is null")]
    NullField(String),
    #[error("Field `{0}` is already set")]
    FieldAlreadySet(String)
}

pub trait Kotlin {
    fn get_rust_field_kt<R>(&self, obj: JObject, field: &str) -> Result<MutexGuard<'static, R>>
        where
            R: Send + 'static;
    fn set_rust_field_kt<R>(&self, obj: JObject, field: &str, rust_object: R) -> Result<()>
        where
            R: Send + 'static;
    fn take_rust_field_kt<'a, R>(&self, obj: JObject, field: &str) -> Result<R>
        where
            R: Send + 'static;
}

impl<'a> Kotlin for JNIEnv<'a> {
    fn get_rust_field_kt<R>(&self, obj: JObject, field: &str) -> Result<MutexGuard<'static, R>>
        where
            R: Send + 'static
    {
        let _ = self.lock_obj(*obj)?;

        let j_obj = self.get_field(obj, field, Settings::LONG_SIG)?.l()?;

        if j_obj.is_null() {
            error!("get_rust_field_kt:: field {} is null", field.to_owned());
            return Err(
                Box::new(HandleError::NullField(field.to_owned()))
            );
        }

        let ptr = self.call_method(j_obj, "longValue", "()J", &[])?.j()? as *mut Mutex<R>;

        if ptr.is_null() {
            error!("take_rust_field_kt:: field {} is null", field.to_owned());
            return Err(
                Box::new(HandleError::NullField(field.to_owned()))
            );
        }

        unsafe {
            // dereferencing is safe, because we checked it for null
            Ok((*ptr).lock().unwrap())
        }
    }

    /// Your field MUST be declared as `private var foo: Long? = null`
    fn set_rust_field_kt<R>(&self, obj: JObject, field: &str, rust_object: R) -> Result<()>
        where
            R: Send + 'static
    {
        let _ = self.lock_obj(*obj)?;

        // Check to see if we've already set this value. If it's not null, that
        // means that we're going to leak memory if it gets overwritten.
        let handle_field = self.get_field(obj, field, Settings::LONG_SIG)?.l()?;
        if !handle_field.is_null() {
            error!("set_rust_field:: field {} already set", field.to_owned());
            return Err(Box::new(HandleError::FieldAlreadySet(field.to_owned())));
        }

        let mbox = Box::new(::std::sync::Mutex::new(rust_object));
        let ptr: *mut Mutex<R> = Box::into_raw(mbox);

        let class = self.find_class(Settings::LONG)?;
        let jlong = self.new_object(class, "(J)V", &[(ptr as jni::sys::jlong).into()])?;

        self.set_field(obj, field, Settings::LONG_SIG, JValue::from(jlong))?;
        Ok(())
    }

    fn take_rust_field_kt<R>(&self, obj: JObject, field: &str) -> Result<R>
        where
            R: Send + 'static
    {
        let mbox = {
            let _ = self.lock_obj(*obj)?;

            let j_obj = self.get_field(obj, field, Settings::LONG_SIG)?.l()?;

            if j_obj.is_null() {
                error!("take_rust_field_kt:: field {} is null", field.to_owned());
                return Err(
                    Box::new(HandleError::NullField(field.to_owned()))
                );
            }

            let ptr = self.call_method(j_obj, "longValue", "()J", &[])?.j()? as *mut Mutex<R>;

            if ptr.is_null() {
                error!("take_rust_field_kt:: field {} is null", field.to_owned());
                return Err(
                    Box::new(HandleError::NullField(field.to_owned()))
                );
            }

            let mbox = unsafe { Box::from_raw(ptr) };

            // attempt to acquire the lock. This prevents us from consuming the
            // mutex if there's an outstanding lock. No one else will be able to
            // get a new one as long as we're in the guarded scope.
            drop(mbox.try_lock().unwrap());

            self.set_field(obj, field, Settings::LONG_SIG, JValue::from(::std::ptr::null_mut() as jobject))?;

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
    fn get_handle<R>(&self, obj: JObject) -> Result<MutexGuard<R>>
        where
            R: Send + 'static;
    fn set_handle<R>(&self, obj: JObject, rust_obj: R) -> Result<()>
        where
            R: Send + 'static;
    fn take_handle<R>(&self, obj: JObject) -> Result<R>
        where
            R: Send + 'static;
}

impl<'a> Handle for JNIEnv<'a> {
    fn get_handle<R>(&self, obj: JObject) -> Result<MutexGuard<R>>
        where
            R: Send + 'static
    {
        Ok(self.get_rust_field_kt::<R>(obj, Settings::HANDLE)?)
    }

    fn set_handle<R>(&self, obj: JObject, rust_object: R) -> Result<()>
        where
            R: Send + 'static
    {
        Ok(self.set_rust_field_kt(obj, Settings::HANDLE, rust_object)?)
    }

    fn take_handle<R>(&self, obj: JObject) -> Result<R>
        where
            R: Send + 'static
    {
        Ok(self.take_rust_field_kt::<R>(obj, Settings::HANDLE)?)
    }
}
