/// Construct a wand wrapper over Wand types which implements Send
/// and (naturally) deref and deref_mut
macro_rules! wand_wrapper {
    ($name:ident) => {
        use std::ops::{Deref, DerefMut};

        pub struct $name {
            wand: magick_rust::$name
        }

        unsafe impl Send for $name {}

        impl Deref for $name {
            type Target = magick_rust::$name;

            fn deref(&self) -> &Self::Target {
                &self.wand
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.wand
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    wand: magick_rust::$name::new()
                }
            }
        }
    }
}
