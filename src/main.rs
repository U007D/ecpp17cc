#![cfg_attr(feature = "clippy", feature(plugin))] //nightly rustc required by `clippy`
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(match_bool, unit_arg, pub_enum_variant_names)] //disable false positives
#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, empty_enum, enum_glob_use,
        fallible_impl_from, filter_map, if_not_else, int_plus_one, invalid_upcast_comparisons, maybe_infinite_iter,
        mem_forget, missing_debug_implementations, mut_mut, mutex_integer, nonminimal_bool, option_map_unwrap_or,
        option_map_unwrap_or_else, option_map_unwrap_or_else, option_unwrap_used, /*print_stdout,*/
        range_plus_one, redundant_closure, result_map_unwrap_or_else, result_unwrap_used,
        trivial_casts, non_camel_case_types, stutter, trivial_numeric_casts, unicode_not_nfc,
        unseparated_literal_suffix, /*use_debug,*/ use_self, used_underscore_binding, unused_import_braces,
        unnecessary_mut_passed, unused_qualifications, wrong_pub_self_convention)]
#![deny(overflowing_literals, unused_must_use)]
#![feature(termination_trait, try_trait)]

#[macro_use] extern crate failure;
extern crate single;
mod consts;
mod error;

use consts::*;
pub use error::Errar;
use single::Single;
use std::fs::File;
use std::ffi::OsString;
use std::env::args_os;
use std::io::{BufReader, BufRead, BufWriter};
type Result<T> = std::result::Result<T, Errar>;

const CSV_DELIMITER: char = ',';

fn main() -> Result<()> {
    let args = args_os().map(|arg| arg.into_string())
                        .collect::<std::result::Result<Vec<_>, OsString>>()?;
    let (in_file, column_name, replacement, out_file) = (&args[1], &args[2], &args[3], &args[4]);
    let lines = BufReader::new(File::open(&in_file)?).lines();
    let col_idx = lines.take(1)
                       .map(|l| l.map_err(|e| Errar::IoError(e))
                           .and_then(|s|
                               s.split(CSV_DELIMITER)
                                .enumerate()
                                .filter(|&(i, col_name)| col_name == column_name)
                                .map(|(i, col_name)| i)
                                .single()
                                     .map_err(|e| Errar::SingleError(e))
                           )
                       )
                       .single()??;
    let wtr = BufWriter::new(File::create(out_file)?);

    println!("in_file: {}", col_idx);
    Ok(())
}
