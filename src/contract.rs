#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{
    add_pending_tx, fulfill_pending_tx, move_pending_tx_to_fulfilled_tx, remove_pending_tx,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{query_fulfilled_txs, query_pending_txs};
use crate::state::{State, FULFILL_REPLY_STATES, STATE};

// version info for migration info
pub const FULFILL_ID: u64 = 1u64;
const CONTRACT_NAME: &str = "crates.io:catalyst";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STATE.save(
        deps.storage,
        &State {
            module_account: msg.module_account,
            pending_txs: vec![],
            fulfilled_txs: vec![],
            next_id: 1,
        },
    )?;

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddTx {
            destination_addr,
            output_coin,
        } => add_pending_tx(deps, env, info, destination_addr, output_coin),
        ExecuteMsg::FulfillTx { tx_id } => fulfill_pending_tx(deps, env, info, tx_id),
        ExecuteMsg::RemoveTx { tx_id } => remove_pending_tx(deps, env, info, tx_id),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPendingTxs {} => query_pending_txs(deps),
        QueryMsg::GetFulfilledTxs {} => query_fulfilled_txs(deps),
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    deps.api
        .debug(&format!("executing bank send reply: {msg:?}"));
    if msg.id == FULFILL_ID {
        let msg_clone = msg.clone();

        let fulfill_reply_state = FULFILL_REPLY_STATES.load(deps.storage, msg_clone.id)?;
        FULFILL_REPLY_STATES.remove(deps.storage, msg_clone.id);

        move_pending_tx_to_fulfilled_tx(deps, env, msg, fulfill_reply_state)
    } else {
        Ok(Response::new())
    }
}
