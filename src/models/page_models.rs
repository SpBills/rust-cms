use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[path = "../schemas/schema.rs"]
mod schema;

use crate::{models::Joinable, module_models::Module};

use super::{
    models::{establish_database_connection, Model},
};
use schema::pages;

/// The main Rust implementation for the Page model.
#[derive(Debug, Serialize, Deserialize, Queryable, PartialEq, Clone)]
pub struct Page {
    pub page_id: i32,
    pub title: String,
    pub time_created: NaiveDateTime,
}
/// This acts as both the insertable and update object.
/// This can be done since pages only really have a `title` column that isn't auto filled.
#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[table_name = "pages"]
pub struct MutPage {
    pub page_id: Option<i32>,
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PageModuleRelation {
    pub page_id: i32,
    pub title: String,
    pub time_created: NaiveDateTime,
    pub modules: Vec<Module>
}

/// Implementation for Page restricted by models.rs trait.
/// schema::...::dsl exports all of the columns.
/// It also exports the table name again. This allows for filtering through the rows of the table.
/// Every one of these functions exports only what they need out of `dsl`.
/// Taking all of the columns (for instance whenever using schema::pages::dsl::*)
/// is unnecessary and leads to higher RAM usage.
impl Model<Page, MutPage> for Page {
    fn create(new_page: &MutPage) -> Result<usize, diesel::result::Error> {
        let db = establish_database_connection();

        Ok(diesel::insert_or_ignore_into(pages::table)
            .values(new_page)
            .execute(&db)?)
    }

    fn read_one(id: i32) -> Result<Self, diesel::result::Error> {
        use schema::pages::dsl::page_id;
        use schema::pages::dsl::pages;
        let db = establish_database_connection();

        pages.filter(page_id.eq(id)).first::<Self>(&db)
    }

    fn read_all() -> Result<Vec<Self>, diesel::result::Error> {
        let db = establish_database_connection();

        pages::table.load::<Self>(&db)
    }

    fn update(id: i32, new_page: &MutPage) -> Result<usize, diesel::result::Error> {
        use schema::pages::dsl::page_id;
        use schema::pages::dsl::pages;
        let db = establish_database_connection();

        Ok(diesel::update(pages.filter(page_id.eq(id)))
            .set(new_page)
            .execute(&db)?)
    }

    fn delete(id: i32) -> Result<usize, diesel::result::Error> {
        use schema::pages::dsl::page_id;
        use schema::pages::dsl::pages;
        let db = establish_database_connection();

        Ok(diesel::delete(pages.filter(page_id.eq(id))).execute(&db)?)
    }
}

/// Separate implementation for joinable trait.
impl Joinable<Page, Module> for Page {
    fn read_one_join_on(id: i32) -> Result<Vec<(Self, Module)>, diesel::result::Error> {
        use schema::pages::dsl::page_id;
        use schema::pages::dsl::pages;
        use schema::modules::dsl::modules;
        let db = establish_database_connection();

        pages.inner_join(modules).filter(page_id.eq(id)).load::<(Page, Module)>(&db)
    }
}
