/// Construct a wand wrapper over Wand types which implements Send
/// and (naturally) deref and deref_mut
macro_rules! wand_wrapper {
    ($name:ident) => {
        struct $name {
            wand: magick_rust::$name
        }

        unsafe impl Send for $name {}

        impl std::ops::Deref for $name {
            type Target = magick_rust::$name;

            fn deref(&self) -> &Self::Target {
                &self.wand
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.wand
            }
        }

        paste::paste! {
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/" $name "Exception")]
            impl $name {
                #[jni_tools::jnew]
                fn new() -> Self {
                    Self {
                        wand: magick_rust::$name::new()
                    }
                }

                #[jni_tools::jname(name="nativeClone")]
                #[jni_tools::jnew]
                fn clone(env: jni::JNIEnv, _: jni::objects::JObject, wand: jni::objects::JObject) -> std::result::Result<Self, Box<dyn std::error::Error>> {
                    use jni_tools::Handle;

                    let r_obj = env.get_handle::<$name>(wand)?;
                    let new_wand = r_obj.clone();

                    return Ok(Self {
                        wand: new_wand
                    })
                }

                fn clearException(&mut self) -> std::result::Result<(), &'static str> {
                    Ok(self.wand.clear_exception()?)
                }

                #[jni_tools::jname(name="nativeGetExceptionType")]
                fn getExceptionType(&self) -> jni::sys::jint {
                    // bindings::ExceptionType == i32 == jint
                    self.wand.get_exception_type() as jni::sys::jint
                }

                fn getException(
                    &self,
                    env: jni::JNIEnv
                ) -> std::result::Result<jni::sys::jobject, Box<dyn std::error::Error>>
                {
                    let exc_res = self.wand.get_exception()?;

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
    }
}
