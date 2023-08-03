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
	($code:expr, $($fmt:tt)+) => {
		eprint!("{}: ", $crate::env::args().next().unwrap_or("Error".into()));
		eprintln!($($fmt)+);
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
		$crate::err_code!(1, $($fmt)+);
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
		eprintln!($($fmt)+);
	};
}

/// Require that an expression returns [`Result::Ok`] or [`Option::Some`].
///
/// This macro simplifies error handling when the [`Result::Err`] or
/// [`Option::None`] cases of a [`std::result::Result`] or an
/// [`std::option::Option`] result in logging an error and terminating the
/// program.
///
/// When invoked with a [`std::result::Result`], the macro takes only 1 parameter
/// — the [`std::result::Result`] whose success case you require — and in the
/// case of `Err(v)` returns `v`.  Otherwise the [`err!`] macro is called with
/// the format string `"{e}"` (with `e` being the error).
///
/// Since [`std::option::Option`]s don’t return error values — simply returning
/// [`Option::None` ]— we can’t print a meaningful diagnostic message for you.
/// Therefore you the user also need to provide as additional arguments the same
/// parameters you would pass to an invokation of [`err!`].
///
/// # Panics
///
/// Calls [`err!`] which may panic if it fails.
///
/// # Examples
///
/// Try to take a child process’ standard input and assign it to `ci`.  If the
/// result of `stdin.take()` is [`Option::None` ]then print the given diagnostic
/// message and exit the program using [`err!`].
///
/// ```
/// use cerm::require;
///
/// let ci = require!(
/// 	child.stdin.take(),
/// 	"Failed to open stdin of “{}”",
/// 	cmd.to_string_lossy()
/// );
/// ```
///
///
/// Wait for a child process to terminate, and execute [`err!`] if the call to
/// `child.wait()` fails.  Notice how because `child.wait()` returns a
/// [`std::result::Result`], we only specify one argument.
///
/// ```
/// use cerm::require;
///
/// require!(child.wait());
/// ```
#[macro_export]
macro_rules! require {
	($e:expr) => {
		match $e {
			Ok(v) => v,
			Err(e) => { err!("{e}"); },
		}
	};
	($e:expr, $($fmt:tt)+) => {
		match $e {
			Some(v) => v,
			None => { err!($($fmt)+); },
		}
	};
}
