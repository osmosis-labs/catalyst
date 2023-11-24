use crate::state::{State, CONFIG_KEY};
use crate::ContractError;
use cosmwasm_std::{
    BankMsg, Coin, DepsMut, Env, MessageInfo, Reply, Response, SubMsg, SubMsgResponse, SubMsgResult,
};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Tx {
    pub id: u64,
    pub destination_addr: String,
    pub coin: Coin,
}

pub fn add_tx(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    destination_addr: String,
    coin: Coin,
) -> Result<Response, ContractError> {
    let config_key_str = std::str::from_utf8(CONFIG_KEY).unwrap();
    let config = Item::new(config_key_str);
    let mut state: State = config.load(deps.storage).map_err(ContractError::Std)?;

    let new_id = state.next_id;
    state.txs.push(Tx {
        id: new_id,
        destination_addr,
        coin,
    });
    state.next_id += 1;

    config
        .save(deps.storage, &state)
        .map_err(ContractError::Std)?;

    Ok(Response::new())
}

pub fn fulfill_tx(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tx_id: u64,
) -> Result<Response, ContractError> {
    let config_key_str = std::str::from_utf8(CONFIG_KEY).unwrap();
    let config = Item::new(config_key_str);
    let state: State = config.load(deps.storage).map_err(ContractError::Std)?;

    let tx_position = state.txs.iter().position(|tx| tx.id == tx_id);

    let tx: Tx;
    match tx_position {
        Some(index) => {
            tx = state.txs[index].clone();
        }
        None => {
            return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                "Transaction not found",
            )));
        }
    }

    let coins: Vec<Coin> = info.funds;
    if coins.len() != 1 {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
            "Expected exactly one coin",
        )));
    }
    let coin = coins[0].clone();
    let bank_send_msg = BankMsg::Send {
        to_address: tx.clone().destination_addr,
        amount: vec![coin],
    };

    Ok(Response::new()
        .add_attribute("action", "fulfill_tx")
        .add_submessage(SubMsg::reply_on_success(bank_send_msg, tx.id)))
}

pub fn move_to_fulfilled_tx(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
    tx_id: u64,
) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(_) = msg.result {
        let config_key_str = std::str::from_utf8(CONFIG_KEY).unwrap();
        let config = Item::new(config_key_str);
        let mut state: State = config.load(deps.storage).map_err(ContractError::Std)?;

        let tx_position = state.txs.iter().position(|tx| tx.id == tx_id);
        match tx_position {
            Some(index) => {
                let tx = state.txs.remove(index);
                state.fulfilled_txs.push(tx);
            }
            None => {
                return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                    "Transaction not found",
                )));
            }
        }

        config
            .save(deps.storage, &state)
            .map_err(ContractError::Std)?;

        return Ok(Response::new());
    }

    Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
        "Transaction not found",
    )))
}

pub fn remove_tx(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    tx_id: u64,
) -> Result<Response, ContractError> {
    let config_key_str = std::str::from_utf8(CONFIG_KEY).unwrap();
    let config = Item::new(config_key_str);
    let mut state: State = config.load(deps.storage).map_err(ContractError::Std)?;

    let tx_position = state.txs.iter().position(|tx| tx.id == tx_id);
    match tx_position {
        Some(index) => {
            state.txs.remove(index);
        }
        None => {
            return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                "Transaction not found",
            )));
        }
    }

    config
        .save(deps.storage, &state)
        .map_err(ContractError::Std)?;

    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Binary;
    use cosmwasm_std::Uint128;

    #[test]
    fn test_add_tx() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(2, "token"));

        let destination_addr = "destination_addr".to_string();
        let coin = Coin {
            denom: "token".to_string(),
            amount: Uint128::from(100u128),
        };

        // Initialize state
        let config_key_str = std::str::from_utf8(CONFIG_KEY).unwrap();
        let config = Item::new(config_key_str);
        let state = State {
            market_maker: "maker".to_string(),
            txs: vec![],
            fulfilled_txs: vec![],
            next_id: 0,
        };
        config.save(deps.as_mut().storage, &state).unwrap();

        // Call add_tx
        add_tx(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            destination_addr.clone(),
            coin.clone(),
        )
        .unwrap();

        // Load state from storage
        let state: State = config.load(deps.as_ref().storage).unwrap();

        // Check if the transaction was added
        assert_eq!(state.txs.len(), 1);
        assert_eq!(state.txs[0].destination_addr, destination_addr);
        assert_eq!(state.txs[0].coin, coin);
    }

    #[test]
    fn test_fulfill_tx() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(2, "token"));

        let destination_addr = "destination_addr".to_string();
        let coin = Coin {
            denom: "token".to_string(),
            amount: Uint128::from(100u128),
        };

        // Initialize state
        let config_key_str = std::str::from_utf8(CONFIG_KEY).unwrap();
        let config = Item::new(config_key_str);
        let state = State {
            market_maker: "maker".to_string(),
            txs: vec![],
            fulfilled_txs: vec![],
            next_id: 0,
        };
        config.save(deps.as_mut().storage, &state).unwrap();

        // Add a transaction
        add_tx(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            destination_addr.clone(),
            coin.clone(),
        )
        .unwrap();

        // Fulfill the transaction
        let result = fulfill_tx(deps.as_mut(), env.clone(), info.clone(), 0);

        // Check if the transaction was fulfilled successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_to_fulfilled_tx() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(2, "token"));

        let destination_addr = "destination_addr".to_string();
        let coin = Coin {
            denom: "token".to_string(),
            amount: Uint128::from(100u128),
        };

        // Initialize state
        let config_key_str = std::str::from_utf8(CONFIG_KEY).unwrap();
        let config = Item::new(config_key_str);
        let state = State {
            market_maker: "maker".to_string(),
            txs: vec![],
            fulfilled_txs: vec![],
            next_id: 0,
        };
        config.save(deps.as_mut().storage, &state).unwrap();

        // Add a transaction
        add_tx(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            destination_addr.clone(),
            coin.clone(),
        )
        .unwrap();

        // Create a Reply message
        let msg = Reply {
            id: 0,
            result: SubMsgResult::Ok(SubMsgResponse {
                data: Some(Binary::from(vec![0u8, 1, 2])),
                events: vec![],
            }),
        };
        // Move the transaction to fulfilled transactions
        move_to_fulfilled_tx(deps.as_mut(), env.clone(), msg, 0).unwrap();

        // Load state from storage
        let state: State = config.load(deps.as_ref().storage).unwrap();

        // Check if the transaction was moved to fulfilled transactions
        assert_eq!(state.txs.len(), 0);
        assert_eq!(state.fulfilled_txs.len(), 1);
        assert_eq!(state.fulfilled_txs[0].id, 0);
    }

    #[test]
    fn test_remove_tx() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(2, "token"));

        let owner = "owner".to_string();
        let output_coin = Coin {
            denom: "token".to_string(),
            amount: Uint128::from(100u128),
        };

        // Initialize state
        let config_key_str = std::str::from_utf8(CONFIG_KEY).unwrap();
        let config = Item::new(config_key_str);
        let state = State {
            market_maker: "maker".to_string(),
            txs: vec![],
            fulfilled_txs: vec![],
            next_id: 0,
        };
        config.save(deps.as_mut().storage, &state).unwrap();

        // Add 3 transactions
        for _ in 0..3 {
            add_tx(
                deps.as_mut(),
                env.clone(),
                info.clone(),
                owner.clone(),
                output_coin.clone(),
            )
            .unwrap();
        }

        // Remove one transaction
        remove_tx(deps.as_mut(), env.clone(), info.clone(), 1).unwrap();

        // Load state from storage
        let state: State = config.load(deps.as_ref().storage).unwrap();

        // Check if only 2 transactions remain
        assert_eq!(state.txs.len(), 2);
        assert!(state.txs.iter().find(|&tx| tx.id == 1).is_none());
    }
}
