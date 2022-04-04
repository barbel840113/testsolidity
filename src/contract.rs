use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage
};

use cosmwasm_std::{CanonicalAddr, HumanAddr};
use secret_toolkit::snip20::{register_receive_msg, token_info_query, transfer_msg, TokenInfo};
use crate::msg::{SymbolResponse, HandleMsg, InitMsg, QueryMsg, FactoryResponse,     
    PairNameResponse, DecimalsResponse, TokensResponse};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {      
        symbol: msg.symbol,
        pair_name: msg.pair_name,
        decimals: msg.decimals,
        token0: msg.token0,
        token1: msg.token1,  
        factory: msg.factory,
        // price0_cumulative_last: 0.0,
        // price1_cumulative_last: 0.0,
        // minimum_liquidity: msg.min_liquidity,
        // block_timestamp_last: env.block.time,              
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Increment {} => try_increment(deps, env)        
    }
}

pub fn try_increment<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        //state.count += 1;
       // debug_print!("count = {}", state.count);
        Ok(state)
    })?;

    debug_print("count incremented successfully");
    Ok(HandleResponse::default())
}

pub fn try_reset<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env
) -> StdResult<HandleResponse> {
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    config(&mut deps.storage).update(|mut state| {
        if sender_address_raw != state.owner {
            return Err(StdError::Unauthorized { backtrace: None });
        }        
        Ok(state)
    })?;
    debug_print("count reset successfully");
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPairName {} => to_binary(&query_pairname(deps)?),
        QueryMsg::GetSymbol {} => to_binary(&query_symbolname(deps)?),
        QueryMsg::GetDecimals {} => to_binary(&query_decimals(deps)?),
        QueryMsg::GetFactory {} =>  to_binary(&query_factory(deps)?),
        // QueryMsg::GetReservers {} => to_binary(&query_symbolname(deps)?),
        // QueryMsg::GetOwner {} => to_binary(&query_symbolname(deps)?),
        QueryMsg::GetTokens {} => to_binary(&query_token(deps)?),
    }
}

fn query_pairname<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<PairNameResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(PairNameResponse { pair_name: state.pair_name })
}

fn query_symbolname<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<SymbolResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(SymbolResponse { symbol: state.symbol })
}

fn query_token<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<TokensResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(TokensResponse { token0: state.token0, token1: state.token1 })
}

fn query_decimals<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<DecimalsResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(DecimalsResponse { decimals: state.decimals })
}

fn query_factory<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<FactoryResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(FactoryResponse { factory: state.factory })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let env = mock_env("creator", &coins(1000, "earth"));
        let address = HumanAddr("test".to_string());
        let mut deps = mock_dependencies(20, &[]);            
        let msg = InitMsg { pair_name: "TestPair".to_string(), symbol: "TH".to_string(), decimals: 18,
         factory: HumanAddr("factory".to_string()), token0: HumanAddr("token0".to_string()), token1: HumanAddr("token1".to_string())};       
        let _res = init(&mut deps, env, msg).unwrap();
        let queryPairName = query(&deps, QueryMsg::GetPairName {}).unwrap();
        let pair_name_result : PairNameResponse = from_binary(&queryPairName).unwrap();
        assert_eq!("TestPair", pair_name_result.pair_name);

        let query_symbol = query(&deps,QueryMsg::GetSymbol {}).unwrap();
        let symbol_result: SymbolResponse = from_binary(&query_symbol).unwrap();
        assert_eq!("TH", symbol_result.symbol);

        let query_token0 = query(&deps, QueryMsg::GetTokens {}).unwrap();
        let query_token_result: TokensResponse = from_binary(&query_token0).unwrap();
        assert_eq!(HumanAddr("token0".to_string()), query_token_result.token0);

        let query_token0 = query(&deps, QueryMsg::GetTokens {}).unwrap();
        let query_token_result: TokensResponse = from_binary(&query_token0).unwrap();
        assert_eq!(HumanAddr("token1".to_string()), query_token_result.token1);

        let query_factory = query(&deps, QueryMsg::GetFactory {}).unwrap();
        let query_factory_result: FactoryResponse = from_binary(&query_factory).unwrap();
        assert_eq!(HumanAddr("factory".to_string()), query_factory_result.factory);
        // we can just call .unwrap() to assert this was a success
        // let res = init(&mut deps, env, msg).unwrap();
        // assert_eq!(0, res.messages.len());

        // it worked, let's query the state
       // let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        //let value: CountResponse = from_binary(&res).unwrap();
        //assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        // let mut deps = mock_dependencies(20, &coins(2, "token"));        
        // let msg = InitMsg { 
        //     pair_name: "TestPair".to_string(), 
        //     symbol: "TH".to_string(),             
        //     decimals: 18,
        //     token0: HumanAddr("TEST".to_string())
        // };
        // let env = mock_env("creator", &coins(2, "token"));
        // let _res = init(&mut deps, env, msg);
       
        // // anyone can increment
        // let env = mock_env("anyone", &coins(2, "token"));
        // let msg = HandleMsg::Increment {};
        // let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        //let res = query(&deps, QueryMsg::GetCount {}).unwrap();
       // let value: CountResponse = from_binary(&res).unwrap();
//assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
       

        // let msg = InitMsg { count: 17 };
        // let env = mock_env("creator", &coins(2, "token"));
        // let _res = init(&mut deps, env, msg).unwrap();

        // // not anyone can reset
        // let unauth_env = mock_env("anyone", &coins(2, "token"));
        // let msg = HandleMsg::Reset { count: 5 };
        // let res = handle(&mut deps, unauth_env, msg);
        // match res {
        //     Err(StdError::Unauthorized { .. }) => {}
        //     _ => panic!("Must return unauthorized error"),
        // }

        // // only the original creator can reset the counter
        // let auth_env = mock_env("creator", &coins(2, "token"));
        // let msg = HandleMsg::Reset { count: 5 };
        // let _res = handle(&mut deps, auth_env, msg).unwrap();

        // // should now be 5
        // let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(5, value.count);
    }
}
