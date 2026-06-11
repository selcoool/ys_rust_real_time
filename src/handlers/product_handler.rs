use actix::Addr;
use actix_web::{
    web,
    HttpResponse,
    Responder,
};

use serde_json::json;
use sqlx::MySqlPool;

use crate::{
    models::product::{
        CreateProduct,
        UpdateProduct,
    },
    repositories::product_repository,
    websocket::ws_server::{
        Broadcast,
        WsServer,
    },
};

pub async fn get_products(
    pool: web::Data<MySqlPool>,
) -> impl Responder {

    let products =
        product_repository::get_all(
            &pool,
        )
        .await
        .unwrap();

    HttpResponse::Ok().json(products)
}

pub async fn create_product(
    pool: web::Data<MySqlPool>,
    ws_server: web::Data<Addr<WsServer>>,
    body: web::Json<CreateProduct>,
) -> impl Responder {

    let id =
        product_repository::create(
            &pool,
            body.into_inner(),
        )
        .await
        .unwrap();

    let product =
        product_repository::get_by_id(
            &pool,
            id as i64,
        )
        .await
        .unwrap();

    let payload = json!({
        "event":"created",
        "product":product
    });

    ws_server.do_send(
        Broadcast {
            message:
                payload.to_string(),
        },
    );

    HttpResponse::Ok().json(payload)
}

pub async fn update_product(
    path: web::Path<i64>,
    pool: web::Data<MySqlPool>,
    ws_server: web::Data<Addr<WsServer>>,
    body: web::Json<UpdateProduct>,
) -> impl Responder {

    let id = path.into_inner();

    product_repository::update(
        &pool,
        id,
        body.into_inner(),
    )
    .await
    .unwrap();

    let product =
        product_repository::get_by_id(
            &pool,
            id,
        )
        .await
        .unwrap();

    let payload = json!({
        "event":"updated",
        "product":product
    });

    ws_server.do_send(
        Broadcast {
            message:
                payload.to_string(),
        },
    );

    HttpResponse::Ok().json(payload)
}

pub async fn delete_product(
    path: web::Path<i64>,
    pool: web::Data<MySqlPool>,
    ws_server: web::Data<Addr<WsServer>>,
) -> impl Responder {

    let id = path.into_inner();

    product_repository::delete(
        &pool,
        id,
    )
    .await
    .unwrap();

    let payload = json!({
        "event":"deleted",
        "id":id
    });

    ws_server.do_send(
        Broadcast {
            message:
                payload.to_string(),
        },
    );

    HttpResponse::Ok().json(payload)
}



// use actix::Addr;

// use actix_web::{
//     web,
//     HttpResponse,
//     Responder,
// };

// use sqlx::MySqlPool;

// use crate::{
//     models::product::{
//         CreateProduct,
//         UpdateProduct,
//     },
//     repositories::product_repository,
//     websocket::ws_server::{
//         Broadcast,
//         WsServer,
//     },
// };

// pub async fn get_products(
//     pool: web::Data<MySqlPool>,
// ) -> impl Responder {

//     let data =
//         product_repository::get_all(
//             &pool,
//         )
//         .await
//         .unwrap();

//     HttpResponse::Ok().json(data)
// }

// pub async fn create_product(
//     pool: web::Data<MySqlPool>,
//     ws_server: web::Data<Addr<WsServer>>,
//     body: web::Json<CreateProduct>,
// ) -> impl Responder {

//     let id =
//         product_repository::create(
//             &pool,
//             body.into_inner(),
//         )
//         .await
//         .unwrap();

//     ws_server.do_send(
//         Broadcast {
//             message: format!(
//                 r#"{{"event":"created","id":{}}}"#,
//                 id
//             ),
//         },
//     );

//     HttpResponse::Created().finish()
// }

// pub async fn update_product(
//     path: web::Path<i64>,
//     pool: web::Data<MySqlPool>,
//     ws_server: web::Data<Addr<WsServer>>,
//     body: web::Json<UpdateProduct>,
// ) -> impl Responder {

//     product_repository::update(
//         &pool,
//         path.into_inner(),
//         body.into_inner(),
//     )
//     .await
//     .unwrap();

//     ws_server.do_send(
//         Broadcast {
//             message:
//                 r#"{"event":"updated"}"#
//                     .to_string(),
//         },
//     );

//     HttpResponse::Ok().finish()
// }

// pub async fn delete_product(
//     path: web::Path<i64>,
//     pool: web::Data<MySqlPool>,
//     ws_server: web::Data<Addr<WsServer>>,
// ) -> impl Responder {

//     product_repository::delete(
//         &pool,
//         path.into_inner(),
//     )
//     .await
//     .unwrap();

//     ws_server.do_send(
//         Broadcast {
//             message:
//                 r#"{"event":"deleted"}"#
//                     .to_string(),
//         },
//     );

//     HttpResponse::Ok().finish()
// }