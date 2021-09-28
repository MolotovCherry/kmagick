use jni::JNIEnv;
use jni::objects::JObject;
use jni::objects::JString;
use super::magick_wand::MagickWand;

use super::env::Utils;
use super::macros;

struct Object {
    foo: String
}
impl Object {
    fn print(&self) {
        println!("{}", self.foo);
    }
}

#[no_mangle]
pub extern fn Java_com_cherryleafroad_kmagick_Test_new(env: JNIEnv, obj: JObject) {
    magick_rust::magick_wand_genesis();
    let a = MagickWand::new();
    
    env.set_magickwand_handle(obj, a);
}

#[no_mangle]
pub extern fn Java_com_cherryleafroad_kmagick_Test_test(env: JNIEnv, obj: JObject) {
    let b = env.get_magickwand_handle(obj);
    b.unwrap().is_wand();
}

#[no_mangle]
pub extern fn Java_com_cherryleafroad_kmagick_Test_take(env: JNIEnv, obj: JObject) {
    env.take_magickwand_handle(obj);
}

#[no_mangle]
pub extern fn Java_com_cherryleafroad_kmagick_Test_free(env: JNIEnv, obj: JObject, test: JString) {
    println!("{}", env.get_jstring(test));
}

#[no_mangle]
pub extern fn Java_com_cherryleafroad_kmagick_Test_destroy(env: JNIEnv, obj: JObject) {
    println!("destroy");
    magick_rust::magick_wand_terminus();
}
