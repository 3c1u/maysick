/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RuntimeError {
    CastError,
    IncompatibleTypeError,
    UnimplementedErr,

    ArgumentErr,

    IOError,

    OutOfIndexError,

    UnknownErr,
}
