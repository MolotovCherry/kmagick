/// Construct a wand wrapper over Wand types which implements Send
/// and (naturally) deref and deref_mut.
/// This wrapper acts as if it's the exact same type as magick_rust::wand.
/// You can call methods on it normally, and even access .wand
macro_rules! wand_wrapper {
    ($wand:ident) => {
        pub struct $wand {
            pub instance: magick_rust::$wand,
            pub id: u64
        }

        impl crate::utils::WandId for $wand {
            fn id(&self) -> u64 {
                self.id
            }
        }

        unsafe impl Send for $wand {}

        impl std::ops::Deref for $wand {
            type Target = magick_rust::$wand;

            fn deref(&self) -> &Self::Target {
                &self.instance
            }
        }

        impl std::ops::DerefMut for $wand {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.instance
            }
        }

        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                #[jni_tools::jnew]
                pub fn new(env: jni::JNIEnv, obj: jni::objects::JObject) -> jni_tools::JNIResult<Self> {
                    let cache = &*crate::cache::[<$wand:upper _CACHE>];
                    let id = crate::cache::insert(cache, env.new_global_ref(obj)?, stringify!($wand))?;

                    let _id = bytemuck::cast::<u64, jni::sys::jlong>(id);
                    env.set_field(obj, "_id", "J", jni::objects::JValue::from(_id))?;

                    let res = Self {
                        instance: magick_rust::$wand::new(),
                        id
                    };

                    Ok(res)
                }

                // can't use the from trait since I need more params
                #[jni_tools::jignore]
                pub fn from_wand(env: jni::JNIEnv, obj: jni::objects::JObject, wand: magick_rust::$wand) -> jni_tools::JNIResult<Self> {
                    // this should never fail, so if it does, panicking is probably just as well at this point
                    let cache = &*crate::cache::[<$wand:upper _CACHE>];
                    let id = crate::cache::insert(cache, env.new_global_ref(obj)?, stringify!($wand))?;

                    Ok(Self {
                        instance: wand,
                        id
                    })
                }

                #[jni_tools::jnew]
                fn clone(env: jni::JNIEnv, obj: jni::objects::JObject, wand: jni::objects::JObject) -> jni_tools::JNIResult<Self> {
                    use jni_tools::Handle;

                    let c_wand = env.get_handle::<$wand>(wand)?;
                    Ok($wand::from_wand(env, obj, c_wand.instance.clone())?)
                }

                fn isWand(&self) -> jni::sys::jboolean {
                    match self.is_wand() {
                        Ok(_) => true as jni::sys::jboolean,
                        Err(_) => false as jni::sys::jboolean
                    }
                }

                fn clearException(&mut self) -> jni_tools::JNIResult<()> {
                    let res = self.clear_exception().map_err(|f| f.to_string())?;
                    Ok(res)
                }

                #[jni_tools::jname(name="nativeGetExceptionType")]
                fn getExceptionType(&self) -> jni::sys::jint {
                    // bindings::ExceptionType == i32 == jint
                    self.get_exception_type() as jni::sys::jint
                }

                fn getException(
                    &self,
                    env: jni::JNIEnv
                ) -> jni_tools::JNIResult<jni::sys::jobject>
                {
                    let exc_res = self.get_exception()?;

                    let msg = jni::objects::JValue::Object(
                        jni::objects::JObject::from(
                            env.new_string(exc_res.0)?
                        )
                    );
                    let id = jni::objects::JValue::Int(exc_res.1 as jni::sys::jint);

                    let cls_str = "com/cherryleafroad/kmagick/NativeMagickException$Companion";
                    let cls = env.find_class(cls_str)?;
                    let cls_init_mid = env.get_method_id(cls, "<init>", "()V")?;
                    let obj = env.new_object_unchecked(cls, cls_init_mid, &[])?;

                    let mid = env.get_method_id(
                        cls,
                        "fromNative",
                        "(ILjava/lang/String;)Lcom/cherryleafroad/kmagick/NativeMagickException;"
                    )?;

                    let res = env.call_method_unchecked(
                        obj,
                        mid,
                        jni::signature::JavaType::Object(String::from("Lcom/cherryleafroad/kmagick/NativeMagickException;")),
                        &[id, msg]
                    )?;

                    Ok(res.l()?.into_inner())
                }

                // Everything gets dropped on its own
                #[jni_tools::jdestroy]
                fn destroy(&self, env: jni::JNIEnv) {
                    // item will automatically be taken and dropped
                    // but we need to also remove it from the cache
                    let cache = &*crate::cache::[<$wand:upper _CACHE>];
                    crate::cache::remove::<$wand>(env, cache, self.id);
                }
            }
        }
    }
}

// unfortunately it's impossible to nest macro calls due to the attribute macro
// &str -> String -> JString
macro_rules! get_string {
    (
        $wand:ident,
        $($get:ident, $m_get:ident)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self, env: jni::JNIEnv) -> jni_tools::JNIResult<jni::sys::jobject> {
                        let res = match self.$m_get() {
                            Ok(v) => v,
                            Err(e) => {
                                return if e.0.starts_with(concat!("null ptr returned by ", stringify!($m_get))) {
                                    Ok(std::ptr::null_mut())
                                } else {
                                    crate::utils::runtime_exception(e.0)?
                                };
                            }
                        };

                        Ok(env.new_string(&*res)?.into_inner())
                    }
                )*
            }
        }
    }
}

// &str -> String -> JString
macro_rules! set_string {
    (
        $wand:ident,
        $($set:ident, $m_set:ident)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $set(&mut self, env: jni::JNIEnv, _: jni::objects::JObject, arg: jni::objects::JString) -> jni_tools::JNIResult<()> {
                        use jni_tools::Utils;
                        let arg = env.get_jstring(arg)?;
                        Ok(self.$m_set(&*arg)?)
                    }
                )*
            }
        }
    }
}

// &str -> String -> JString
macro_rules! get_set_string {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self, env: jni::JNIEnv) -> jni_tools::JNIResult<jni::sys::jobject> {
                        let res = match self.$m_get() {
                            Ok(v) => v,
                            Err(e) => {
                                return if e.0.starts_with(concat!("null ptr returned by ", stringify!($m_get))) {
                                    Ok(std::ptr::null_mut())
                                } else {
                                    crate::utils::runtime_exception(e.0)?
                                };
                            }
                        };

                        Ok(env.new_string(&*res)?.into_inner())
                    }

                    fn $set(&mut self, env: jni::JNIEnv, _: jni::objects::JObject, arg: jni::objects::JString) -> jni_tools::JNIResult<()> {
                        use jni_tools::Utils;
                        let arg = env.get_jstring(arg)?;
                        Ok(self.$m_set(&*arg)?)
                    }
                )*
            }
        }
    }
}

macro_rules! res_to_jniresult {
    (
        $res:expr
    ) => {{
        let res = $res;
        match res {
            Ok(v) => Ok(v),
            Err(e) => {
                Err(Box::new(e))
            }
        }
    }}
}

macro_rules! to_jenum {
    (
        $env:ident, $enum:ident, $int:expr
    ) => {{
        let val = jni::objects::JValue::Int($int as jint);

        let cls = $env.find_class(
            concat!("com/cherryleafroad/kmagick/", concat!(stringify!($enum), "$Companion"))
        )?;
        let j_obj = $env.new_object(cls, "()V", &[])?;
        let mid = $env.get_method_id(
            cls,
            "fromNative",
            concat!("(I)Lcom/cherryleafroad/kmagick/", concat!(stringify!($enum), ";"))
        )?;

        $env.call_method_unchecked(
            j_obj,
            mid,
            jni::signature::JavaType::Object(
                concat!("Lcom/cherryleafroad/kmagick/", concat!(stringify!($enum), ";")).into()
            ),
            &[val]
        )?.l()?.into_inner()
    }}
}

// enums operate as i32 values
macro_rules! get_set_enum {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident, $ty:ty)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self, env: jni::JNIEnv) -> jni_tools::JNIResult<jni::sys::jobject> {
                        cfg_if::cfg_if! {
                            if #[cfg(target_os="android")] {
                                use std::convert::TryFrom;
                                let res = i32::try_from(self.$m_get())?;
                            } else {
                                let res = self.$m_get();
                            }
                        }

                        let val = jni::objects::JValue::Int(res);
                        let cls = env.find_class(
                            concat!("com/cherryleafroad/kmagick/", concat!(stringify!($ty), "$Companion"))
                        )?;
                        let j_obj = env.new_object(cls, "()V", &[])?;
                        let mid = env.get_method_id(
                            cls,
                            "fromNative",
                            concat!("(I)Lcom/cherryleafroad/kmagick/", concat!(stringify!($ty), ";"))
                        )?;

                        Ok(env.call_method_unchecked(
                            j_obj,
                            mid,
                            jni::signature::JavaType::Object(
                                concat!("Lcom/cherryleafroad/kmagick/", concat!(stringify!($ty), ";")).into()
                            ),
                            &[val]
                        )?.l()?.into_inner())
                    }

                    #[cfg(not(target_os="android"))]
                    fn $set(&mut self, _: jni::JNIEnv, _: jni::objects::JObject, arg: jni::sys::jint) {
                        self.$m_set(arg);
                    }

                    #[cfg(target_os="android")]
                    fn $set(&mut self, _: jni::JNIEnv, _: jni::objects::JObject, arg: jni::sys::jint) -> jni_tools::JNIResult<()> {
                        use std::convert::TryFrom;
                        let arg = u32::try_from(arg)?;

                        self.$m_set(arg);
                        Ok(())
                    }
                )*
            }
        }
    }
}

// enums operate as i32 values
// TODO: better way to combine these?
macro_rules! get_set_enum_result {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident, $ty:ty)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self, env: jni::JNIEnv) -> jni_tools::JNIResult<jni::sys::jobject> {
                        cfg_if::cfg_if! {
                            if #[cfg(target_os="android")] {
                                use std::convert::TryFrom;
                                let res = i32::try_from(self.$m_get())?;
                            } else {
                                let res = self.$m_get();
                            }
                        }

                        let val = jni::objects::JValue::Int(res);
                        let cls = env.find_class(
                            concat!("com/cherryleafroad/kmagick/", concat!(stringify!($ty), "$Companion"))
                        )?;
                        let j_obj = env.new_object(cls, "()V", &[])?;
                        let mid = env.get_method_id(
                            cls,
                            "fromNative",
                            concat!("(I)Lcom/cherryleafroad/kmagick/", concat!(stringify!($ty), ";"))
                        )?;

                        Ok(env.call_method_unchecked(
                            j_obj,
                            mid,
                            jni::signature::JavaType::Object(
                                concat!("Lcom/cherryleafroad/kmagick/", concat!(stringify!($ty), ";")).into()
                            ),
                            &[val]
                        )?.l()?.into_inner())
                    }

                    fn $set(&mut self, _: jni::JNIEnv, _: jni::objects::JObject, arg: jni::sys::jint) -> jni_tools::JNIResult<()> {
                        cfg_if::cfg_if! {
                            if #[cfg(target_os="android")] {
                                use std::convert::TryFrom;
                                let arg = u32::try_from(arg)?;
                            }
                        }

                        Ok(self.$m_set(arg)?)
                    }
                )*
            }
        }
    }
}

// f64 / jdouble
macro_rules! get_set_double {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self) -> jni::sys::jdouble {
                        self.$m_get()
                    }

                    fn $set(&mut self, _: jni::JNIEnv, _: jni::objects::JObject, arg: jni::sys::jdouble) {
                        self.$m_set(arg);
                    }
                )*
            }
        }
    }
}

// f32 / jfloat (also includes Quantum)
macro_rules! get_set_float {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self) -> jni::sys::jfloat {
                        self.$m_get()
                    }

                    fn $set(&mut self, _: jni::JNIEnv, _: jni::objects::JObject, arg: jni::sys::jfloat) {
                        self.$m_set(arg);
                    }
                )*
            }
        }
    }
}

macro_rules! get_sized {
    (
        $wand:ident,
        $($get:ident, $m_get:ident, $ty:ty)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self) -> jni_tools::JNIResult<jni::sys::jlong> {
                        use std::convert::TryFrom;

                        // i64 == jlong
                        // when we use usize, we need to make sure it fits (or error out)
                        // i32, i64, and u32 will always succeed, but u64 may not fit
                        Ok(i64::try_from(self.$m_get())?)
                    }
                )*
            }
        }
    }
}

// usize and isize <-> jint <-> i32
macro_rules! get_set_sized {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident, $ty:ty)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self) -> jni_tools::JNIResult<jni::sys::jlong> {
                        use std::convert::TryFrom;

                        // i64 == jlong
                        // when we use usize, we need to make sure it fits (or error out)
                        // i32, i64, and u32 will always succeed, but u64 may not fit
                        Ok(i64::try_from(self.$m_get())?)
                    }

                    fn $set(&mut self, _: jni::JNIEnv, _: jni::objects::JObject, arg: jni::sys::jlong) -> jni_tools::JNIResult<()> {
                        use std::convert::TryFrom;

                        // try from i64 -> isize/i32/i64 (will always work)
                        // for usize/u32/u64, may not always fit
                        let arg = $ty::try_from(arg)?;
                        self.$m_set(arg);
                        Ok(())
                    }
                )*
            }
        }
    }
}

// usize and isize <-> jint <-> i32
// annoying I have to make another macro for this
// TODO: better way to combine these?
macro_rules! get_set_sized_result {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident, $ty:ty)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self) -> jni_tools::JNIResult<jni::sys::jlong> {
                        use std::convert::TryFrom;

                        // i64 == jlong
                        // when we use usize, we need to make sure it fits (or error out)
                        // i32, i64, and u32 will always succeed, but u64 may not fit
                        Ok(i64::try_from(self.$m_get())?)
                    }

                    fn $set(&mut self, _: jni::JNIEnv, _: jni::objects::JObject, arg: jni::sys::jlong) -> jni_tools::JNIResult<()> {
                        use std::convert::TryFrom;

                        // try from i64 -> isize/i32/i64 (will always work)
                        // for usize/u32/u64, may not always fit
                        let arg = $ty::try_from(arg)?;
                        Ok(self.$m_set(arg)?)
                    }
                )*
            }
        }
    }
}

// get / set any wand type
macro_rules! get_set_wand {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident, $ty:ty)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self, env: jni::JNIEnv, _: jni::objects::JObject) -> jni_tools::JNIResult<jni::sys::jobject> {
                        use jni_tools::Handle;

                        let res = self.$m_get();

                        let cls = env.find_class(
                            concat!("com/cherryleafroad/kmagick/", concat!(stringify!($ty), "$Companion"))
                        )?;

                        let c_obj = env.new_object(cls, "()V", &[])?;

                        let mid = env.get_method_id(
                            cls,
                            "newInstance",
                            concat!("()Lcom/cherryleafroad/kmagick/", concat!(stringify!($ty), ";"))
                        )?;

                        let n_obj = env.call_method_unchecked(
                            c_obj,
                            mid,
                            jni::signature::JavaType::Object(
                                concat!("Lcom/cherryleafroad/kmagick/", concat!(stringify!($ty), ";")).into()
                            ),
                            &[]
                        )?.l()?;

                        let r_obj = crate::$ty::from_wand(env, n_obj, res)?;
                        env.set_handle(n_obj, r_obj)?;
                        Ok(n_obj.into_inner())
                    }

                    fn $set(&mut self, env: jni::JNIEnv, _: jni::objects::JObject, wand: jni::objects::JObject) -> jni_tools::JNIResult<()> {
                        use jni_tools::Handle;
                        let r_obj = env.get_handle::<crate::$ty>(wand)?;
                        let arg =  &r_obj.instance;
                        self.$m_set(&arg);
                        Ok(())
                    }
                )*
            }
        }
    }
}

macro_rules! magick_enum_int_conversion {
    ($vis:vis enum $name:ident {
        $($vname:ident,)*
    }) => {
        use crate::utils::EnumIntConversion;

        impl EnumIntConversion for magick_rust::$name {
            type Output = magick_rust::$name;

            fn try_from_int(v: i32) -> crate::utils::Result<magick_rust::$name> {
                match v {
                    $(x if x == magick_rust::$name::$vname as i32 => Ok(magick_rust::$name::$vname),)*
                    _ => crate::utils::runtime_exception(concat!(stringify!($name), " failed enum to int conversion")),
                }
            }
        }
    }
}

macro_rules! new_from_wand {
    ($env:ident, $wand:expr, $ty:ident) => {{
        let cls = $env.find_class(
            concat!("com/cherryleafroad/kmagick/", concat!(stringify!($ty), "$Companion"))
        )?;

        let c_obj = $env.new_object(cls, "()V", &[])?;

        let mid = $env.get_method_id(
            cls,
            "newInstance",
            concat!("()Lcom/cherryleafroad/kmagick/", concat!(stringify!($ty), ";"))
        )?;

        let n_obj = $env.call_method_unchecked(
            c_obj,
            mid,
            jni::signature::JavaType::Object(
                concat!("Lcom/cherryleafroad/kmagick/", concat!(stringify!($ty), ";")).into()
            ),
            &[]
        )?.l()?;

        let r_obj = crate::$ty::from_wand($env, n_obj, $wand)?;
        $env.set_handle(n_obj, r_obj)?;

        n_obj
    }}
}

macro_rules! simple_call {
    (
        $wand:ident,
        $($get:ident, $m_get:ident)*
    ) => {
        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $wand "Exception")]
            impl $wand {
                $(
                    fn $get(&self) -> jni_tools::JNIResult<()> {
                        Ok(self.$m_get()?)
                    }
                )*
            }
        }
    }
}
