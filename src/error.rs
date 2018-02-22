use std::fmt;
use std::ffi::OsString;
use std::io::Error as IoError;
use std::option::NoneError;
use single::Error as SingleError;
#[allow(unused_imports)]
use super::*;

#[derive(Fail, Debug)]  //Papercut: PartialEq is not object safe and cannot be used with io::Error :(
pub enum Errar {
    //#[fail(display = "{}", MSG_ERROR)]
    ArgInvalidUtf8(OsString),
    IoError(IoError),
    NoneError(NoneError),
    SingleError(SingleError),
}

impl fmt::Display for Errar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Errar::ArgInvalidUtf8(ref arg) => format!("{}: {:?}", MSG_ERR_ARG_INVALID_UTF8, arg),
            Errar::IoError(ref err) => format!("{}: {:?}", MSG_ERR_IO_ERROR, err),
            Errar::NoneError(ref err) => format!("{}: {:?}", MSG_ERR_NONE_ERROR, err),
            Errar::SingleError(ref err) => format!("{}: {:?}", MSG_ERR_SINGLE_ERROR, err),
        })
    }
}

impl From<OsString> for Errar {
    fn from(arg: OsString) -> Self {
        Errar::ArgInvalidUtf8(arg)
    }
}

impl From<IoError> for Errar {
    fn from(err: IoError) -> Self {
        Errar::IoError(err)
    }
}

impl From<NoneError> for Errar {
    fn from(err: NoneError) -> Self {
        Errar::NoneError(err)
    }
}

impl From<SingleError> for Errar {
    fn from(err: SingleError) -> Self {
        Errar::SingleError(err)
    }
}
