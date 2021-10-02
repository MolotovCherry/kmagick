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
            #[jni_tools::jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad.kmagick/" $name "Exception")]
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

                    let r_obj = env.get_handle::<$name>(concat!("com/cherryleafroad.kmagick/", stringify!($name)), wand)?;
                    let new_wand = r_obj.clone();

                    return Ok(Self {
                        wand: new_wand
                    })
                }

                #[jni_tools::jdestroy]
                fn destroy(&self) {
                    // object dropped when this scope ends
                }
            }
        }
    }
}
