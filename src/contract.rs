#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Order};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Entry, TodoResponse};
use crate::state::{STATE, Todo};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cudos-todo-athena";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Allows you to call in to the smart contract logic
// Entry point or a constructor for your smart contract

// You can initialize or instantiate the state of your application
// In-contract State

// Web2:
// REST API
// {
//  "username": "bob"
// }
// The server would consume this and it would process and return a response like 200 status code

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

// Mutate the state or the current situation of the smart contract
// Pattern match the message we receive

// {
//  "message": "increment",
//  "data": {
//   "count": 32
//  }
// }
// JSON data input to REST API

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateTodo { id, description } => try_create_todo(deps, info.sender, description, id),
    }
}

pub fn try_create_todo(deps: DepsMut, owner: Addr, description: String, id: String) -> Result<Response, ContractError> {
    let todo = Todo {
        description: description,
        done: false,
        owner: owner
    };

    STATE.save(deps.storage, &id, &todo);

    Ok(Response::new().add_attribute("action", "create_todo"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&get_todos(deps)?),
    }
}

pub fn get_todos(deps: Deps) -> StdResult<TodoResponse> {
    let all: StdResult<Vec<_>> = STATE
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
        
    let mut resp: Vec<Entry> = Vec::new();
    for (id, data) in all? {
        resp.push(Entry {
            description: data.description,
            owner: data.owner,
            done: data.done
        });
    }

    Ok(TodoResponse { entries: resp })
}
