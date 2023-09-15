use std::{ops::ControlFlow, sync::Arc};

use arrow_schema::{FieldRef, Schema as ArrowSchema};
use datafusion_iceberg::catalog::context::IcebergContext;
use datafusion_sql::planner::SqlToRel;
use iceberg_rust::{catalog::Catalog, model::types::StructType};
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

pub(crate) async fn get_schema(
    sql: &str,
    relations: Vec<String>,
    catalog: Arc<dyn Catalog>,
) -> Result<StructType, Error> {
    let context = IcebergContext::new(relations, catalog).await?;
    let statement = Parser::parse_sql(&GenericDialect, sql)?
        .pop()
        .ok_or(Error::Text("No sql statement found.".to_string()))?;

    let planner = SqlToRel::new(&context);

    let logical_plan = planner.sql_statement_to_plan(statement)?;
    let fields: Vec<FieldRef> = logical_plan
        .schema()
        .fields()
        .iter()
        .map(|field| field.field())
        .cloned()
        .collect();
    let struct_type = StructType::try_from(&ArrowSchema::new(fields))?;
    Ok(struct_type)
}
