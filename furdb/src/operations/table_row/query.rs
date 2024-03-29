use actix_web::{get, web, Responder};
use furdb_core::models as core_models;
use std::error::Error;

use crate::models;

#[get("/{database_id}/{table_id}/query")]
pub(crate) async fn query_handler(
    path: web::Path<(String, String)>,
    query_params: web::Json<models::params::query_params::QueryParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();
    let index = query_params.get_column_index();
    let value = query_params.get_value();

    let furdb = core_models::furdb::FurDB::new(core_models::config::Config::new(None)?)?;
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let data = table.query(index, value)?;

    let response = models::response::get_rows_response::GetRowsResponse::new(&data)?;

    Ok(web::Json(response))
}

#[get("/{database_id}/{table_id}/data")]
pub(crate) async fn get_rows_handler(
    path: web::Path<(String, String)>,
    get_row_params: web::Json<models::params::get_rows_params::GetRowParams>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (database_id, table_id) = path.into_inner();
    let indices = get_row_params.get_indices();

    let furdb = core_models::furdb::FurDB::new(core_models::config::Config::new(None)?)?;
    let database = furdb.get_database(&database_id)?;
    let table = database.get_table(&table_id)?;

    let data = table.get_rows(indices)?;

    let response = models::response::get_rows_response::GetRowsResponse::new(&data)?;

    Ok(web::Json(response))
}
