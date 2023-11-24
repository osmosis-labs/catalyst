use cosmwasm_std::{to_json_binary, Binary, Deps, StdResult};

use crate::msg::GetTxsResponse;
use crate::state::{State, STATE};

pub fn query_pending_txs(deps: Deps) -> StdResult<Binary> {
    let state: State = STATE.load(deps.storage)?;
    let txs = state.pending_txs;
    let response = GetTxsResponse { txs };
    to_json_binary(&response)
}

pub fn query_fulfilled_txs(deps: Deps) -> StdResult<Binary> {
    let state: State = STATE.load(deps.storage)?;
    let txs = state.fulfilled_txs;
    let response = GetTxsResponse { txs };
    to_json_binary(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute::add_pending_tx;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Coin};
    use cosmwasm_std::{from_json, Uint128};
    #[test]
    fn test_query_all_txs() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(2, "token"));

        let owner = "owner".to_string();
        let output_coin = Coin {
            denom: "token".to_string(),
            amount: Uint128::from(100u128),
        };

        // Initialize state
        let state = State {
            module_account: "module_account".to_string(),
            pending_txs: vec![],
            fulfilled_txs: vec![],
            next_id: 0,
        };
        STATE.save(deps.as_mut().storage, &state).unwrap();

        // Add 3 transactions
        for _ in 0..3 {
            add_pending_tx(
                deps.as_mut(),
                env.clone(),
                info.clone(),
                owner.clone(),
                output_coin.clone(),
            )
            .unwrap();
        }

        // Query all transactions
        let result = query_pending_txs(deps.as_ref()).unwrap();
        let response: GetTxsResponse = from_json(&result).unwrap();

        // Check if the returned result contains the 3 transactions
        assert_eq!(response.txs.len(), 3);
    }
}
