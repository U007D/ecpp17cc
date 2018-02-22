use std::env::args_os;
use std::ffi::OsString;
#[allow(unused_imports)]
use super::*;
use cpp17cc::{App, Error};

type Result<T> = std::result::Result<T, Error>;

// TODO: Auto-generate `Container` using `di::registration` module and `code_gen`.
/// Construct an `App`, supplying all required dependencies (typically via constructor dependency injection).
///
/// # Error
/// The first argument encountered that cannot be converted to valid UTF-8 will exit the app with an error.
///
/// To map invalid UTF-8 sequences to the Unicode-standard replacement character instead of returning a conversion
/// error, (U+FFFD), replace the library invokation line below with:
/// `run(args.os().map(|arg| arg.to_string_lossy()).collect())`
pub struct Container {}

impl Container {
    pub fn new() -> Self {
            Self {}
    }

    pub fn resolve_app(&self) -> Result<App> {
        Ok(
            App::new(self.resolve_args()?, self.resolve_arch_width())
        )
    }

    /// # Remarks
    /// `arg.into_string()` does not allocate; upon successful validation, `OsString` is moved into a `String`
    /// (see `into_string()` at /rust/src/libstd/sys/unix/os_str.rs) via
    /// [`String::from_utf8`](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8)
    fn resolve_args(&self) -> Result<Vec<String>> {
        Ok(
            args_os().map(|arg| arg.into_string())
                 .collect::<std::result::Result<Vec<_>, OsString>>()?
        )
    }

    fn resolve_arch_width(&self) -> usize {
        0_usize.count_zeros() as usize
    }
}
