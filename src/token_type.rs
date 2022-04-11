use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const BLOCK_SIZE: usize = 256;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TokenType<A>{
    CustomToken{
        contract_add: A,
        token_code_hash: String
    },
    NativeToken{
        denom: String
    }
}

impl Canonize<TokenType<CanonicalAddr>> for TokenType<HumanAddr> {
    fn canonize(&self, api: &impl Api) -> StdResult<TokenType<CanonicalAddr>> {
        Ok(match self {
            Self::CustomToken {
                contract_addr,
                token_code_hash,
            } => TokenType::CustomToken {
                contract_addr: contract_addr.canonize(api)?,
                token_code_hash: token_code_hash.clone(),
            },
            Self::NativeToken { denom } => TokenType::NativeToken {
                denom: denom.clone(),
            },
        })
    }
}
impl Humanize<TokenType<HumanAddr>> for TokenType<CanonicalAddr> {
    fn humanize(&self, api: &impl Api) -> StdResult<TokenType<HumanAddr>> {
        Ok(match self {
            Self::CustomToken {
                contract_addr,
                token_code_hash,
            } => TokenType::CustomToken {
                contract_addr: contract_addr.humanize(api)?,
                token_code_hash: token_code_hash.clone(),
            },
            Self::NativeToken { denom } => TokenType::NativeToken {
                denom: denom.clone(),
            },
        })
    }
}