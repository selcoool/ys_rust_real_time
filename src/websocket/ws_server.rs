use actix::prelude::*;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Broadcast {
    pub message: String,
}

pub struct WsServer {
    sessions: HashMap<
        usize,
        Recipient<WsMessage>,
    >,
    next_id: usize,
}

impl WsServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = usize;

    fn handle(
        &mut self,
        msg: Connect,
        _: &mut Context<Self>,
    ) -> Self::Result {

        let id = self.next_id;

        self.next_id += 1;

        self.sessions.insert(
            id,
            msg.addr,
        );

        id
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(
        &mut self,
        msg: Disconnect,
        _: &mut Context<Self>,
    ) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<Broadcast> for WsServer {
    type Result = ();

    fn handle(
        &mut self,
        msg: Broadcast,
        _: &mut Context<Self>,
    ) {

        for client in self.sessions.values() {

            let _ = client.do_send(
                WsMessage(
                    msg.message.clone(),
                ),
            );
        }
    }
}





// use actix::prelude::*;
// use std::collections::HashMap;

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct WsMessage(pub String);

// #[derive(Message)]
// #[rtype(usize)]
// pub struct Connect {
//     pub addr: Recipient<WsMessage>,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Disconnect {
//     pub id: usize,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Broadcast {
//     pub message: String,
// }

// pub struct WsServer {
//     pub sessions:
//         HashMap<usize, Recipient<WsMessage>>,
//     pub next_id: usize,
// }

// impl WsServer {
//     pub fn new() -> Self {
//         Self {
//             sessions: HashMap::new(),
//             next_id: 1,
//         }
//     }
// }

// impl Actor for WsServer {
//     type Context = Context<Self>;
// }

// impl Handler<Connect> for WsServer {
//     type Result = usize;

//     fn handle(
//         &mut self,
//         msg: Connect,
//         _: &mut Context<Self>,
//     ) -> Self::Result {
//         let id = self.next_id;

//         self.sessions.insert(id, msg.addr);

//         self.next_id += 1;

//         id
//     }
// }

// impl Handler<Disconnect> for WsServer {
//     type Result = ();

//     fn handle(
//         &mut self,
//         msg: Disconnect,
//         _: &mut Context<Self>,
//     ) {
//         self.sessions.remove(&msg.id);
//     }
// }

// impl Handler<Broadcast> for WsServer {
//     type Result = ();

//     fn handle(
//         &mut self,
//         msg: Broadcast,
//         _: &mut Context<Self>,
//     ) {
//         for session in self.sessions.values() {
//             let _ =
//                 session.do_send(
//                     WsMessage(
//                         msg.message.clone(),
//                     ),
//                 );
//         }
//     }
// }