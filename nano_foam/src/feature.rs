#![allow(unused)]

use std::{collections::HashSet, str::FromStr};

use syn::{bracketed, parse::ParseStream, punctuated::Punctuated, Ident, Token};

use crate::token;

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) enum Feature {
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

pub(crate) struct Features {
    features: Punctuated<Ident, Token![,]>,
    feature_set: HashSet<Feature>
}

impl Features {
    pub(crate) fn contains(&self, f: Feature) -> bool {
        self.feature_set.contains(&f)
    }
}

impl syn::parse::Parse for Features {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<token::features>()?;
        input.parse::<syn::Token![:]>()?;

        let content;
        bracketed!(content in input);

        let features = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;

        let mut feature_set = HashSet::new();
        for feature in features.iter() {
            let f = format!("{}", feature);

            if let Ok(f) = f.parse() {
                feature_set.insert(f);
            } else {
                return Err(syn::Error::new(feature.span(), format!("unknown feature `{}`", f)));
            }
        }

        Ok(Self{
            features,
            feature_set
        })
    }
}