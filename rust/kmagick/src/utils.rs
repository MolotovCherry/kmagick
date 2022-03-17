use std::error::Error as StdError;

use crate::errors::JNIError;

pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;

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

pub trait WandId {
    fn id(&self) -> u64;
}
