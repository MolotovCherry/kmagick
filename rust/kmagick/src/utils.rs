use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;

#[derive(Error, Debug)]
pub enum JNIError {
    #[error("JNI runtime exception occurred: {0}")]
    RuntimeException(String)
}

pub fn runtime_exception<T, S>(string: S) -> Result<T>
    where S: AsRef<str> + ToOwned
{
    Err(
        Box::new(
            JNIError::RuntimeException(
                string.as_ref().to_owned()
            )
        )
    )
}

pub trait EnumIntConversion {
    type Output;

    fn try_from_int(val: i32) -> Result<Self::Output>;
}
