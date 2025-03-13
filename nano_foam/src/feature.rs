#![allow(unused)]

use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash)]
pub(super) enum Feature {
    JSON,
    XML,
    SQL
}

impl FromStr for Feature {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JSON" => Ok(Feature::JSON),
            "XML" => Ok(Feature::XML),
            "SQL" => Ok(Feature::SQL),
            _ => Err(()),
        }
    }
}
