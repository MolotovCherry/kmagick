# KMagick Native Library

Built with love using Rust. ♥️

## How to build

### Windows
- Download and install the latest ImageMagick DLL version
- Add the imagemagick program directory (containing the DLL's) to your `PATH`
- Install LLVM (clang)
- Add LLVM to your `PATH` (e.g. `C:\Program Files\LLVM\bin`)
- Open up `build-win.ps1` in an editor, and make sure the im directory path is correct
- Open powershell and run `build-win.ps1`

The build script has several flags you can use.  
`-release` build as release (default will build as debug)  
`-expand` to use cargo expand to see the generated output  

### Android
- Download the entire [Android-ImageMagick](https://github.com/cherryleafroad/Android-ImageMagick7) repo
- Place this kmagick repo in the Android-Imagemagick repo folder
- Download the [latest Android-Imagemagick shared lib release](https://github.com/cherryleafroad/Android-ImageMagick7/releases)
- Make a `jniLibs` folder in the Android-Imagemagick repo root
- Place the shared libs in the `jniLibs` folder (structure should look like `Android-Imagemagick/jniLibs/arm64-v8a/*.so`)
- Download and install the [Android ndk](https://developer.android.com/ndk/downloads)
- Create env vars for Android NDK: (note, this is not a script, only pseudocode)
```
$ndkRoot = "path/to/android-ndk-r22b"
ANDROID_NDK_HOME=$ndkRoot
NDK_HOME=$ndkRoot
PATH=$PATH:$ndkRoot
CLANG_PATH=$ndkRoot\toolchains\llvm\prebuilt\windows-x86_64\bin\clang.exe
PATH=$PATH:\$ndkRoot\toolchains\llvm\prebuilt\windows-x86_64\bin\
```
- Open powershell and run `build-android.ps1`

The build script has several flags you can use.  
`-release` build as release (default will build as debug)  
`-expand` to use cargo expand to see the generated output

## Rust devs - a note for you all
The two crates `jni-macros` and `jni-tools` offers some FULLY working macros which generate JNI bindings for Rust functions. Of course, it's only Kotlin compatible (no Java; although you _could_ edit the handle fn's to fix that). You can even use regular impl's which KEEP STATE between calls as if it was a real class instance! The `jni-tools` crate offers the visible public API for it. There's also docs on it to explain its usage, however, if you want to fully know how to use it, you should take a look at my Rust and Kotlin code as the prime example.

### Requirements
As these generate binding functions, they use the dependency crates you have.  
You need the following crates imported for it to work:  
`log`  
`jni`

### Macros

`#[jmethod]` - Wraps a function to use in jni
```
    Usage: #[jmethod(cls="some/java/cls", exc="some/exception/ExcCls")]

      Note: the first two parameters are always `env: JNIEnv, obj: JObject`. These
            MUST ALWAYS be `env: JNIEnv, obj: JObject`. If your fn accepts
            more than these 2 parameters, you MUST include the first two regardless.
            If you don't want one or both of them, you can elide them. `_: JNIEnv, _: JObject`.
            If your fn takes these 2 or less parameters, you can omit the variable entirely.

      For example, these are all valid signatures:
        - fn foo(env: JNIEnv, obj: JObject, ...) {}
        - fn foo(_: JNIEnv, obj: JObject, ...) {}
        - fn foo(env: JNIEnv, _: JObject, ...) {}
        - fn foo(_: JNIEnv, _: JObject, ...) {}
        - fn foo(env: JNIEnv, obj: JObject) {}
        - fn foo(_: JNIEnv, obj: JObject) {}
        - fn foo(env: JNIEnv, _: JObject) {}
        - fn foo(_: JNIEnv, _: JObject) {}
        - fn foo(env: JNIEnv) {}
        - fn foo(_: JNIEnv) {}
        - fn foo() {}

      Allowed fn argument types:
        - jobject, jclass, jthrowable, jstring, jarray, jbooleanArray,
          jbyteArray, jcharArray, jshortArray, jintArray, jlongArray,
          jfloatArray, jdoubleArray, jobjectArray, jweak, jint, jlong,
          jbyte, jboolean, jchar, jshort, jfloat, jdouble, jsize,
          jfieldID, jmethodID, JByteBuffer, JClass, JFieldID, JList, JMap,
          JMethodID, JObject, JStaticFieldID, JStaticMethodID, JString,
          JThrowable, JValue
        - Special types:
            JNIEnv - first param must be this
            JNIObject - second param must be this
            JClass - second param can also be this (for a static fn)

      Allowed return types:
        - jarray, jboolean, jbooleanArray, jbyte, jbyteArray,
          jchar, jcharArray, jclass, jdouble, jdoubleArray,
          jfieldID, jfloat, jfloatArray, jint, jintArray, jlong,
          jlongArray, jmethodID, jobject, jobjectArray, jshort, jshortArray,
          jsize, jstring, jthrowable, jweak
        - Special types:
            Self - only allowed from jnew
            jni_tools::JNIResult<> - a normal result type. Any result you want to return MUST
                                     use this special type. You can use any of the valid regular
                                     types inside of this, including ()
            () - ZST can be used inside JNIResult (can also be used outside, but why would you?)


* `cls` - the Kotlin class this method belongs to. This is required.
* `exc` - exception to use if fn returns a JNIResult and it fails. This is optional.
          If missing, `java/lang/RuntimeException` will be used.
* `name` - rename the function to this name on the JNI side. This is optional.
           If missing, the fn name will be used.
```

`#[jclass]` - Wraps an entire impl's fn's for jni.
```
    Usage: #[jclass(pkg="some/java/pkg", exc="some/exception/Cls")]

      Note: the first two parameters are always `env: JNIEnv, obj: JObject`. These
            MUST ALWAYS be `env: JNIEnv, obj: JObject`. If your fn accepts
            more than these 2 parameters, you MUST include the first two regardless.
            If you don't want one or both of them, you can elide them. `_: JNIEnv, _: JObject`.
            If your fn takes these 2 or less parameters, you can omit the variable entirely.

      For example, these are all valid signatures:
        - fn foo(&self, env: JNIEnv, obj: JObject, ...) {}
        - fn foo(&self, _: JNIEnv, obj: JObject, ...) {}
        - fn foo(&self, env: JNIEnv, _: JObject, ...) {}
        - fn foo(&self, _: JNIEnv, _: JObject, ...) {}
        - fn foo(&self, env: JNIEnv, obj: JObject) {}
        - fn foo(&self, _: JNIEnv, obj: JObject) {}
        - fn foo(&self, env: JNIEnv, _: JObject) {}
        - fn foo(&self, _: JNIEnv, _: JObject) {}
        - fn foo(&self, env: JNIEnv) {}
        - fn foo(&self, _: JNIEnv) {}
        - fn foo(&self) {}

      Allowed fn argument types:
        - jobject, jclass, jthrowable, jstring, jarray, jbooleanArray,
          jbyteArray, jcharArray, jshortArray, jintArray, jlongArray,
          jfloatArray, jdoubleArray, jobjectArray, jweak, jint, jlong,
          jbyte, jboolean, jchar, jshort, jfloat, jdouble, jsize,
          jfieldID, jmethodID, JByteBuffer, JClass, JFieldID, JList, JMap,
          JMethodID, JObject, JStaticFieldID, JStaticMethodID, JString,
          JThrowable, JValue
        - Special types:
            JNIEnv - first param must be this
            JNIObject - second param must be this
            JClass - second param can also be this (for a static fn)

      Allowed return types:
        - jarray, jboolean, jbooleanArray, jbyte, jbyteArray,
          jchar, jcharArray, jclass, jdouble, jdoubleArray,
          jfieldID, jfloat, jfloatArray, jint, jintArray, jlong,
          jlongArray, jmethodID, jobject, jobjectArray, jshort, jshortArray,
          jsize, jstring, jthrowable, jweak
        - Special types:
            Self - only allowed from jnew
            jni_tools::JNIResult<> - a normal result type. Any result you want to return MUST
                                     use this special type. You can use any of the valid regular
                                     types inside of this, including ()
            () - ZST can be used inside JNIResult (can also be used outside, but why would you?)


* `cls` - the Kotlin class this method belongs to. Either this or `pkg` are required.
* `pkg` - the Kotlin pkg these fn's belong to. Either this or `cls` are required.
          the class name used will be the same as the impl's name.
* `exc` - exception to use if fn returns a JNIResult and it fails. This is optional.
          If missing, `java/lang/RuntimeException` will be used.
```

`#[jname]` - Change the fn name of the generated binding that java sees.
Only used for impl statements; for jmethod, use name attribute instead.
```
* `name` - the name to use. This is required.
```

`#[jignore]` - Ignore this fn and don't generate an implementation for it.

`#[jstatic]` - For use in impls. Call as static function instead of instance function.

`#[jdestroy]` - Handle the destruction of instance. Takes the object out of the handle allowing it to be dropped.

`#[jnew]` - Create a new instance, and set the handle.

`#[jget]` - In a regular instance function, it will get the handle to the second parameter `obj`.
Using this, you can change the object it grabs the handle to.
```
* `from` - the obj variable to use. Must be a JObject.
```

`#[jset]` - When using jnew, in regular usage, it will set the handle to the second parameter `obj`.
You can change the variable used to set the handle to a different one.
```
* `to` - the obj variable to use. Must be a JObject.
```

`#[jtake]` - When using jdestroy, it will take the handle to the second parameter `obj`.
You can change the variable whose handle is taken with this.
```
* `from` - the obj variable to use. Must be a JObject.
```
