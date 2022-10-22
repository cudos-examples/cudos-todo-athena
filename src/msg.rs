use cosmwasm_schema::{cw_serde};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateTodo { id: String, description: String },
}

#[cw_serde]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TodoResponse {
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub description: String,
    pub owner: Addr,
    pub done: bool,
}