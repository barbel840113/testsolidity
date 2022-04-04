use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Storage, Querier, Extern, debug_print,load, save to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

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
    pub token0: A,
    pub token1: A,
    pub factory: A,
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

pub(crate) fn store_config <S: Storage, A: Api, Q: Querier>(
    deps:   &mut Extern<S, A, Q>,
    config: &Config<HumanAddr>
) -> StdResult<()> {
    save(&mut deps.storage, CONFIG_KEY, &config.canonize(&deps.api)?)
}

pub(crate) fn load_config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> StdResult<Config<HumanAddr>> {
    let result: Config<CanonicalAddr> = load(&deps.storage, CONFIG_KEY)?.ok_or(
        StdError::generic_err("Config doesn't exist in storage.")
    )?;
    result.humanize(&deps.api)
}
