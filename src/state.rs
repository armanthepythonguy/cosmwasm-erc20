use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Uint128};


pub const NAME:Item<String> = Item::new("name");
pub const SYMBOL:Item<String> = Item::new("symbol");
pub const DECIMAL:Item<u8> = Item::new("decimal");
pub const TOTALSUPPLY:Item<Uint128> = Item::new("totalsupply");
pub const OWNER:Item<Addr> = Item::new("owner");
pub const BALANCES:Map<&Addr, Uint128> = Map::new("balances");
pub const ALLOWANCES:Map<(&Addr, &Addr), Uint128> = Map::new("allowances");