#[cfg(test)]
pub mod initialize {

    use cosmwasm_std::{Addr, Coin};
    use osmosis_test_tube::{Account, Module, OsmosisTestApp, SigningAccount, Wasm};

    use crate::msg::InstantiateMsg;

    const ADMIN_BALANCE_AMOUNT: u128 = 340282366920938463463374607431768211455u128;
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    pub fn default_init() -> (OsmosisTestApp, Addr, SigningAccount, SigningAccount) {
        init_test_contract(
            "../../target/wasm32-unknown-unknown/release/catalyst.wasm",
            &[
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_BASE),
                Coin::new(ADMIN_BALANCE_AMOUNT, DENOM_QUOTE),
            ],
        )
    }

    pub fn init_test_contract(
        filename: &str,
        admin_balance: &[Coin],
    ) -> (OsmosisTestApp, Addr, SigningAccount, SigningAccount) {
        // Create new osmosis appchain instance
        let app = OsmosisTestApp::new();
        let wasm = Wasm::new(&app);

        // Create new account with initial funds
        let admin = app.init_account(admin_balance).unwrap();

        // Load compiled wasm bytecode
        let wasm_byte_code = std::fs::read(filename).unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &admin)
            .unwrap()
            .data
            .code_id;

        // Initialize an account with no coins
        let module_account = app
            .init_account(&[Coin::new(1_000_000_000_000, "uosmo")])
            .unwrap();

        // Instantiate vault
        let contract = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    module_account: module_account.address(),
                },
                Some(admin.address().as_str()),
                Some("cl-vault"),
                &[],
                &admin,
            )
            .unwrap();

        (
            app,
            Addr::unchecked(contract.data.address),
            admin,
            module_account,
        )
    }
}
