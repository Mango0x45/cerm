# `cerm` — C Inspired Error Reporting Macros

This crate provides a few handy macros in the style of BSD C’s `<err.h>` for
error-reporting to the user.  These macros just remove some of the boilerplate of
having to prefix your diagnostic messages with `progname: ` everytime you want to
exit the program.
