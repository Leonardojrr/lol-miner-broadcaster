mod connection;
mod miner;
mod msg_classifier;

use connection::ConnectionEvent;
use miner::MinerEvent;
use std::sync::{Arc, Mutex};
use tokio::task;
use uuid::Uuid;

enum Event {
    Conn(ConnectionEvent),
    Msg(MinerEvent),
}

#[tokio::main]
async fn main() {}
