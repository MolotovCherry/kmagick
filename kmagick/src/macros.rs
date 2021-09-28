/// If in debug mode, sends first param to function. If in release mode, sends 2nd param to function
macro_rules! debug_cond {
    ($a:expr, $b:expr) => {
        if cfg!(debug_assertions) {
            $a
        } else {
            $b
        }
    };
}

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

macro_rules! throw_magick_exc {
    ($env:ident, $m:expr) => {
        {
            let cls = $env.cache_find_class("com/cherryleafroad/kmagick/MagickException").unwrap();
            $env.throw_new(cls, $m).ok();
        }
    }
}

macro_rules! throw_magickwand_exc {
    ($env:ident, $m:expr) => {
        {
            let cls = $env.cache_find_class(&$env, &"com/cherryleafroad/kmagick/MagickWandException").unwrap();
            $env.throw_new(cls, $m).ok();
        }
    }
}

macro_rules! throw_pixelwand_exc {
    ($env:ident, $m:expr) => {
        {
            let cls = $env.cache_find_class($env, "com/cherryleafroad/kmagick/PixelWandException").unwrap();
            $env.throw_new(cls, $m);
        }
    }
}

macro_rules! throw_drawingwand_exc {
    ($env:ident, $m:expr) => {
        {
            let cls = $env.cache_find_class($env, "com/cherryleafroad/kmagick/DrawingWandException").unwrap();
            $env.throw_new(cls, $m).ok();
        }
    }
}
