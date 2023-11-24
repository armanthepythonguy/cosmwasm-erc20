use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg{
    #[serde(default)]
    pub name : String,
    pub symbol : String,
    pub decimal : u8,
    pub total_supply : Uint128
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NameResp)]
    Name {},
    #[returns(SymbolResp)]
    Symbol {},
    #[returns(DecimalResp)]
    Decimal {},
    #[returns(TotalSupplyResp)]
    TotalSupply {},
    #[returns(BalanceOfResp)]
    BalanceOf {
        owner : Addr
    },
    #[returns(AllowanceResp)]
    Allowance {
        sender : Addr,
        recipient : Addr 
    },
}

#[cw_serde]
pub struct NameResp{
    pub name:String
}

#[cw_serde]
pub struct SymbolResp{
    pub symbol:String
}

#[cw_serde]
pub struct DecimalResp{
    pub decimal:u8
}

#[cw_serde]
pub struct TotalSupplyResp{
    pub total_supply:Uint128
}

#[cw_serde]
pub struct BalanceOfResp{
    pub balance_of:Uint128
}

#[cw_serde]
pub struct AllowanceResp{
    pub allowance:Uint128
}

#[cw_serde]
pub enum ExecMsg{
    Mint{
        recipient: Addr,
        value: Uint128
    },
    Burn{
        value: Uint128
    },
    Transfer{
        recipient : Addr,
        value : Uint128
    },
    TransferFrom{
        sender : Addr,
        recipient : Addr,
        value : Uint128
    },
    Approve{
        recipient : Addr,
        value : Uint128
    }
}