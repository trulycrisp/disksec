//! Identify command result parsing.

pub mod controller;
pub mod namespace;
pub mod namespace_list;

/// Identify error.
#[derive(Debug)]
pub enum Error {
    /// Identify controller error.
    Controller(controller::Error),
    /// Identify namespace error.
    Namespace(namespace::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Controller(x) => x.source(),
            Self::Namespace(x) => x.source(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Controller(x) => x.fmt(f),
            Self::Namespace(x) => x.fmt(f),
        }
    }
}

impl From<controller::Error> for Error {
    fn from(value: controller::Error) -> Self {
        Self::Controller(value)
    }
}

impl From<namespace::Error> for Error {
    fn from(value: namespace::Error) -> Self {
        Self::Namespace(value)
    }
}
