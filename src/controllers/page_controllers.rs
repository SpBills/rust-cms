use actix_web::{HttpRequest, HttpResponse};
use async_trait::async_trait;

use crate::{controllers::Controller, models::{Joinable}, page_models::PageModuleRelation};

use crate::page_models::{MutPage, Page};

use crate::errors_middleware::map_int_parsing_error;
use crate::errors_middleware::map_sql_error;
use crate::errors_middleware::CustomHttpError;

use crate::response_middleware::HttpResponseBuilder;

pub struct PageController;

#[async_trait]
impl Controller<Page, MutPage> for PageController {}

/// This function also parses the read_one_join_on result out of a tuple.
/// A tuple generates a nasty response that isn't well defined.
/// This function parses it in to a Page that has all of the Modules as children.
pub async fn get_page_join_modules(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    let page_id: i32 = req
        .match_info()
        .get("id")
        .ok_or(CustomHttpError::BadRequest)?
        .parse()
        .map_err(map_int_parsing_error)?;

    let page = Page::read_one_join_on(page_id).map_err(map_sql_error)?;

    let origin_page = &page.get(0).ok_or(CustomHttpError::NotFound)?.0;

    // cast the origin page that is always standard into a new object that has the modules as a vec of children.
    let mut res = PageModuleRelation {
        page_id: origin_page.page_id,
        title: origin_page.title.to_owned(),
        time_created: origin_page.time_created,
        modules: Vec::new(),
    };

    // Parsing of the tuples starts here.
    for tuple in page {
        let module = tuple.1;
        res.modules.push(module);
    }

    HttpResponseBuilder::new(200, &res)
}