use actix_web::{web, Scope};

use crate::controllers::Controller;

use crate::{module_controllers::ModuleController};

pub struct ModuleRouter;

impl ModuleRouter {
    pub fn new() -> Scope {
        web::scope("/modules")
            .route("", web::post().to(ModuleController::create))
            .route("", web::get().to(ModuleController::read_all))
            .route("/{id}", web::get().to(ModuleController::read_one))
            .route("/{id}", web::put().to(ModuleController::update))
            .route("/{id}", web::delete().to(ModuleController::delete))
    }
}
