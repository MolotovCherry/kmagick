// Unwrap Ok or return Syn error
macro_rules! compile_err {
    (
        $input:expr
    ) => {{
        match $input {
            Ok(v) => v,
            Err(e) => return e.to_compile_error().into()
        }
    }}
}
