#![allow(unused)]

#[derive(Debug, Eq, PartialEq, Hash)]
pub(super) enum Feature {
    JSON,
    XML,
    SQLite
}