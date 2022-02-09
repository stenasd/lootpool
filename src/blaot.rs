use crate::msg::{CountResponse, HandleMsg, IniDataResponse, InitMsg, QueryMsg};
use crate::state::{load, may_load, remove, save, Item, State, User, PREFIX_VIEW_KEY};
use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr,
    InitResponse, Querier, StdError, StdResult, Storage,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
/*
TODO ini setup

    account system with adding and reducing funds

*/
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let sender_raw = deps.api.canonical_address(&msg.address)?;
    let state = State {
        code_hash: msg.code_hash,
        address: sender_raw,
        owner: deps.api.canonical_address(&env.message.sender)?,
        accounts: Vec::new(),
        items: Vec::new(),
    };
    let mut key_store = PrefixedStorage::new(PREFIX_VIEW_KEY, &mut deps.storage);
    let message_sender = &deps.api.canonical_address(&env.message.sender)?;
    let string = "hello world";
    save(&mut key_store, message_sender.as_slice(), &string)?;
    debug_print!("Contract was initialized by {}", env.message.sender);
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Increment {} => try_increment(deps, env),
        HandleMsg::Reset { count } => try_reset(deps, env, count),
    }
}

pub fn try_increment<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    debug_print("count incremented successfully");
    Ok(HandleResponse::default())
}

pub fn try_reset<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    count: i32,
) -> StdResult<HandleResponse> {
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    debug_print("count reset successfully");
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?,),
    }
}

fn query_count<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<IniDataResponse> {
    let load_key: String = load(&deps.storage, PREFIX_VIEW_KEY)?;
    Ok(IniDataResponse {
        /*address: deps.api.human_address(&state.address).unwrap(),
        code_hash: state.code_hash,
        owner: deps.api.human_address(&state.owner).unwrap(),*/
        address: "asd".to_string(),
        code_hash: load_key,
        owner: "asd".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, HumanAddr, StdError};
    //tests that init when succeful and adress is stored as canonical and returned human
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);
        let msg = InitMsg {
            address: HumanAddr("testadress".to_string()),
            code_hash: "testhash".to_string(),
        };
        let env = mock_env("creator", &coins(1000, "earth"));
        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: IniDataResponse = from_binary(&res).unwrap();
        //assert_eq!(HumanAddr("creator".to_string()), value.address);
        assert_eq!("hello world".to_string(), value.code_hash);
        //assert_eq!(HumanAddr("creator".to_string()), value.owner);
    }
    #[test]
    fn increment() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {
            address: HumanAddr("testadress".to_string()),
            code_hash: "testhash".to_string(),
        };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // anyone can increment
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Increment {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }
}
