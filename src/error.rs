use std::fmt::{Debug, Display, Formatter};

pub type Result<T,E = Error> = std::result::Result<T,E>;
pub type StdError = dyn std::error::Error;
pub type BoxError = Box<StdError>;
pub type IoError = std::io::Error;

pub enum Error {
    Custom(BoxError),
    Io(IoError),
}

impl<E> From<E> for Error where E: std::error::Error + 'static {
    fn from(value: E) -> Self {
        Self::Custom(Box::new(value))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(error) => Display::fmt(&error, f),
            Error::Io(error) => {
                write!(f, "IO Error: ")?;
                Display::fmt(&error, f)
            },
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(error) => Debug::fmt(&error, f),
            Error::Io(error) => {
                writeln!(f, "IO Error:")?;
                Debug::fmt(&error, f)
            },
        }
    }
}


