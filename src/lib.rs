#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use std::{env, process};

/// Print a diagnostic message to the standard error and exit with a given code.
///
/// This macro is analagous to the BSD [`errx(3)`] C function.  It takes at a
/// minimum two arguments.  The first argument is the code with passed to
/// [`std::process::exit()`] with which we exit the program.  The second and
/// optional additional arguments are passed to the [`eprintln!`] macro.
///
/// When invoked, the given format and arguments are printed to the standard
/// error, prepended by the string `"progname: "`, where `progname` is the
/// program name as defined by the first element in [`std::env::args`].  If for
/// whatever reason no such element exists (which is possible), we default to
/// simply using `"Error"` as the program name.
///
/// If you do not care about specifying a specific exit code and are fine with
/// simply defaulting to `1`, you may prefer to use [`err!`].
///
/// # Panics
///
/// Calls [`eprint!`], [`eprintln!`], and [`std::env::args`] which may all panic
/// if they fail.
///
/// # Examples
///
/// Print a diagnostic in the form `progname: path: error`, and then exit the
/// program with an exit status of 2.
///
/// ```
/// use std::fs;
/// use cerm::err_code;
///
/// let res = match fs::create_dir(&path) {
/// 	Ok(v) => v,
/// 	Err(e) => { err_code!(2, "{}: {}", path, e); }
/// };
/// ```
///
/// [`errx(3)`]: https://man.openbsd.org/err.3
#[macro_export]
macro_rules! err_code {
	($code:tt, $($fmt:tt)+) => {
		eprint!("{}: ", $crate::env::args().next().unwrap_or("Error".into()));
		eprintln!($($fmt)*);
		$crate::process::exit($code);
	};
}

/// The same thing as [`err_code!`], but the exit code is always 1.
///
/// This macro is simply a wrapper around [`err_code!`], but with `1` passed as
/// the first argument; everything else is the same.  For documentation on this
/// macro, read the documentation for [`err_code!`] instead.
///
/// # Panics
///
/// Calls [`err!`] which may panic if it fails.
///
/// # Examples
///
/// Print a diagnostic in the form `progname: path: error`, and then exit the
/// program with an exit status of 1.
///
/// ```
/// use std::fs;
/// use cerm::err_code;
///
/// let res = match fs::create_dir(&path) {
/// 	Ok(v) => v,
/// 	Err(e) => { err!("{}: {}", path, e); }
/// };
/// ```
#[macro_export]
macro_rules! err {
	($($fmt:tt)+) => {
		$crate::err_code!(1, $($fmt)*);
	}
}

/// Print a diagnostic message to the standard error.
///
/// This macro is analagous to the BSD [`warnx(3)`] C function.  It takes the
/// same arguments one would pass to a macro like [`println!`].  In fact, the
/// arguments are passed directly to [`eprintln]`.
///
/// When invoked, the given format and arguments are printed to the standard
/// error, prepended by the string `"progname: "`, where `progname` is the
/// program name as defined by the first element in [`std::env::args`].  If for
/// whatever reason no such element exists (which is possible), we default to
/// simply using `"Error"` as the program name.
///
/// # Panics
///
/// Calls [`eprint!`], [`eprintln!`], and [`std::env::args`] which may all panic
/// if they fail.
///
/// # Examples
///
/// Print a diagnostic in the form `progname: path: error`
///
/// ```
/// use std::fs;
/// use cerm::warn;
///
/// let res = match fs::create_dir(&path) {
/// 	Ok(v) => v,
/// 	Err(e) => {
/// 		warn!("{}: {}", path, e);
/// 		/* … */
/// 	}
/// };
/// ```
///
/// [`errx(3)`]: https://man.openbsd.org/err.3
#[macro_export]
macro_rules! warn {
	($($fmt:tt)+) => {
		eprint!("{}: ", $crate::env::args().next().unwrap_or("Error".into()));
		eprintln!($($fmt)*);
	};
}
