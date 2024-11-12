// src/protocol.rs
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::data::GossipTableValue;

#[derive(Serialize, Deserialize, Debug)]
pub enum Protocol {
    PullRequest(Pubkey, Filter),
    PullResponse(Vec<GossipTableValue>),
    PushMessage(Pubkey, Vec<GossipTableValue>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    // Add filter fields as needed
}