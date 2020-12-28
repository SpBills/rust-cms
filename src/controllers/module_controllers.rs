use crate::{controllers::Controller, module_models::{Module, MutModule}};
use async_trait::async_trait;
pub struct ModuleController;

#[async_trait]
impl Controller<Module, MutModule> for ModuleController {}