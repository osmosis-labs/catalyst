#[cfg(test)]
mod tests {
    use crate::msg::ExecuteMsg;
    use crate::test_tube::initialize::initialize::default_init;
    use cosmwasm_std::Coin;
    use osmosis_test_tube::{Account, Module, Wasm};

    #[test]
    #[ignore]
    fn tx_lifecycle_fulfilled() {
        let (app, contract_address, _admin, module_account) = default_init();
        let wasm = Wasm::new(&app);

        // In this example, dest is the destination the tx creator wants to bridge funds to
        let dest = app.init_account(&[]).unwrap();

        // In this example, bob is the market maker
        let bob = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uosmo"),
                Coin::new(1_000_000, "ufoo"),
            ])
            .unwrap();

        // Add the tx to the store as the module account, since the module account is the only one authorized to add and remove from this store
        let add_tx = ExecuteMsg::AddTx {
            destination_addr: dest.address(),
            output_coin: Coin::new(1_000_000, "ufoo"),
        };
        wasm.execute(contract_address.as_str(), &add_tx, &[], &module_account)
            .unwrap();

        // Query pending txs to see if the tx is added
        let pending_txs = wasm
            .query::<_, crate::msg::GetTxsResponse>(
                contract_address.as_str(),
                &crate::msg::QueryMsg::GetPendingTxs {},
            )
            .unwrap();

        // Query fulfilled txs to ensure it is empty
        let fulfilled_txs = wasm
            .query::<_, crate::msg::GetTxsResponse>(
                contract_address.as_str(),
                &crate::msg::QueryMsg::GetFulfilledTxs {},
            )
            .unwrap();

        // Check the pending tx
        assert_eq!(pending_txs.txs[0].id, 1);
        assert_eq!(pending_txs.txs[0].destination_addr, dest.address());
        assert_eq!(pending_txs.txs[0].coin, Coin::new(1_000_000, "ufoo"));

        // Check that the fulfilled txs is empty
        assert_eq!(fulfilled_txs.txs.len(), 0);

        // Fulfill the tx
        let fulfill_tx = ExecuteMsg::FulfillTx { tx_id: 1 };
        wasm.execute(
            contract_address.as_str(),
            &fulfill_tx,
            &[Coin::new(1_000_000, "ufoo")],
            &bob,
        )
        .unwrap();

        // Query pending txs to see if the tx is removed
        let pending_txs = wasm
            .query::<_, crate::msg::GetTxsResponse>(
                contract_address.as_str(),
                &crate::msg::QueryMsg::GetPendingTxs {},
            )
            .unwrap();

        // Query fulfilled txs to see if the tx is added
        let fulfilled_txs = wasm
            .query::<_, crate::msg::GetTxsResponse>(
                contract_address.as_str(),
                &crate::msg::QueryMsg::GetFulfilledTxs {},
            )
            .unwrap();

        // Check the pending tx is empty
        assert_eq!(pending_txs.txs.len(), 0);

        // Check the fulfilled tx
        assert_eq!(fulfilled_txs.txs[0].id, 1);
        assert_eq!(fulfilled_txs.txs[0].destination_addr, bob.address());
        assert_eq!(fulfilled_txs.txs[0].coin, Coin::new(1_000_000, "ufoo"));
    }

    #[test]
    #[ignore]
    fn tx_lifecycle_not_fulfilled() {
        let (app, contract_address, _admin, module_account) = default_init();
        let wasm = Wasm::new(&app);

        // In this example, dest is the destination the tx creator wants to bridge funds to
        let dest = app.init_account(&[]).unwrap();

        // Add the tx to the store as the module account, since the module account is the only one authorized to add and remove from this store
        let add_tx = ExecuteMsg::AddTx {
            destination_addr: dest.address(),
            output_coin: Coin::new(1_000_000, "ufoo"),
        };
        wasm.execute(contract_address.as_str(), &add_tx, &[], &module_account)
            .unwrap();

        // Query pending txs to see if the tx is added
        let pending_txs = wasm
            .query::<_, crate::msg::GetTxsResponse>(
                contract_address.as_str(),
                &crate::msg::QueryMsg::GetPendingTxs {},
            )
            .unwrap();

        // Query fulfilled txs to ensure it is empty
        let fulfilled_txs = wasm
            .query::<_, crate::msg::GetTxsResponse>(
                contract_address.as_str(),
                &crate::msg::QueryMsg::GetFulfilledTxs {},
            )
            .unwrap();

        // Check the pending tx
        assert_eq!(pending_txs.txs[0].id, 1);
        assert_eq!(pending_txs.txs[0].destination_addr, dest.address());
        assert_eq!(pending_txs.txs[0].coin, Coin::new(1_000_000, "ufoo"));

        // Check that the fulfilled txs is empty
        assert_eq!(fulfilled_txs.txs.len(), 0);

        // Some time passes and the tx is not fulfilled.
        // The module account manually removes the tx from the pending txs store.
        let remove_tx = ExecuteMsg::RemoveTx { tx_id: (1) };
        wasm.execute(contract_address.as_str(), &remove_tx, &[], &module_account)
            .unwrap();

        // Query pending txs to see if the tx is removed
        let pending_txs = wasm
            .query::<_, crate::msg::GetTxsResponse>(
                contract_address.as_str(),
                &crate::msg::QueryMsg::GetPendingTxs {},
            )
            .unwrap();

        // Query fulfilled txs to see if the tx is not added
        let fulfilled_txs = wasm
            .query::<_, crate::msg::GetTxsResponse>(
                contract_address.as_str(),
                &crate::msg::QueryMsg::GetFulfilledTxs {},
            )
            .unwrap();

        // Check the pending tx is empty
        assert_eq!(pending_txs.txs.len(), 0);

        // Check the fulfilled tx is empty
        assert_eq!(fulfilled_txs.txs.len(), 0);
    }
}
