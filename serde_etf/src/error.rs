use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    NonFiniteFloat,
    SerializeKey,
    SerializeValue,
    InvalidInput,
    InvalidUtf8,
    NumberTooLarge,
    NumberTooSmall,
    ExtraneousInput,
    ExpectedBool,
    ExpectedInt,
    ExpectedFloat,
    ExpectedString,
    ExpectedBytes,
    ExpectedList,
    ExpectedTuple,
    ExpectedMap,
    ExpectedEnum,
    WrongTupleLength,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(s) => write!(f, "{}", s),
            Error::NonFiniteFloat => write!(f, "Tried to encode a non-finite float"),
            Error::SerializeKey => write!(
                f,
                "Tried to encode just a key (did you mean to call serialize_entry?)"
            ),
            Error::SerializeValue => write!(
                f,
                "Tried to encode just a value (did you mean to call serialize_entry?)"
            ),
            Error::InvalidInput => write!(f, "Supplied input can't be represented in Rust"),
            Error::InvalidUtf8 => write!(f, "String contained invalid UTF-8"),
            Error::NumberTooLarge => write!(f, "Encoded number to large to be represented"),
            Error::NumberTooSmall => write!(f, "Encoded number to small to be represented"),
            Error::ExtraneousInput => {
                write!(f, "Term has extraneous members or is itself extraneous")
            }
            Error::ExpectedBool => write!(f, "Expected bool"),
            Error::ExpectedInt => write!(f, "Expected integer"),
            Error::ExpectedFloat => write!(f, "Expected float"),
            Error::ExpectedString => write!(f, "Expected string"),
            Error::ExpectedBytes => write!(f, "Expected byte array"),
            Error::ExpectedList => write!(f, "Expected list"),
            Error::ExpectedTuple => write!(f, "Expected tuple"),
            Error::ExpectedMap => write!(f, "Expected map"),
            Error::ExpectedEnum => write!(f, "Expected enum"),
            Error::WrongTupleLength => write!(f, "Found tuple with wrong length"),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}
