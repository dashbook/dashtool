use std::ops::ControlFlow;

use datafusion_sql::parser::DFParser;
use iceberg_rust::model::schema::Schema;
use sqlparser::{ast::visit_relations, dialect::GenericDialect, parser::Parser};

use crate::error::Error;

pub(crate) fn find_relations(sql: &str) -> Result<Vec<String>, Error> {
    let statements = Parser::parse_sql(&GenericDialect, sql)?;
    let mut visited = Vec::new();

    visit_relations(&statements, |relation| {
        visited.push(relation.to_string());
        ControlFlow::<()>::Continue(())
    });
    Ok(visited)
}

pub(crate) fn get_schema(sql: &str) -> Result<Vec<Schema>, Error> {
    let statements = DFParser::parse_sql(sql)?;
    Ok(vec![])
}
