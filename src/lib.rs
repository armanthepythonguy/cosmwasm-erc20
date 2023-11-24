use cosmwasm_std::{entry_point, DepsMut, MessageInfo, StdResult, Response, Env, StdError, Uint128, Binary, Deps, to_json_binary};
use thiserror::Error;
use contract::execute;
pub mod msg;
mod contract;
mod state;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError{
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized {owner: String},

    #[error("You don't have enough balance. You have {balance}")]
    Minbalance {balance: Uint128},

    #[error("You don't have enough allowance. You have {allowance}")]
    MinAllowance {allowance: Uint128}
}

#[entry_point]
pub fn instantiate(_deps: DepsMut, _env:Env, _info:MessageInfo, _msg: msg::InstantiateMsg) -> StdResult<Response>{
    contract::instantiate(_deps, _info, _msg.name, _msg.symbol, _msg.decimal, _msg.total_supply)
}

#[entry_point]
pub fn query(
    _deps: Deps,
    _env: Env,
    _msg: msg::QueryMsg
) -> StdResult<Binary>{
    use msg::QueryMsg::*;
    match _msg{
        Name {} => to_json_binary(&contract::query::name(_deps)?),
        Symbol {} => to_json_binary(&contract::query::symbol(_deps)?),
        Decimal {} => to_json_binary(&contract::query::decimal(_deps)?),
        TotalSupply {} => to_json_binary(&contract::query::total_supply(_deps)?),
        BalanceOf { owner } => to_json_binary(&contract::query::balance_of(_deps, owner)?),
        Allowance { sender, recipient } => to_json_binary(&contract::query::allowance(_deps, sender, recipient)?)
    }
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: msg::ExecMsg
) -> Result<Response, ContractError>{
    use msg::ExecMsg::*;
    match _msg{
        Mint{ recipient, value } => execute::mint(_deps, _info, value, recipient),
        Burn{value} => execute::burn(_deps, _info, value),
        Transfer{ recipient, value } => execute::transfer(_deps, _info, recipient, value),
        TransferFrom{ sender,recipient,value } =>  execute::transfer_from(_deps, _info, sender, recipient, value),
        Approve{ recipient, value } => execute::approve(_deps, _info, recipient, value) 
    }
}
#[cfg(test)]
pub mod multitest;