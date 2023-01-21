use core::fmt::{self, Display, Formatter};

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum ListenerError {
    TickError(&'static str), // Write doc header above later, not in variants. This is for errors caused by the tick implementation for the listener.
}

impl Display for ListenerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
