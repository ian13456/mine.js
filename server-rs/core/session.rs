use log::info;

use ansi_term::Colour::Yellow;

use actix::prelude::*;
use actix_broker::BrokerIssue;
use actix_web_actors::ws;

use crate::core::models::{create_message, encode_message, MessageComponents};
use crate::libs::types::{Coords2, Coords3, Quaternion};
use crate::models::{
    self,
    messages::{self, message::Type as MessageType},
};

use super::message::{self, PlayerUpdate};
use super::message::{JoinWorld, LeaveWorld};
use super::server::WsServer;

#[derive(Debug, Default)]
pub struct WsSession {
    // unique sessions id
    pub id: usize,
    // joined world
    pub world_name: String,
    // name in world
    pub name: Option<String>,
    // radius of render?
    pub render_radius: i16,
}

impl WsSession {
    pub fn join_world(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let world_name = self.world_name.to_owned();

        // First send a leave message for the current room
        let leave_msg = LeaveWorld {
            world_name: self.world_name.clone(),
            client_id: self.id,
        };

        // issue_sync comes from having the `BrokerIssue` trait in scope
        self.issue_system_sync(leave_msg, ctx);

        let join_msg = JoinWorld {
            world_name: world_name.to_owned(),
            client_name: self.name.clone(),
            client_addr: ctx.address().recipient(),
            render_radius: self.render_radius,
        };

        WsServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(result) = id {
                    act.id = result.id;
                    act.world_name = world_name;
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    fn on_chunk_request(&mut self, message: messages::Message) {
        let json = message.parse_json().unwrap();

        let cx = json["x"].as_i64().unwrap() as i32;
        let cz = json["z"].as_i64().unwrap() as i32;

        let mut update = PlayerUpdate::default();

        update.world_name = self.world_name.to_owned();
        update.client_id = self.id.to_owned();
        update.chunk = Some(Coords2(cx, cz));

        WsServer::from_registry().do_send(update);
    }

    fn on_peer(&mut self, message: messages::Message) {
        let messages::Peer {
            name,
            px,
            py,
            pz,
            qx,
            qy,
            qz,
            qw,
            ..
        } = &message.peers[0];

        // TODO: fix this ambiguous logic
        if self.name.is_none() {
            let message = format!(
                "{}(id={}) joined the world {}",
                name.clone(),
                self.id,
                self.world_name
            );

            info!("{}", Yellow.bold().paint(message));
        }

        self.name = Some(name.to_owned());
        let position = Coords3(*px, *py, *pz);
        let rotation = Quaternion(*qx, *qy, *qz, *qw);

        // TODO: fix this monstrosity of cloning
        WsServer::from_registry().do_send(PlayerUpdate {
            client_id: self.id,
            world_name: self.world_name.to_owned(),
            name: self.name.to_owned(),
            position: Some(position),
            rotation: Some(rotation),
            chunk: None,
        });
    }

    fn on_request(&mut self, message: messages::Message) {
        let msg_type = messages::Message::r#type(&message);

        match msg_type {
            MessageType::Request => self.on_chunk_request(message),
            MessageType::Config => {}
            MessageType::Update => {}
            MessageType::Peer => self.on_peer(message),
            MessageType::Message => {}
            MessageType::Init => {
                println!("INIT?")
            }
            _ => {}
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.join_world(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        let message = format!(
            "{}(id={}) left the world {}",
            self.name.clone().unwrap_or_else(|| "unnamed".to_string()),
            self.id,
            self.world_name
        );

        info!("{}", Yellow.bold().paint(message));
    }
}

impl Handler<message::Message> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: message::Message, ctx: &mut Self::Context) {
        let message::Message(msg) = msg;
        let encoded = encode_message(&msg);

        ctx.binary(encoded)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Binary(bytes) => {
                let message = models::decode_message(&bytes.to_vec()).unwrap();
                self.on_request(message);
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
