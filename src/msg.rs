use crate::execute::Tx;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub module_account: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    AddTx {
        destination_addr: String,
        output_coin: Coin,
    },
    FulfillTx {
        tx_id: u64,
    },
    RemoveTx {
        tx_id: u64,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetTxsResponse)]
    GetPendingTxs {},

    #[returns(GetTxsResponse)]
    GetFulfilledTxs {},
}

#[cw_serde]
pub struct GetTxsResponse {
    pub txs: Vec<Tx>,
}
