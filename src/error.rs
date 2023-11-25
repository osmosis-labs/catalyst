use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Transaction not found: {id:}")]
    TransactionNotFound { id: u64 },

    #[error("Only single coin authorized, got: {num_coins:}")]
    MultipleCoinError { num_coins: usize },
}
