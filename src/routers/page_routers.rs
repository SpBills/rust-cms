use actix_web::{web, Scope};

use crate::controllers::Controller;

use crate::page_controllers::PageController;
use crate::page_controllers::get_page_join_modules;

pub struct PageRouter;

impl PageRouter {
    pub fn new() -> Scope {
        web::scope("/pages")
            .route("", web::post().to(PageController::create))
            .route("", web::get().to(PageController::read_all))
            .route("/{id}", web::get().to(PageController::read_one))
            .route("/{id}/modules", web::get().to(get_page_join_modules))
            .route("/{id}", web::put().to(PageController::update))
            .route("/{id}", web::delete().to(PageController::delete))
    }
}
