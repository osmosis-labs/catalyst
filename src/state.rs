use crate::execute::Tx;
use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

pub const CONFIG_KEY: &[u8] = b"config";

pub const STATE: Item<State> = Item::new("state");

#[cw_serde]
pub struct State {
    pub module_account: String,
    pub pending_txs: Vec<Tx>,
    pub fulfilled_txs: Vec<Tx>,
    pub next_id: u64,
}
