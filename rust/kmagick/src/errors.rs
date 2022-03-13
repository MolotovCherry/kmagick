use thiserror::Error;

#[derive(Error, Debug)]
pub enum JNIError {
    // To get around some problems with the api requiring 'static str
    #[error("{0}")]
    MagickError(String),

    #[error("JNI runtime exception occurred: {0}")]
    RuntimeException(String)
}
