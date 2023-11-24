use crate::execute::Tx;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const CONFIG_KEY: &[u8] = b"config";

pub const STATE: Item<State> = Item::new("state");

#[cw_serde]
pub struct State {
    pub module_account: String,
    pub pending_txs: Vec<Tx>,
    pub fulfilled_txs: Vec<Tx>,
    pub next_id: u64,
}

#[cw_serde]
pub struct FulfillState {
    pub fulfiller_addr: Addr,
}

pub const FULFILL_REPLY_STATES: Map<u64, FulfillState> = Map::new("fulfill_reply_states");
