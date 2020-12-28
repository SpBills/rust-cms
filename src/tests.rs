use super::*;
use actix_web::{http::StatusCode, test};
use module_models::MutModule;
use page_models::MutPage;

use controllers::{Controller, Req};

use module_controllers::ModuleController;
use page_controllers::PageController;

// Creates a page used for unit tests.
async fn create_test_page() {
    let new_page = MutPage {
        page_id: Some(-1),
        title: String::from("Hello world!"),
    };
    PageController::create(serde_json::to_string(&new_page).unwrap())
        .await
        .unwrap();
}

// Creates a module used for unit tests.
async fn create_test_module() {
    let new_module = MutModule {
        module_id: Some(-1),
        module_type_id: 1,
        page_id: -1,
        content: Some(String::from("Hello world!")),
    };
    ModuleController::create(serde_json::to_string(&new_module).unwrap())
        .await
        .unwrap();
}

/// Deletes both pages and modules with the test IDs.
/// This function does nto have to be efficient, as it is only for tests.
async fn cleanup_test_values() {
    let req = Req { id: -1 };
    let cleanup_req: actix_web::web::Path<Req> = actix_web::web::Path(req);
    // Had to do a weird recasting because cloning Path clones the inside object instead of the entire `Path` struct.
    PageController::delete(actix_web::web::Path(cleanup_req.clone()))
        .await
        .unwrap();
    ModuleController::delete(cleanup_req).await.unwrap();
}

#[actix_rt::test]
async fn create_page() {
    let new_page = MutPage {
        page_id: Some(-1),
        title: String::from("Hello world!"),
    };

    let resp = PageController::create(serde_json::to_string(&new_page).unwrap())
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::from_u16(201).unwrap());
}

#[actix_rt::test]
async fn read_all_pages() {
    println!("before");
    create_test_page().await;
    println!("after");

    let resp = PageController::read_all().await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_page() {
    create_test_page().await;

    let req: actix_web::web::Path<Req> = actix_web::web::Path(Req { id: -1 });
    let resp = PageController::read_one(req).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_page_join() {
    create_test_page().await;
    create_test_module().await;

    let req = test::TestRequest::get().param("id", "-1").to_http_request();
    let resp = page_controllers::get_page_join_modules(req).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn update_page() {
    create_test_page().await;

    let new_page = MutPage {
        page_id: None,
        title: String::from("Hello world! updated"),
    };

    let req: actix_web::web::Path<Req> = actix_web::web::Path(Req { id: -1 });
    let resp = PageController::update(serde_json::to_string(&new_page).unwrap(), req)
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn create_modules() {
    create_test_page().await;

    let new_module = MutModule {
        module_id: Some(-1),
        module_type_id: 1,
        page_id: -1,
        content: Some(String::from("Hello world!")),
    };
    let resp = ModuleController::create(serde_json::to_string(&new_module).unwrap())
        .await
        .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::from_u16(201).unwrap());
}

#[actix_rt::test]
async fn read_all_modules() {
    create_test_page().await;
    create_test_module().await;

    let resp = ModuleController::read_all().await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn read_one_module() {
    create_test_page().await;
    create_test_module().await;

    let req: actix_web::web::Path<Req> = actix_web::web::Path(Req { id: -1 });
    let resp = ModuleController::read_one(req).await.unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn update_modules() {
    create_test_page().await;
    create_test_module().await;

    let new_module = MutModule {
        module_id: None,
        module_type_id: 1,
        page_id: -1,
        content: Some(String::from("Hello world! updated")),
    };

    let req: actix_web::web::Path<Req> = actix_web::web::Path(Req { id: -1 });
    let resp = module_controllers::ModuleController::update(
        serde_json::to_string(&new_module).unwrap(),
        req,
    )
    .await
    .unwrap();

    cleanup_test_values().await;

    assert_eq!(resp.status(), StatusCode::OK);
}
