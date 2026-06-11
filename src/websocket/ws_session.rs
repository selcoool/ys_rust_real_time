use actix::prelude::*;
use actix_web_actors::ws;

use super::ws_server::{
    Connect,
    Disconnect,
    WsMessage,
    WsServer,
};

pub struct WsSession {
    pub id: usize,
    pub server: Addr<WsServer>,
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
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .map(|res, act, _| {
                act.id = res.unwrap();
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
            },
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
    >,
> for WsSession
{
    fn handle(
        &mut self,
        _: Result<
            ws::Message,
            ws::ProtocolError,
        >,
        _: &mut Self::Context,
    ) {
    }
}





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