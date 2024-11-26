use std::{fmt::Display, path::Path};

use derive_getters::Getters;
use iceberg_rust::catalog::identifier::Identifier;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Getters)]
pub(crate) struct FullIdentifier {
    catalog_name: String,
    namespace_name: String,
    table_name: String,
}

impl FullIdentifier {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let mut parts = input.split(".");
        let catalog_name = parts
            .next()
            .ok_or(Error::Text("Input is empty".to_string()))?
            .to_owned();
        let namespace_name = parts
            .next()
            .ok_or(Error::Text(format!(
                "Identifier {} has only one part",
                input
            )))?
            .to_owned();
        let table_name = parts
            .next()
            .ok_or(Error::Text(format!(
                "Identifier {} has only two parts",
                input
            )))?
            .to_owned();
        Ok(FullIdentifier {
            catalog_name,
            namespace_name,
            table_name,
        })
    }

    pub(crate) fn parse_path(input: &Path) -> Result<Self, Error> {
        let mut parts = input.iter().rev();
        let table_name = parts
            .next()
            .ok_or(Error::Text(format!(
                "Identifier {:?} has only two parts",
                input
            )))?
            .to_str()
            .ok_or(Error::Text("Failed to convert OsStr".to_string()))?
            .trim_end_matches(".sql")
            .trim_end_matches(".ingest.json")
            .to_owned();
        let namespace_name = parts
            .next()
            .ok_or(Error::Text(format!(
                "Identifier {:?} has only one part",
                input
            )))?
            .to_str()
            .ok_or(Error::Text("Failed to convert OsStr".to_string()))?
            .to_owned();
        let catalog_name = parts
            .next()
            .ok_or(Error::Text("Input is empty".to_string()))?
            .to_str()
            .ok_or(Error::Text("Failed to convert OsStr".to_string()))?
            .to_owned();
        Ok(FullIdentifier {
            catalog_name,
            namespace_name,
            table_name,
        })
    }

    pub(crate) fn identifier(&self) -> Result<Identifier, Error> {
        Ok(Identifier::try_new(
            &vec![self.namespace_name.clone(), self.table_name.clone()],
            None,
        )?)
    }
}

impl Display for FullIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.catalog_name, self.namespace_name, self.table_name
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::FullIdentifier;

    #[test]
    fn parse() {
        let identifier =
            FullIdentifier::parse("bronze.inventory.orders").expect("Failed to parse identifier");

        assert_eq!("bronze.inventory.orders", &identifier.to_string());
    }

    #[test]
    fn parse_path_sql() {
        let identifier = FullIdentifier::parse_path(&Path::new("silver/inventory/factOrder.sql"))
            .expect("Failed to parse identifier");

        assert_eq!("silver.inventory.factOrder", &identifier.to_string());
    }
}
