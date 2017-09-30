pub use self::Priority::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum Priority {
    P0 = 0,
    P1,
    P2,
    P3,
}

impl Priority {
    pub fn decrement(&mut self) {
        *self = match *self {
            P0 => P1,
            P1 => P2,
            P2 => P3,
            P3 => unreachable!(),
        };
    }
}