use anyhow::Error;
use anyhow::Ok;

pub trait ErrCast<T> {
    fn to_err(self) -> Result<T, Error>;
}

impl<T> ErrCast<T> for Option<T> {
    fn to_err(self) -> Result<T, Error> {
        match self {
            Some(t) => Ok(t),
            None => {
                Err(Error::msg("`Option<T>` did not provite value `T`"))
            }
        }
    }
}
