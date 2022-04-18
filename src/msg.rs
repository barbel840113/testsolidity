use fadroma::{
    scrt_callback::Callback,
    scrt_link::{ContractLink, ContractInstantiationInfo},
    scrt::{Binary, Decimal, HumanAddr, Uint128},
};

use serde::{Deserialize, Serialize};

use token::{TokenPair, TokenPairAmount, TokenTypeAmount};

pub mod paircontract{
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct InitMsg {       
        pub pair: TokenPair<HumanAddr>,
        pub lp_token_contract: ContractInstantiationInfo,      
        pub factory_info: ContractLink<HumanAddr>,
        pub callback: Callback<HumanAddr>,
        pub prng_seed: Binary,
        pub entropy: Binary,
    }
    
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum HandleMsg {
        AddLiquidityToPairContract {
            deposit: TokenPairAmount<HumanAddr>,
        },
        SwapTokens {
            /// The token type to swap from.
            offer: TokenTypeAmount<HumanAddr>,
            expected_return: Option<Uint128>,
            to: Option<HumanAddr>,
        },
        // SNIP20 receiver interface
        ReceiveCallback {
            from: HumanAddr,
            msg: Option<Binary>,
            amount: Uint128,
        },
        // Sent by the LP token contract so that we can record its address.
        OnLpTokenInitAddr
    }
    
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum InvokeMsg {
        SwapTokens {
            expected_return: Option<Uint128>,
            to: Option<HumanAddr>,
        },
        RemoveLiquidity {
            recipient: HumanAddr,
        },
    }
    
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        PairInfo,    
    }
    
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsgResponse {
        PairInfo {
            liquidity_token: ContractLink<HumanAddr>,
            factory: ContractLink<HumanAddr>,
            pair: TokenPair<HumanAddr>,
            amount_0: Uint128,
            amount_1: Uint128,
            total_liquidity: Uint128,
            contract_version: u32,
        },
    }
   
}

