use thiserror::Error;

#[derive(Error, Debug)]
pub enum JNIError {
    #[error("JNI runtime exception occurred: {0}")]
    RuntimeException(String)
}
