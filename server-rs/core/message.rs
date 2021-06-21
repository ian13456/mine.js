use actix::prelude::*;

use crate::libs::types::{Coords2, Coords3, Quaternion};

use super::{
    models::{self, ChunkProtocol},
    world::WorldMetrics,
};

/// Base actor message to derive from
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Message(pub models::messages::Message);

/* -------------------------------------------------------------------------- */
/*                             Connection Messages                            */
/* -------------------------------------------------------------------------- */
#[derive(MessageResponse)]
pub struct JoinResult {
    pub id: usize,
}

#[derive(Clone, Message)]
#[rtype(result = "JoinResult")]
pub struct JoinWorld {
    pub world_name: String,
    pub client_name: Option<String>,
    pub client_addr: Recipient<Message>,
    pub render_radius: i16,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveWorld {
    pub world_name: String,
    pub client_id: usize,
}

/* -------------------------------------------------------------------------- */
/*                             Game Play Messages                             */
/* -------------------------------------------------------------------------- */
#[derive(Clone, Message, Default)]
#[rtype(result = "()")]
pub struct PlayerUpdate {
    pub world_name: String,
    pub client_id: usize,

    // Client attributes below
    pub name: Option<String>,
    pub position: Option<Coords3<f32>>,
    pub rotation: Option<Quaternion>,
    pub chunk: Option<Coords2<i32>>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    // World name
    pub world_name: String,
    // id of client session
    pub client_id: usize,
    // Peer message
    pub content: models::messages::Message,
}

#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
pub struct ListWorlds;
