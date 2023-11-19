use std::collections::HashMap;

use git2::Oid;
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(
    rename_all = "camelCase",
    try_from = "crate::state::_serde::State",
    into = "crate::state::_serde::State"
)]
pub struct State {
    pub commits: HashMap<String, Oid>,
}

mod _serde {
    use std::collections::HashMap;

    use git2::Oid;
    use serde::{Deserialize, Serialize};

    use crate::error::Error;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct State {
        commits: HashMap<String, String>,
    }

    impl TryFrom<State> for super::State {
        type Error = Error;
        fn try_from(value: State) -> Result<Self, Error> {
            Ok(super::State {
                commits: HashMap::from_iter(
                    value
                        .commits
                        .into_iter()
                        .map(|(k, v)| Ok((k, Oid::from_str(&v)?)))
                        .collect::<Result<Vec<_>, Error>>()?,
                ),
            })
        }
    }

    impl From<super::State> for State {
        fn from(value: super::State) -> Self {
            State {
                commits: HashMap::from_iter(
                    value.commits.into_iter().map(|(k, v)| (k, v.to_string())),
                ),
            }
        }
    }
}
