use crate::execute::Tx;
use cosmwasm_schema::cw_serde;

pub const CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub struct State {
    pub market_maker: String,
    pub txs: Vec<Tx>,
    pub fulfilled_txs: Vec<Tx>,
    pub next_id: u64,
}
