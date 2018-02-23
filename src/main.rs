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
#![feature(termination_trait, try_trait, iterator_try_fold)]

#[macro_use] extern crate failure;
#[macro_use] extern crate structopt;
mod consts;
mod error;

use consts::*;
pub use error::Error;
use std::fs::File;
use std::ffi::OsString;
use std::env::args_os;
use std::io::{BufRead, BufReader, BufWriter, Write};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Error>;

const CSV_DELIMITER: char = ',';

#[derive(StructOpt, Debug)]
struct Opt {
    in_file: String,
    col_name: String,
    replacement: String,
    out_file: String,
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    let (in_file, col_name, replacement, out_file) = (&args.in_file, &args.col_name, &args.replacement, &args.out_file);

    let mut lines = BufReader::new(File::open(&in_file)?).lines();
    let col_idx = lines.next()??.split(CSV_DELIMITER)
                                .position(|c| c == col_name )
                                .ok_or_else(|| Error::ColumnNotFound(col_name.to_string()))?;

    let mut wtr = BufWriter::new(File::create(out_file)?);

    for line in lines {
        wtr.write_all((line?.split(CSV_DELIMITER)
                .enumerate()
                .map(|(i, c)| {
                    match i == col_idx {
                        true => replacement,
                        false => c,
                    }
                })
                .fold(String::new(), |r, c| r + c + "," ) + "\n")
            .as_ref())?;
    }
    Ok(())
}
