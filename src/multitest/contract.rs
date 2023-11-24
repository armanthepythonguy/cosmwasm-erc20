use cosmwasm_std::{Addr, StdResult, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::{ execute, query, instantiate, msg::{InstantiateMsg, ExecMsg, BalanceOfResp, QueryMsg, TotalSupplyResp, AllowanceResp}, ContractError};



pub struct CW_ERC20(Addr);

impl CW_ERC20{

    pub fn addr(&self) -> &Addr{
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64{
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        name: impl Into<Option<String>>,
        symbol: impl Into<Option<String>>,
        decimal : impl Into<Option<u8>>,
        total_supply: impl Into<Option<Uint128>>
    ) -> StdResult<Self>{
        let name = name.into().unwrap_or_default();
        let symbol = symbol.into().unwrap_or_default();
        let decimal = decimal.into().unwrap_or_default();
        let total_supply = total_supply.into().unwrap_or_default();

        app.instantiate_contract(
            code_id, 
            sender.clone(), 
            &InstantiateMsg{
                name: name,
                symbol: symbol,
                decimal: decimal,
                total_supply: total_supply
            }, 
            &[], 
            label, 
            None
        )
        .map(CW_ERC20)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn mint(
        &self,
        app: &mut App,
        sender: &Addr,
        recipient : &Addr,
        value: &Uint128
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Mint { recipient: recipient.clone(), value: value.clone() }, &[])
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn burn(
        &self,
        app: &mut App,
        sender: &Addr,
        value: &Uint128
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Burn { value: value.clone() }, &[])
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn transfer(
        &self,
        app: &mut App,
        sender: &Addr,
        recipient : &Addr,
        value: &Uint128
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Transfer { recipient: recipient.clone(), value: value.clone() } , &[])
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn approve(
        &self,
        app: &mut App,
        sender: &Addr,
        recipient : &Addr,
        value: &Uint128
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Approve { recipient: recipient.clone(), value: value.clone() } , &[])
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn transfer_from(
        &self,
        app: &mut App,
        sender: &Addr,
        owner: &Addr,
        recipient : &Addr,
        value: &Uint128
    ) -> Result<(), ContractError>{
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::TransferFrom { sender: owner.clone(), recipient: recipient.clone(), value: value.clone() } , &[])
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn balanceOf(
        &self,
        app: &mut App,
        sender: &Addr,
        owner: &Addr
    ) -> StdResult<BalanceOfResp>{
        app.wrap().query_wasm_smart(self.0.clone(), &QueryMsg::BalanceOf { owner: owner.clone() })
    }

    #[track_caller]
    pub fn total_supply(
        &self,
        app: &mut App,
        sender: &Addr,
    ) -> StdResult<TotalSupplyResp>{
        app.wrap().query_wasm_smart(self.0.clone(), &QueryMsg::TotalSupply {  })
    }

    #[track_caller]
    pub fn allowance(
        &self,
        app: &mut App,
        sender: &Addr,
        owner: &Addr,
        recipient : &Addr
    ) -> StdResult<AllowanceResp>{
        app.wrap().query_wasm_smart(self.0.clone(), &QueryMsg::Allowance { sender: owner.clone(), recipient: recipient.clone() })
    }

}

impl From<CW_ERC20> for Addr{
    fn from(contract: CW_ERC20) -> Self{
        contract.0
    }
}