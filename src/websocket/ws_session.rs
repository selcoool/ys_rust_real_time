
use actix::prelude::*;
use actix_web_actors::ws;

use serde_json::json;
use sqlx::MySqlPool;

use crate::{
    models::product::Product,
    repositories::product_repository,
};

use super::ws_server::{
    Broadcast,
    Connect,
    Disconnect,
    WsMessage,
    WsServer,
};

pub struct WsSession {
    pub id: usize,
    pub server: Addr<WsServer>,
    pub pool: MySqlPool,
}

impl Actor for WsSession {
    type Context =
        ws::WebsocketContext<Self>;

    fn started(
        &mut self,
        ctx: &mut Self::Context,
    ) {

        let addr = ctx.address();

        self.server
            .send(
                Connect {
                    addr: addr.recipient(),
                }
            )
            .into_actor(self)
            .map(|res, act, ctx| {

                act.id =
                    res.unwrap();

                let pool =
                    act.pool.clone();

                let addr =
                    ctx.address();

                actix::spawn(
                    async move {

                        let products =
                            product_repository::get_all(
                                &pool
                            )
                            .await
                            .unwrap_or_else(
                                |_| Vec::<Product>::new()
                            );

                        let payload =
                            json!({
                                "event":"init",
                                "products":products
                            });

                        addr.do_send(
                            WsMessage(
                                payload.to_string()
                            )
                        );
                    }
                );
            })
            .wait(ctx);
    }

    fn stopped(
        &mut self,
        _: &mut Self::Context,
    ) {

        self.server.do_send(
            Disconnect {
                id: self.id,
            }
        );
    }
}

impl Handler<WsMessage>
    for WsSession
{
    type Result = ();

    fn handle(
        &mut self,
        msg: WsMessage,
        ctx: &mut Self::Context,
    ) {

        ctx.text(msg.0);
    }
}

impl StreamHandler<
    Result<
        ws::Message,
        ws::ProtocolError,
    >
> for WsSession
{
    fn handle(
        &mut self,
        msg: Result<
            ws::Message,
            ws::ProtocolError,
        >,
        ctx: &mut Self::Context,
    ) {

        match msg {

            Ok(
                ws::Message::Ping(
                    msg
                )
            ) => {
                ctx.pong(&msg);
            }

            Ok(
                ws::Message::Text(
                    text
                )
            ) => {

                if text.starts_with(
                    "ALL "
                )
                {

                    let message =
                        text.replace(
                            "ALL ",
                            ""
                        );

                    self.server.do_send(
                        Broadcast {
                            message,
                        }
                    );
                }
            }

            Ok(
                ws::Message::Close(_)
            ) => {
                ctx.stop();
            }

            _ => {}
        }
    }
}



// use actix::prelude::*;
// use actix_web_actors::ws;

// use super::ws_server::{
//     Broadcast,
//     Connect,
//     Disconnect,
//     JoinRoom,
//     LeaveRoom,
//     SendToClient,
//     SendToRoom,
//     WsMessage,
//     WsServer,
// };

// pub struct WsSession {
//     pub id: usize,
//     pub server: Addr<WsServer>,
// }

// impl Actor for WsSession {
//     type Context =
//         ws::WebsocketContext<Self>;

//     fn started(
//         &mut self,
//         ctx: &mut Self::Context,
//     ) {

//         let addr = ctx.address();

//         self.server
//             .send(Connect {
//                 addr: addr.recipient(),
//             })
//             .into_actor(self)
//             .map(|res, act, _| {
//                 act.id = res.unwrap();
//             })
//             .wait(ctx);
//     }

//     fn stopped(
//         &mut self,
//         _: &mut Self::Context,
//     ) {

//         self.server.do_send(
//             Disconnect {
//                 id: self.id,
//             },
//         );
//     }
// }

// impl Handler<WsMessage>
//     for WsSession
// {
//     type Result = ();

//     fn handle(
//         &mut self,
//         msg: WsMessage,
//         ctx: &mut Self::Context,
//     ) {

//         ctx.text(msg.0);
//     }
// }

// impl StreamHandler<
//     Result<
//         ws::Message,
//         ws::ProtocolError,
//     >,
// > for WsSession
// {
//     fn handle(
//         &mut self,
//         msg: Result<
//             ws::Message,
//             ws::ProtocolError,
//         >,
//         ctx: &mut Self::Context,
//     ) {

//         match msg {

//             Ok(
//                 ws::Message::Ping(
//                     msg,
//                 ),
//             ) => {
//                 ctx.pong(&msg);
//             }

//             Ok(
//                 ws::Message::Text(
//                     text,
//                 ),
//             ) => {

//                 let text =
//                     text.trim();

//                 if text.starts_with(
//                     "JOIN ",
//                 ) {

//                     let room =
//                         text.replace(
//                             "JOIN ",
//                             "",
//                         );

//                     self.server.do_send(
//                         JoinRoom {
//                             room,
//                             id: self.id,
//                         },
//                     );

//                     ctx.text(
//                         "Joined room",
//                     );
//                 }

//                 else if text
//                     .starts_with(
//                         "LEAVE ",
//                     )
//                 {

//                     let room =
//                         text.replace(
//                             "LEAVE ",
//                             "",
//                         );

//                     self.server.do_send(
//                         LeaveRoom {
//                             room,
//                             id: self.id,
//                         },
//                     );

//                     ctx.text(
//                         "Left room",
//                     );
//                 }

//                 else if text
//                     .starts_with(
//                         "ALL ",
//                     )
//                 {

//                     let message =
//                         text.replace(
//                             "ALL ",
//                             "",
//                         );

//                     self.server.do_send(
//                         Broadcast {
//                             message,
//                         },
//                     );
//                 }

//                 else if text
//                     .starts_with(
//                         "USER:",
//                     )
//                 {

//                     let parts:
//                         Vec<&str> =
//                         text.splitn(
//                             3,
//                             ':',
//                         )
//                         .collect();

//                     if parts.len()
//                         == 3
//                     {

//                         if let Ok(id)
//                             = parts[1]
//                                 .parse::<
//                                     usize,
//                                 >()
//                         {

//                             self.server.do_send(
//                                 SendToClient {
//                                     id,
//                                     message:
//                                         parts[2]
//                                             .to_string(),
//                                 },
//                             );
//                         }
//                     }
//                 }

//                 else if text
//                     .starts_with(
//                         "ROOM:",
//                     )
//                 {

//                     let parts:
//                         Vec<&str> =
//                         text.splitn(
//                             3,
//                             ':',
//                         )
//                         .collect();

//                     if parts.len()
//                         == 3
//                     {

//                         self.server.do_send(
//                             SendToRoom {
//                                 room:
//                                     parts[1]
//                                         .to_string(),
//                                 message:
//                                     parts[2]
//                                         .to_string(),
//                             },
//                         );
//                     }
//                 }
//             }

//             Ok(
//                 ws::Message::Close(
//                     _,
//                 ),
//             ) => {
//                 ctx.stop();
//             }

//             _ => {}
//         }
//     }
// }




// use actix::prelude::*;
// use actix_web_actors::ws;

// use super::ws_server::{
//     Connect,
//     Disconnect,
//     WsMessage,
//     WsServer,
// };

// pub struct WsSession {
//     pub id: usize,
//     pub server: Addr<WsServer>,
// }

// impl Actor for WsSession {
//     type Context =
//         ws::WebsocketContext<Self>;

//     fn started(
//         &mut self,
//         ctx: &mut Self::Context,
//     ) {
//         let addr = ctx.address();

//         self.server
//             .send(Connect {
//                 addr: addr.recipient(),
//             })
//             .into_actor(self)
//             .map(|res, act, _| {
//                 act.id = res.unwrap();
//             })
//             .wait(ctx);
//     }

//     fn stopped(
//         &mut self,
//         _: &mut Self::Context,
//     ) {
//         self.server.do_send(
//             Disconnect {
//                 id: self.id,
//             },
//         );
//     }
// }

// impl Handler<WsMessage>
//     for WsSession
// {
//     type Result = ();

//     fn handle(
//         &mut self,
//         msg: WsMessage,
//         ctx: &mut Self::Context,
//     ) {
//         ctx.text(msg.0);
//     }
// }

// impl StreamHandler<
//     Result<
//         ws::Message,
//         ws::ProtocolError,
//     >,
// > for WsSession
// {
//     fn handle(
//         &mut self,
//         _: Result<
//             ws::Message,
//             ws::ProtocolError,
//         >,
//         _: &mut Self::Context,
//     ) {
//     }
// }





// use actix::prelude::*;
// use actix_web_actors::ws;

// use super::ws_server::{
//     Connect,
//     Disconnect,
//     WsMessage,
//     WsServer,
// };

// pub struct WsSession {
//     pub id: usize,
//     pub server: Addr<WsServer>,
// }

// impl Actor for WsSession {
//     type Context =
//         ws::WebsocketContext<Self>;

//     fn started(
//         &mut self,
//         ctx: &mut Self::Context,
//     ) {
//         let addr = ctx.address();

//         self.server
//             .send(Connect {
//                 addr: addr.recipient(),
//             })
//             .into_actor(self)
//             .map(|res, act, _| {
//                 act.id = res.unwrap();
//             })
//             .wait(ctx);
//     }

//     fn stopped(
//         &mut self,
//         _: &mut Self::Context,
//     ) {
//         self.server.do_send(
//             Disconnect {
//                 id: self.id,
//             },
//         );
//     }
// }

// impl Handler<WsMessage> for WsSession {
//     type Result = ();

//     fn handle(
//         &mut self,
//         msg: WsMessage,
//         ctx: &mut Self::Context,
//     ) {
//         ctx.text(msg.0);
//     }
// }

// impl StreamHandler<
//     Result<
//         ws::Message,
//         ws::ProtocolError,
//     >,
// > for WsSession
// {
//     fn handle(
//         &mut self,
//         _: Result<
//             ws::Message,
//             ws::ProtocolError,
//         >,
//         _: &mut Self::Context,
//     ) {
//     }
// }