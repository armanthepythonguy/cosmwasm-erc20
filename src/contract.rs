use crate::state::*;
use cosmwasm_std::{Deps, DepsMut, StdResult, MessageInfo, Uint128, Response, Addr};


pub fn instantiate(deps: DepsMut, info:MessageInfo, name: String, symbol: String, decimal:u8, total_supply: Uint128) -> StdResult<Response>{
    NAME.save(deps.storage, &name)?;
    SYMBOL.save(deps.storage, &symbol)?;
    DECIMAL.save(deps.storage, &decimal)?;
    BALANCES.update(deps.storage, &info.sender, |balance: Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() + total_supply)})?;
    TOTALSUPPLY.save(deps.storage, &total_supply)?;
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

pub mod query{

    use cosmwasm_std::{Deps, StdResult, Addr};
    use crate::{msg::*, state::*};

    pub fn name(deps: Deps) -> StdResult<NameResp>{
        let res = NAME.load(deps.storage)?;
        Ok(NameResp{ name : res })
    }

    pub fn symbol(deps: Deps) -> StdResult<SymbolResp>{
        let res = SYMBOL.load(deps.storage)?;
        Ok(SymbolResp{ symbol: res })
    }

    pub fn decimal(deps: Deps) -> StdResult<DecimalResp>{
        let res = DECIMAL.load(deps.storage)?;
        Ok(DecimalResp{ decimal : res })
    }

    pub fn total_supply(deps: Deps) -> StdResult<TotalSupplyResp>{
        let res = TOTALSUPPLY.load(deps.storage)?;
        Ok(TotalSupplyResp{ total_supply : res})
    }

    pub fn balance_of(deps: Deps, owner : Addr) -> StdResult<BalanceOfResp>{
        let res = BALANCES.load(deps.storage, &owner)?;
        Ok(BalanceOfResp{ balance_of : res })
    }

    pub fn allowance(deps: Deps, owner: Addr, spender : Addr) ->  StdResult<AllowanceResp>{
        let res = ALLOWANCES.load(deps.storage, (&owner, &spender))?;
        Ok(AllowanceResp{ allowance : res })
    }
    
}

pub mod execute{
    use cosmwasm_std::{DepsMut, MessageInfo, Uint128, StdResult, Response, Addr};

    use crate::{ContractError, state::{OWNER, BALANCES, TOTALSUPPLY, ALLOWANCES}};


    pub fn mint(deps:DepsMut, info:MessageInfo, value:Uint128, recipient: Addr) -> Result<Response, ContractError>{
        let owner = OWNER.load(deps.storage)?;
        if info.sender!=owner{
            return Err(ContractError::Unauthorized { owner: owner.to_string() });
        }
        BALANCES.update(deps.storage, &recipient, |balance: Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() + value)})?;
        TOTALSUPPLY.update(deps.storage, |supply : Uint128| -> StdResult<_> {Ok(supply + value)})?;
        let resp = Response::new()
            .add_attribute("action", "mint")
            .add_attribute("recipient", recipient.to_string())
            .add_attribute("value", value.to_string());
        Ok(resp)  
    }

    pub fn burn(deps:DepsMut, info:MessageInfo, value:Uint128) -> Result<Response, ContractError>{
        let balance = BALANCES.load(deps.storage, &info.sender)?;
        if balance < value{
            return Err(ContractError::Minbalance { balance: balance })
        }
        BALANCES.update(deps.storage, &info.sender, |balance: Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() - value)})?;
        TOTALSUPPLY.update(deps.storage, |supply : Uint128| -> StdResult<_> {Ok(supply - value)})?;
        let resp = Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("value", value.to_string());
        Ok(resp) 
    }

    pub fn transfer(deps:DepsMut, info:MessageInfo, recipient: Addr, value:Uint128) -> Result<Response, ContractError>{
        let balance = BALANCES.load(deps.storage, &info.sender)?;
        if balance < value{
            return Err(ContractError::Minbalance { balance: balance })
        }
        BALANCES.update(deps.storage, &info.sender, |balance: Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() - value)})?;
        BALANCES.update(deps.storage, &recipient, |balance: Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() + value)})?;
        let resp = Response::new()
            .add_attribute("action", "transfer")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("recipient", recipient.to_string())
            .add_attribute("value", value.to_string());
        Ok(resp) 
    }

    pub fn transfer_from(deps:DepsMut, info:MessageInfo, sender: Addr, recipient: Addr, value:Uint128) -> Result<Response, ContractError>{
        let allowance = ALLOWANCES.load(deps.storage, (&sender, &recipient))?;
        if allowance < value{
            return Err(ContractError::MinAllowance { allowance: allowance })
        }
        BALANCES.update(deps.storage, &sender, |balance: Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() - value)})?;
        BALANCES.update(deps.storage, &recipient, |balance: Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() + value)})?;
        ALLOWANCES.update(deps.storage, (&sender, &recipient), |balance:Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() - value)})?;
        let resp = Response::new()
            .add_attribute("action", "transfer")
            .add_attribute("sender", sender.to_string())
            .add_attribute("recipient", recipient.to_string())
            .add_attribute("value", value.to_string());
        Ok(resp) 
    }

    pub fn approve(deps:DepsMut, info:MessageInfo, recipient: Addr, value:Uint128) -> Result<Response, ContractError>{
        let balance = BALANCES.load(deps.storage, &info.sender)?;
        if balance < value{
            return Err(ContractError::Minbalance { balance: balance })
        }
        ALLOWANCES.update(deps.storage, (&info.sender, &recipient), |balance:Option<Uint128>| -> StdResult<_> {Ok(balance.unwrap_or_default() + value)})?;
        let resp = Response::new()
            .add_attribute("action", "approve")
            .add_attribute("sender", info.sender.to_string())
            .add_attribute("recipient", recipient.to_string())
            .add_attribute("value", value.to_string());
        Ok(resp) 
    }

}
