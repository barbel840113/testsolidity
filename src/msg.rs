use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{HumanAddr};
use scrt_link::ContractLink;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
   pub config: Config<HumanAddr>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Increment {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetConfig {}
}


#[derive(Serialize, Deserialize,PartialEq, JsonSchema)]
pub(crate) struct Config<A: Clone> {
    pub token0: ContractLink<A>,
    pub token1: ContractLink<A>,
    pub factory: ContractLink<A>,
    pub name: String, 
    pub symbol: String,
    pub current_contract_address: A
}

