use actix::Addr;

use actix_web::{
    web,
    Error,
    HttpRequest,
    HttpResponse,
};

use actix_web_actors::ws;

use crate::websocket::{
    ws_server::WsServer,
    ws_session::WsSession,
};

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {

    ws::start(
        WsSession {
            id: 0,
            server: server.get_ref().clone(),
        },
        &req,
        stream,
    )
}



// use actix::Addr;

// use actix_web::{
//     web,
//     Error,
//     HttpRequest,
//     HttpResponse,
// };

// use actix_web_actors::ws;

// use crate::websocket::{
//     ws_server::WsServer,
//     ws_session::WsSession,
// };

// pub async fn ws_route(
//     req: HttpRequest,
//     stream: web::Payload,
//     server: web::Data<Addr<WsServer>>,
// ) -> Result<HttpResponse, Error> {

//     ws::start(
//         WsSession {
//             id: 0,
//             server: server.get_ref().clone(),
//         },
//         &req,
//         stream,
//     )
// }