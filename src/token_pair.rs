
use crate::token_type::TokenType;
use serde::{Deserialize, Serialize};

use  scrt::{
    Api, StdResult, Querier,
    HumanAddr, Uint128, CanonicalAddr
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokenPair<A>(pub TokenType<A>, pub TokenType<A>);

impl Canonize<TokenPair<CanonicalAddr>> for TokenPair<HumanAddr> {
    fn canonize(&self, api: &impl Api) -> StdResult<TokenPair<CanonicalAddr>> {
        Ok(TokenPair(self.0.canonize(api)?, self.1.canonize(api)?))
    }
}

impl Humanize<TokenPair<HumanAddr>> for TokenPair<CanonicalAddr> {
    fn humanize(&self, api: &impl Api) -> StdResult<TokenPair<HumanAddr>> {
        Ok(TokenPair(self.0.humanize(api)?, self.1.humanize(api)?))
    }
}