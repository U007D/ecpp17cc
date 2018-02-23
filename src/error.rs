use std::fmt;
use std::ffi::OsString;
use std::io::Error as IoError;
use std::option::NoneError;
#[allow(unused_imports)]
use super::*;

#[derive(Fail, Debug)]  //Papercut: PartialEq is not object safe and cannot be used with io::Error :(
pub enum Error {
    //#[fail(display = "{}", MSG_ERROR)]
    ArgInvalidUtf8(OsString),
    IoError(IoError),
    ColumnNotFound(String),
    NoneError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::ArgInvalidUtf8(ref arg) => format!("{}: {:?}", MSG_ERR_ARG_INVALID_UTF8, arg),
            Error::IoError(ref err) => format!("{}: {:?}", MSG_ERR_IO_ERROR, err),
            Error::ColumnNotFound(ref name) => format!("{}: {}", MSG_ERR_COLUMN_NOT_FOUND, name),
            Error::NoneError => format!("{}", MSG_ERR_NONE_ERROR),
        })
    }
}

impl From<OsString> for Error {
    fn from(arg: OsString) -> Self {
        Error::ArgInvalidUtf8(arg)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError(err)
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::NoneError
    }
}
