/// Construct a wand wrapper over Wand types which implements Send
/// and (naturally) deref and deref_mut.
/// This wrapper acts as if it's the exact same type as magick_rust::wand.
/// You can call methods on it normally, and even access .wand
macro_rules! wand_wrapper {
    ($wand:ident) => {
        struct $wand {
            instance: magick_rust::$wand
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
            impl$wand{
                #[jni_tools::jnew]
                fn new() -> Self {
                    Self {
                        instance: magick_rust::$wand::new()
                    }
                }

                #[jni_tools::jname(name="nativeClone")]
                #[jni_tools::jnew]
                fn clone(env: jni::JNIEnv, _: jni::objects::JObject, wand: jni::objects::JObject) -> super::utils::Result<Self> {
                    use jni_tools::Handle;

                    let r_obj = env.get_handle::<$wand>(wand)?;
                    Ok(r_obj.clone())
                }

                fn clearException(&mut self) -> std::result::Result<(), &'static str> {
                    Ok(self.clear_exception()?)
                }

                #[jni_tools::jname(name="nativeGetExceptionType")]
                fn getExceptionType(&self) -> jni::sys::jint {
                    // bindings::ExceptionType == i32 == jint
                    self.get_exception_type() as jni::sys::jint
                }

                fn getException(
                    &self,
                    env: jni::JNIEnv
                ) -> super::utils::Result<jni::sys::jobject>
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

                #[jni_tools::jdestroy]
                fn destroy(&self) {
                    // object dropped when this scope ends
                }
            }
        }

        impl Clone for $wand {
            fn clone(&self) -> Self {
                Self {
                    instance: self.instance.clone()
                }
            }
        }
    }
}


macro_rules! string_get_set {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident)*
    ) => {
        magick_bindings::magick_bindings!(
            $wand,
            $($get <<= $m_get() -> Result<String>, mut $set <<= $m_set(arg: &str) -> Result<()>,)*
        );
    }
}

macro_rules! get_set_enum {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident, $ty:ty)*
    ) => {
        magick_bindings::magick_bindings!(
            $wand,
            $($get <<= $m_get() -> $ty, mut $set <<= $m_set(arg: int32),)*
        );
    }
}

macro_rules! get_set_type {
    (
        $wand:ident,
        $($get:ident, $set:ident, $m_get:ident, $m_set:ident, $ty:ty)*
    ) => {
        magick_bindings::magick_bindings!(
            $wand,
            $($get <<= $m_get() -> $ty, mut $set <<= $m_set(arg: $ty),)*
        );
    }
}
