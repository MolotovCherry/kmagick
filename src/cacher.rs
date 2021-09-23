use lazy_static::lazy_static;

use std::sync::Mutex;
use jni::{
    JNIEnv,
    objects::{
        GlobalRef, JStaticMethodID, JMethodID, JClass, JObject
    },
    signature::{
        JavaType, Primitive
    }
};
use std::collections::HashMap;
use jni::objects::{JFieldID, JStaticFieldID, JValue};

lazy_static! {
    pub static ref CLASS_CACHE: Mutex<HashMap<String, GlobalRef>> = Mutex::new(HashMap::new());
    pub static ref METHOD_ID_CACHE: Mutex<HashMap<String, SendPtr<JMethodID<'static>>>> = Mutex::new(HashMap::new());
    pub static ref STATIC_METHOD_ID_CACHE: Mutex<HashMap<String, SendPtr<JStaticMethodID<'static>>>> = Mutex::new(HashMap::new());
}

// A wrapper to allow send+sync for mutex on JMethod types
pub struct SendPtr<T>(T);
unsafe impl<T> Send for SendPtr<T> {}
unsafe impl<T> Sync for SendPtr<T> {}

pub fn get_cls<'a>(env: &'a JNIEnv, cls: &str) -> JClass<'a> {
    let mut cache = &mut *CLASS_CACHE.lock().unwrap();
    match cache.get(cls) {
        Some(gref) => {
            JClass::from(*gref.as_obj())
        }
        None => {
            let class: JClass = env.find_class(cls)
                .expect(&*format!("couldn't find class {}", cls));

            cache.insert(
                String::from(cls),
                env.new_global_ref(class).expect("couldn't get global ref")
            );
            class
        }
    }
}

pub fn get_obj_cls<'a>(env: &'a JNIEnv, obj: &JObject, cls: &str) -> JClass<'a> {
    let mut cache = &mut *CLASS_CACHE.lock().unwrap();
    let identifier = &*format!("{}__object", cls);
    match cache.get(identifier) {
        Some(gref) => {
            JClass::from(*gref.as_obj())
        }
        None => {
            let class: JClass = env.get_object_class(*obj).expect("couldn't get object class");
            cache.insert(
                String::from(identifier),
                env.new_global_ref(class).expect("couldn't get global ref")
            );
            class
        }
    }
}

pub fn get_smid<'a>(env: &'a JNIEnv, cls: &str, method: &str, sig: &str) -> JStaticMethodID<'a> {
    let mut cache = &mut *STATIC_METHOD_ID_CACHE.lock().unwrap();
    let identifier = &*format!("{}.{}{}", cls, method, sig);

    match cache.get(identifier) {
        Some(smid) => smid.0,

        None => {
            let class = get_cls(env, cls);
            let smid = env.get_static_method_id(class, method, sig).unwrap();
            let snmid = JStaticMethodID::from(smid.into_inner());
            cache.insert(
                String::from(identifier),
                SendPtr(snmid)
            );
            smid
        }
    }
}

pub fn get_mid<'a>(env: &'a JNIEnv, obj: &JObject, cls: &str, method: &str, sig: &str) -> JMethodID<'a> {
    let mut cache = &mut *METHOD_ID_CACHE.lock().unwrap();
    let identifier = &*format!("{}.{}{}", cls, method, sig);

    match cache.get(identifier) {
        Some(mid) => mid.0,

        None => {
            let class = get_obj_cls(env, obj, cls);
            // bypass lifetime restriction by transmutation
            let mid = env.get_method_id(class, method, sig).unwrap();
            let nmid = JMethodID::from(mid.into_inner());
            cache.insert(String::from(identifier), SendPtr(nmid));
            mid
        }
    }
}
