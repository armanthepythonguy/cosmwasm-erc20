use cosmwasm_schema::write_api;
use cw_erc20::msg::{InstantiateMsg, ExecMsg, QueryMsg};



fn main(){
    write_api!{
        instantiate : InstantiateMsg,
        execute : ExecMsg,
        query : QueryMsg
    }
}