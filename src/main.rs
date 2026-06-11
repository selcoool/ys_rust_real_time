mod config {
    pub mod database;
}

mod models {
    pub mod product;
}

mod repositories {
    pub mod product_repository;
}

mod websocket {
    pub mod ws_server;
    pub mod ws_session;
}

mod handlers {
    pub mod product_handler;
    pub mod websocket_handler;
}

use actix::Actor;
use actix_web::{
    web,
    App,
    HttpServer,
};

use config::database::connect_db;
use websocket::ws_server::WsServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = connect_db().await;

    let ws_server =
        WsServer::new().start();

    println!("SERVER RUNNING => 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(
                    pool.clone(),
                ),
            )
            .app_data(
                web::Data::new(
                    ws_server.clone(),
                ),
            )
            .route(
                "/ws",
                web::get().to(
                    handlers::websocket_handler::ws_route,
                ),
            )
            .route(
                "/products",
                web::get().to(
                    handlers::product_handler::get_products,
                ),
            )
            .route(
                "/products",
                web::post().to(
                    handlers::product_handler::create_product,
                ),
            )
            .route(
                "/products/{id}",
                web::put().to(
                    handlers::product_handler::update_product,
                ),
            )
            .route(
                "/products/{id}",
                web::delete().to(
                    handlers::product_handler::delete_product,
                ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



// mod config {
//     pub mod database;
// }

// mod models {
//     pub mod product;
// }

// mod repositories {
//     pub mod product_repository;
// }

// mod websocket {
//     pub mod ws_server;
//     pub mod ws_session;
// }

// mod handlers {
//     pub mod product_handler;
//     pub mod websocket_handler;
// }

// use actix::Actor;

// use actix_web::{
//     web,
//     App,
//     HttpServer,
// };

// use config::database::connect_db;

// use websocket::ws_server::WsServer;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

//     let pool = connect_db().await;

//     let ws_server =
//         WsServer::new().start();

//     HttpServer::new(move || {
//         App::new()
//             .app_data(
//                 web::Data::new(
//                     pool.clone(),
//                 ),
//             )
//             .app_data(
//                 web::Data::new(
//                     ws_server.clone(),
//                 ),
//             )
//             .route(
//                 "/ws",
//                 web::get().to(
//                     handlers::websocket_handler::ws_route,
//                 ),
//             )
//             .route(
//                 "/products",
//                 web::get().to(
//                     handlers::product_handler::get_products,
//                 ),
//             )
//             .route(
//                 "/products",
//                 web::post().to(
//                     handlers::product_handler::create_product,
//                 ),
//             )
//             .route(
//                 "/products/{id}",
//                 web::put().to(
//                     handlers::product_handler::update_product,
//                 ),
//             )
//             .route(
//                 "/products/{id}",
//                 web::delete().to(
//                     handlers::product_handler::delete_product,
//                 ),
//             )
//     })
//     .bind(("0.0.0.0", 8080))?
//     .run()
//     .await
// }