// src/service.rs
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use crate::{
    table::{GossipTable},
    contact_info::ContactInfo,
    protocol::Protocol,
};

pub struct GossipService {
    socket: Arc<UdpSocket>,
    table: Arc<RwLock<GossipTable>>,
    keypair: Arc<Keypair>,
    thread_handle: Option<JoinHandle<()>>,
}

impl GossipService {
    pub fn new(entrypoint: SocketAddr, keypair: Keypair) {
        // Same implementation as before...
    }

    pub fn start(&mut self) {
        // Same implementation as before...
    }

    fn start_gossip_pull_push(&self) {
        // Same implementation as before...
    }
}

fn timestamp() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).unwrap().as_secs()
}