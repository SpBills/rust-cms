use actix_web::{HttpResponse, web};
use serde::{de::DeserializeOwned, Serialize};

use crate::models::Model;

use crate::errors_middleware::map_sql_error;
use crate::errors_middleware::CustomHttpError;

use crate::response_middleware::HttpResponseBuilder;
use async_trait::async_trait;

#[derive(serde::Deserialize, Clone)]
pub struct Req {
    pub id: i32
}

#[async_trait]
pub trait Controller<T, TMut>
where
    T: Model<T, TMut> + Serialize + 'static,
    TMut: Serialize + DeserializeOwned + 'static,
{
    async fn create(req_body: String) -> Result<HttpResponse, CustomHttpError> {
        let new: TMut = serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;

        T::create(&new).map_err(map_sql_error)?;

        HttpResponseBuilder::new(201, &new)
    }
    async fn read_all() -> Result<HttpResponse, CustomHttpError> {
        let all = T::read_all().map_err(map_sql_error)?;

        HttpResponseBuilder::new(200, &all)
    }
    async fn read_one(path: web::Path<Req>) -> Result<HttpResponse, CustomHttpError> {
        let id = path.id;

        let module = T::read_one(id).map_err(map_sql_error)?;

        HttpResponseBuilder::new(200, &module)
    }
    async fn update(req_body: String, path: web::Path<Req>) -> Result<HttpResponse, CustomHttpError> {
        let new: TMut = serde_json::from_str(&req_body).or(Err(CustomHttpError::BadRequest))?;
        let id = path.id;

        T::update(id, &new).map_err(map_sql_error)?;

        HttpResponseBuilder::new(200, &new)
    }
    async fn delete(path: web::Path<Req>) -> Result<HttpResponse, CustomHttpError> {
        let id = path.id;


        T::delete(id).map_err(map_sql_error)?;

        HttpResponseBuilder::new(200, &format!("Successfully deleted resource {}", id))
    }
}
