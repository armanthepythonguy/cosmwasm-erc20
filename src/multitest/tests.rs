use cosmwasm_std::{Addr, coins, Uint128};
use cw_multi_test::App;

use crate::{msg::{BalanceOfResp, TotalSupplyResp, AllowanceResp}, ContractError};

use super::contract::CW_ERC20;


#[test]
fn mint_and_check(){
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage|{
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap()
    });

    let code_id = CW_ERC20::store_code(&mut app);

    let contract = CW_ERC20::instantiate(
        &mut app, 
        code_id, 
        &owner, 
        "CW-ERC20", 
        Some(String::from("Circle USD")), 
        Some(String::from("USDC")),
        Some(18), 
        Some(Uint128::new(120))
        ).unwrap();

    let mut resp = contract.balanceOf(&mut app, &sender, &owner).unwrap();
    assert_eq!(resp, BalanceOfResp{ balance_of : Uint128::new(120)});

    let mut totalsup = contract.total_supply(&mut app, &sender).unwrap();
    assert_eq!(totalsup, TotalSupplyResp{ total_supply : Uint128::new(120)});

    contract
        .mint(&mut app, &owner, &sender, &Uint128::new(1000))
        .unwrap();
    
    resp = contract.balanceOf(&mut app, &sender, &sender).unwrap();
    assert_eq!(resp, BalanceOfResp{ balance_of : Uint128::new(1000)});

    totalsup = contract.total_supply(&mut app, &sender).unwrap();
    assert_eq!(totalsup, TotalSupplyResp{ total_supply : Uint128::new(1120)});

    let err = contract
                            .mint(&mut app, &sender, &sender, &Uint128::new(10000))
                            .unwrap_err();

    assert_eq!(ContractError::Unauthorized { owner: owner.into() }, err);

}

#[test]
pub fn burn_and_test(){
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage|{
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap()
    });

    let code_id = CW_ERC20::store_code(&mut app);

    let contract = CW_ERC20::instantiate(
        &mut app, 
        code_id, 
        &owner, 
        "CW-ERC20", 
        Some(String::from("Circle USD")), 
        Some(String::from("USDC")),
        Some(18), 
        Some(Uint128::new(120))
        ).unwrap();

    let mut resp = contract.balanceOf(&mut app, &sender, &owner).unwrap();
    assert_eq!(resp, BalanceOfResp{ balance_of : Uint128::new(120)});

    let mut totalsup = contract.total_supply(&mut app, &sender).unwrap();
    assert_eq!(totalsup, TotalSupplyResp{ total_supply : Uint128::new(120)});

    contract
        .burn(&mut app, &owner, &Uint128::new(10))
        .unwrap();

    resp = contract.balanceOf(&mut app, &sender, &owner).unwrap();
    assert_eq!(resp, BalanceOfResp{ balance_of : Uint128::new(110)});

    totalsup = contract.total_supply(&mut app, &sender).unwrap();
    assert_eq!(totalsup, TotalSupplyResp{ total_supply : Uint128::new(110)});


    let err = contract
                .burn(&mut app, &owner, &Uint128::new(1000))
                .unwrap_err();

    assert_eq!(ContractError::Minbalance { balance: Uint128::new(110) }, err);
}

#[test]
pub fn transfer_and_test(){
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage|{
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap()
    });

    let code_id = CW_ERC20::store_code(&mut app);

    let contract = CW_ERC20::instantiate(
        &mut app, 
        code_id, 
        &owner, 
        "CW-ERC20", 
        Some(String::from("Circle USD")), 
        Some(String::from("USDC")),
        Some(18), 
        Some(Uint128::new(120))
        ).unwrap();

    let mut resp = contract.balanceOf(&mut app, &sender, &owner).unwrap();
    assert_eq!(resp, BalanceOfResp{ balance_of : Uint128::new(120)});

    let mut totalsup = contract.total_supply(&mut app, &sender).unwrap();
    assert_eq!(totalsup, TotalSupplyResp{ total_supply : Uint128::new(120)});

    contract
        .transfer(&mut app, &owner, &sender, &Uint128::new(10))
        .unwrap();
        
    resp = contract.balanceOf(&mut app, &sender, &owner).unwrap();
    assert_eq!(resp, BalanceOfResp { balance_of : Uint128::new(110)});
    resp = contract.balanceOf(&mut app, &sender, &sender).unwrap();
    assert_eq!(resp, BalanceOfResp { balance_of : Uint128::new(10)});

    let err = contract
                .transfer(&mut app, &sender, &owner, &Uint128::new(20))
                .unwrap_err();

    assert_eq!(err, ContractError::Minbalance { balance: Uint128::new(10) });
}

#[test]
pub fn approve_transfer_and_test(){
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");
    let third_party = Addr::unchecked("third_party");

    let mut app = App::new(|router, _api, storage|{
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap()
    });

    let code_id = CW_ERC20::store_code(&mut app);

    let contract = CW_ERC20::instantiate(
        &mut app, 
        code_id, 
        &owner, 
        "CW-ERC20", 
        Some(String::from("Circle USD")), 
        Some(String::from("USDC")),
        Some(18), 
        Some(Uint128::new(120))
        ).unwrap();

    let mut resp = contract.balanceOf(&mut app, &sender, &owner).unwrap();
    assert_eq!(resp, BalanceOfResp{ balance_of : Uint128::new(120)});

    let mut totalsup = contract.total_supply(&mut app, &sender).unwrap();
    assert_eq!(totalsup, TotalSupplyResp{ total_supply : Uint128::new(120)});

    contract
        .approve(&mut app, &owner, &sender, &Uint128::new(10))
        .unwrap();

    contract
        .transfer_from(&mut app, &third_party, &owner, &sender, &Uint128::new(5))
        .unwrap();

    resp = contract.balanceOf(&mut app, &sender, &owner).unwrap();
    assert_eq!(resp, BalanceOfResp{ balance_of : Uint128::new(115)});

    let allowance = contract.allowance(&mut app, &third_party, &owner, &sender).unwrap();
    assert_eq!(allowance, AllowanceResp{allowance: Uint128::new(5)});

    let err = contract
                .transfer_from(&mut app, &third_party, &owner, &sender, &Uint128::new(10))
                .unwrap_err();

    assert_eq!(err, ContractError::MinAllowance { allowance: Uint128::new(5) });
}