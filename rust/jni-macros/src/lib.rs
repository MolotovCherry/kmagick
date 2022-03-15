use proc_macro::TokenStream;

#[macro_use]
mod macros;
mod utils;
mod parser;
mod jmethod;
mod jclass;


///
/// Wrap a function to use in jni.
///
///     Usage: #[jmethod(cls="some/java/cls", exc="some/exception/ExcCls")]
///
///       Note: the first two parameters are always `env: JNIEnv, obj: JObject`. These
///             MUST ALWAYS be `env: JNIEnv, obj: JObject`. If your fn accepts
///             more than these 2 parameters, you MUST include the first two regardless.
///             If you don't want one or both of them, you can elide them. `_: JNIEnv, _: JObject`.
///             If your fn takes these 2 or less parameters, you can omit the variable entirely.
///
///       For example, these are all valid signatures:
///         - fn foo(env: JNIEnv, obj: JObject, ...) {}
///         - fn foo(_: JNIEnv, obj: JObject, ...) {}
///         - fn foo(env: JNIEnv, _: JObject, ...) {}
///         - fn foo(_: JNIEnv, _: JObject, ...) {}
///         - fn foo(env: JNIEnv, obj: JObject) {}
///         - fn foo(_: JNIEnv, obj: JObject) {}
///         - fn foo(env: JNIEnv, _: JObject) {}
///         - fn foo(_: JNIEnv, _: JObject) {}
///         - fn foo(env: JNIEnv) {}
///         - fn foo(_: JNIEnv) {}
///         - fn foo() {}
///
///       Allowed fn argument types:
///         - jobject, jclass, jthrowable, jstring, jarray, jbooleanArray,
///           jbyteArray, jcharArray, jshortArray, jintArray, jlongArray,
///           jfloatArray, jdoubleArray, jobjectArray, jweak, jint, jlong,
///           jbyte, jboolean, jchar, jshort, jfloat, jdouble, jsize,
///           jfieldID, jmethodID, JByteBuffer, JClass, JFieldID, JList, JMap,
///           JMethodID, JObject, JStaticFieldID, JStaticMethodID, JString,
///           JThrowable, JValue
///         - Special types:
///             JNIEnv - first param must be this
///             JNIObject - second param must be this
///             JClass - second param can also be this (for a static fn)
///
///       Allowed return types:
///         - jarray, jboolean, jbooleanArray, jbyte, jbyteArray,
///           jchar, jcharArray, jclass, jdouble, jdoubleArray,
///           jfieldID, jfloat, jfloatArray, jint, jintArray, jlong,
///           jlongArray, jmethodID, jobject, jobjectArray, jshort, jshortArray,
///           jsize, jstring, jthrowable, jweak
///         - Special types:
///             Self - only allowed from jnew
///             jni_tools::JNIResult<> - a normal result type. Any result you want to return MUST
///                                      use this special type. You can use any of the valid regular
///                                      types inside of this, including ()
///             () - ZST can be used inside JNIResult (can also be used outside, but why would you?)
///
///       Safety:
///         A wrapper is generated which uses `panic::catch_unwind` to keep it from unwinding beyond
///         ffi boundaries. Any panics will get caught and generate a nice exception in java instead of
///         crashing everything. Additionally, it also handles a result from the user function and throws
///         a java exception if the result failed. It is very unlikely for this to not catch something and
///         actually crash jvm. However, this will not catch abort panics, only unwind panics.
///
/// * `cls` - the Kotlin class this method belongs to. This is required.
/// * `exc` - exception to use if fn returns a JNIResult and it fails. This is optional.
///           If missing, `java/lang/RuntimeException` will be used.
/// * `name` - rename the function to this name on the JNI side. This is optional.
///            If missing, the fn name will be used.
///
#[proc_macro_attribute]
pub fn jmethod(attr: TokenStream, item: TokenStream) -> TokenStream {
    jmethod::jmethod_internal(attr, item)
}

///
/// Wrap an entire impl's fn's for jni.
///
///     Usage: #[jclass(pkg="some/java/pkg", exc="some/exception/Cls")]
///
///       Note: the first two parameters are always `env: JNIEnv, obj: JObject`. These
///             MUST ALWAYS be `env: JNIEnv, obj: JObject`. If your fn accepts
///             more than these 2 parameters, you MUST include the first two regardless.
///             If you don't want one or both of them, you can elide them. `_: JNIEnv, _: JObject`.
///             If your fn takes these 2 or less parameters, you can omit the variable entirely.
///
///       For example, these are all valid signatures:
///         - fn foo(&self, env: JNIEnv, obj: JObject, ...) {}
///         - fn foo(&self, _: JNIEnv, obj: JObject, ...) {}
///         - fn foo(&self, env: JNIEnv, _: JObject, ...) {}
///         - fn foo(&self, _: JNIEnv, _: JObject, ...) {}
///         - fn foo(&self, env: JNIEnv, obj: JObject) {}
///         - fn foo(&self, _: JNIEnv, obj: JObject) {}
///         - fn foo(&self, env: JNIEnv, _: JObject) {}
///         - fn foo(&self, _: JNIEnv, _: JObject) {}
///         - fn foo(&self, env: JNIEnv) {}
///         - fn foo(&self, _: JNIEnv) {}
///         - fn foo(&self) {}
///
///       Allowed fn argument types:
///         - jobject, jclass, jthrowable, jstring, jarray, jbooleanArray,
///           jbyteArray, jcharArray, jshortArray, jintArray, jlongArray,
///           jfloatArray, jdoubleArray, jobjectArray, jweak, jint, jlong,
///           jbyte, jboolean, jchar, jshort, jfloat, jdouble, jsize,
///           jfieldID, jmethodID, JByteBuffer, JClass, JFieldID, JList, JMap,
///           JMethodID, JObject, JStaticFieldID, JStaticMethodID, JString,
///           JThrowable, JValue
///         - Special types:
///             JNIEnv - first param must be this
///             JNIObject - second param must be this
///             JClass - second param can also be this (for a static fn)
///
///       Allowed return types:
///         - jarray, jboolean, jbooleanArray, jbyte, jbyteArray,
///           jchar, jcharArray, jclass, jdouble, jdoubleArray,
///           jfieldID, jfloat, jfloatArray, jint, jintArray, jlong,
///           jlongArray, jmethodID, jobject, jobjectArray, jshort, jshortArray,
///           jsize, jstring, jthrowable, jweak
///         - Special types:
///             Self - only allowed from jnew
///             jni_tools::JNIResult<> - a normal result type. Any result you want to return MUST
///                                      use this special type. You can use any of the valid regular
///                                      types inside of this, including ()
///             () - ZST can be used inside JNIResult (can also be used outside, but why would you?)
///
///       Safety:
///         A wrapper is generated which uses `panic::catch_unwind` to keep it from unwinding beyond
///         ffi boundaries. Any panics will get caught and generate a nice exception in java instead of
///         crashing everything. Additionally, it also handles a result from the user function and throws
///         a java exception if the result failed. It is very unlikely for this to not catch something and
///         actually crash jvm. However, this will not catch abort panics, only unwind panics.
///
/// * `cls` - the Kotlin class this method belongs to. Either this or `pkg` are required.
/// * `pkg` - the Kotlin pkg these fn's belong to. Either this or `cls` are required.
///           the class name used will be the same as the impl's name.
/// * `exc` - exception to use if fn returns a JNIResult and it fails. This is optional.
///           If missing, `java/lang/RuntimeException` will be used.
///
#[proc_macro_attribute]
pub fn jclass(attr: TokenStream, item: TokenStream) -> TokenStream {
    jclass::jclass_internal(attr, item)
}

/// Change the fn name of the generated binding that java sees.
/// Only used for impl statements; for jmethod, use name attribute instead.
///
/// * `name` - the name to use. This is required.
#[proc_macro_attribute]
pub fn jname(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

/// Ignore this fn and don't generate an implementation for it.
#[proc_macro_attribute]
pub fn jignore(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

/// For use in impls. Call as static function instead of instance function.
#[proc_macro_attribute]
pub fn jstatic(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

/// Handle the destruction of instance. Takes the object out of the handle allowing it to be dropped.
#[proc_macro_attribute]
pub fn jdestroy(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

/// Create a new instance, and set the handle.
#[proc_macro_attribute]
pub fn jnew(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

/// In a regular instance function, it will get the handle to the second parameter `obj`.
/// Using this, you can change the object it grabs the handle to.
///
/// * `from` - the obj variable to use. Must be a JObject.
#[proc_macro_attribute]
pub fn jget(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

/// When using jnew, in regular usage, it will set the handle to the second parameter `obj`.
/// You can change the variable used to set the handle to a different one.
///
/// * `to` - the obj variable to use. Must be a JObject.
#[proc_macro_attribute]
pub fn jset(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}

/// When using jdestroy, it will take the handle to the second parameter `obj`.
/// You can change the variable whose handle is taken with this.
///
/// * `from` - the obj variable to use. Must be a JObject.
#[proc_macro_attribute]
pub fn jtake(_: TokenStream, item: TokenStream) -> TokenStream {
    // this is a no-op, just here for marker purposes
    item
}
