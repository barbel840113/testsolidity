use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use scrt_link::ContractLink;

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {    
    pub owner: CanonicalAddr,
    pub pair_name: String,
    pub symbol: String,
    pub decimals: u64,
    pub token0: HumanAddr,
    pub token1: HumanAddr,
    pub factory: HumanAddr,
    // pub minimum_liquidity: u64,
    // pub price0_cumulative_last: f64,
    // pub price1_cumulative_last: f64,
    // pub block_timestamp_last: u64
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


pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
