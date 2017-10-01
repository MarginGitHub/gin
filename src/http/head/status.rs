use std::ops::Deref;

pub use self::Status::*;

#[derive(Debug, Clone, Copy)]
pub enum Status {
//    1xx
    Continue,
    ChangeProto,
//    2xx
    Success,
    Created,
    Accept,
    UnauthorizedInfo,
    NoContent,
    ResetContent,
    PartialContent,
//    3xx
    MutialSelection,
//    4xx
    ErrorRequest,
    UnauthorizedRequest,
    Forbid,
    NoFound,
//    5xx
    ServerInnerErr,
}

impl Deref for Status {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            Continue => "100",
            ChangeProto => "101",
            Success => "200",
            Created => "201",
            Accept => "202",
            UnauthorizedInfo => "203",
            NoContent => "204",
            ResetContent => "205",
            PartialContent => "206",
            MutialSelection => "300",
            ErrorRequest => "400",
            UnauthorizedRequest => "401",
            Forbid => "402",
            NoFound => "403",
            ServerInnerErr => "500",
        }
    }
}

