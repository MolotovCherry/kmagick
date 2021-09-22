/// If in debug mode, sends first param to function. If in release mode, sends 2nd param to function
#[macro_export]
macro_rules! debug_cond {
    ($a:expr, $b:expr) => {
        if cfg!(debug_assertions) {
            $a
        } else {
            $b
        }
    };
}

#[macro_export]
macro_rules! get_jstring {
    ($env:ident, $a:ident) => {
        unsafe {
            ::std::ffi::CString::from(
                ::std::ffi::CStr::from_ptr(
                    $env.get_string($a).unwrap().as_ptr()
                )
            ).to_str().expect("unable to get jstring")
        }
    }
}
