use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Map};

// Hexadecimal address: 0x000ff4572e
// Bech32: cudos1xf650q8wm97nwqfs0fatmfeaw2hx0xv7y5j40s
// "cudos1xf650q8wm97nwqfs0fatmfeaw2hx0xv7y5j40s" ---> Addr("cudos1xf650q8wm97nwqfs0fatmfeaw2hx0xv7y5j40s")

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Todo {
    pub description: String,
    pub owner: Addr,
    pub done: bool,
}

pub const STATE: Map<&str, Todo> = Map::new("state");

// key --> value
// dictionary, json, series of key-value pair
// id ---> todo { description, doneOrNot, owner }