use actix_web::{HttpRequest, HttpResponse};

use crate::{models::Model};

use crate::module_models::{Module, MutModule};

use crate::errors_middleware::map_int_parsing_error;
use crate::errors_middleware::map_sql_error;
use crate::errors_middleware::CustomHttpError;

use crate::response_middleware::HttpResponseBuilder;

/// Creates a module by passing a module-like JSON object.
pub async fn create_module(req_body: String) -> Result<HttpResponse, CustomHttpError> {
    let new_module: MutModule = serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;

    Module::create(&new_module).map_err(map_sql_error)?;

    HttpResponseBuilder::new(201, &new_module)
}

/// Gets all modules.
pub async fn get_modules() -> Result<HttpResponse, CustomHttpError> {
    let modules = Module::read_all().map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &modules)
}

/// Gets one module by passing a module ID.
pub async fn get_module(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let module_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    let module = Module::read_one(module_id).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &module)
}

/// Updates a module by passing a module-like JSON object and a module ID.
pub async fn update_module(
    req_body: String,
    req: HttpRequest,
) -> Result<HttpResponse, CustomHttpError> {
    let new_module: MutModule = serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;
    let module_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    Module::update(module_id, &new_module).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &new_module)
}

/// Deletes a module by passing a module ID.
pub async fn delete_module(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let module_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    Module::delete(module_id).map_err(map_sql_error)?;

    HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", module_id))
}
