use core::fmt::{self, Display, Formatter};

pub type Result<T> = core::result::Result<T, ListenerError>;

/// An error that occurs in the operations of [`Listener`](crate::listener::Listener)s.
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum ListenerError {
    /// An error that occurs while the listener polls for an event.
    TickError(&'static str),
}

impl Display for ListenerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ListenerError::TickError(err_msg) => err_msg.fmt(f),
        }
    }
}
