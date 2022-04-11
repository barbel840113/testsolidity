use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, HumanAddr};
use  scrt::{
    Api, StdResult, Querier,  HumanAddr, Uint128, CanonicalAddr
};

use cosmwasm_std::{Storage, Querier, Extern, debug_print,Api, Binary, Env, HandleResponse, InitResponse,
    StdError, StdResult};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use token_pair::{TokenPair};

pub static CONFIG_KEY: &[u8] = b"config";
/// Code hashes for MGMT and SNIP20
pub type CodeHash = String;
pub type ContractLink<T> = (T, CodeHash);



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct State {    
    pub owner: CanonicalAddr,
    pub factory_Info: String,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config<A: Clone> {
    pub factory_info: ContractLink<A>,
    pub lp_token_info: ContractLink<A>,
    pub pair: TokenPair<A>,
    pub name: String, 
    pub symbol: String,
    pub contract_addr: A,
}

impl Canonize<Config<CanonicalAddr>> for Config<HumanAddr> {
    fn canonize (&self, api: &impl Api) -> StdResult<Config<CanonicalAddr>> {
        Ok(Config {
            factory_info:  self.factory_info.canonize(api)?,
            lp_token_info: self.lp_token_info.canonize(api)?,
            pair:          self.pair.canonize(api)?,
            contract_addr: self.contract_addr.canonize(api)?,
           // viewing_key:   self.viewing_key.clone()
        })
    }
}

impl Humanize<Config<HumanAddr>> for Config<CanonicalAddr> {
    fn humanize (&self, api: &impl Api) -> StdResult<Config<HumanAddr>> {
        Ok(Config {
            factory_info:  self.factory_info.humanize(api)?,
            lp_token_info: self.lp_token_info.humanize(api)?,
            pair:          self.pair.humanize(api)?,
            contract_addr: self.contract_addr.humanize(api)?,
          //  viewing_key:   self.viewing_key.clone()
        })
    }
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
