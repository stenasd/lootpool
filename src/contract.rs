use crate::controller::{change_currency, creat_item_from_metadata};
use crate::msg::{
    HandleAnswer, HandleMsg, IniDataResponse, InitMsg, LootPoolResponse, QueryAccountResponse,
    QueryMsg,
};
use crate::state::{load, may_load, remove, save, Item, State, User, PREFIX_VIEW_KEY};
use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, CanonicalAddr, CosmosMsg, Env, Extern, HandleResponse,
    HandleResult, HumanAddr, InitResponse, Querier, StdError, StdResult, Storage,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use secret_toolkit::snip721::{
    batch_transfer_nft_msg, nft_info_query, register_receive_nft_msg, Metadata,
};

/*
    TODO change so currency is changed for owner of nft and add to pool.

*/
pub const CONFIG_KEY: &[u8] = b"config";

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    /*
    let message_sender = &deps.api.canonical_address(&env.message.sender)?;
    save(&mut deps.storage, message_sender.as_slice(), &string)?;*/
    let state = State {
        admin: env.message.sender,
        code_hash: msg.code_hash,
        nft: msg.address,
        items: Vec::new(),
    };
    save(&mut deps.storage, CONFIG_KEY, &state)?;
    Ok(InitResponse {
        messages: vec![register_receive_nft_msg(
            env.contract_code_hash,
            Some(true),
            None,
            256,
            state.code_hash.clone(),
            state.nft.clone(),
        )?],
        log: vec![],
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSettings {} => to_binary(&query_settings(deps)?),
        QueryMsg::QueryPool {} => to_binary(&query_pool(deps)?),
        QueryMsg::QueryAccount { adress } => to_binary(&query_account(deps, adress)?),
    }
}
fn query_settings<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<IniDataResponse> {
    let state: State = load(&deps.storage, CONFIG_KEY)?;
    Ok(IniDataResponse {
        /*address: deps.api.human_address(&state.address).unwrap(),
        code_hash: state.code_hash,
        owner: deps.api.human_address(&state.owner).unwrap(),*/
        admin: state.admin,
        code_hash: state.code_hash,
        nft: state.nft,
        items: state.items,
    })
}

fn query_pool<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<LootPoolResponse> {
    let state: State = load(&deps.storage, CONFIG_KEY)?;
    let items = state.items.clone();
    let mut v: Vec<i32> = state.items.into_iter().map(|x| x.value).rev().collect();
    v.sort();
    let median = v.len() / 2;
    Ok(LootPoolResponse {
        buyin: median as i32,
        items: items,
    })
}
fn query_account<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    adress: HumanAddr,
) -> StdResult<QueryAccountResponse> {
    let sender = adress.as_str();
    let user: Option<User> = may_load(&deps.storage, sender.as_bytes())?;
    let account = user.unwrap_or_else(|| User { currency: 0 });
    Ok(QueryAccountResponse {
        /*address: deps.api.human_address(&state.address).unwrap(),
        code_hash: state.code_hash,
        owner: deps.api.human_address(&state.owner).unwrap(),*/
        adress: adress,
        funds: account.currency,
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddFunds { ammount } => change_funds(deps, env, ammount),
        HandleMsg::BatchReceiveNft {
            from,
            token_ids,
            msg,
            ..
        } => try_receive(deps, env, from, &token_ids, msg),
        HandleMsg::StartLootPool {} => try_start_lootpool(deps, env),
    }
}
pub fn try_start_lootpool<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> HandleResult {
    let mut messages: Vec<CosmosMsg> = Vec::new();

    //get balance check if you can afford
    // randomize array
    // remove from pool
    // 

    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::WonItem {
            item: card_contract.address,
        })?),
    })
}
pub fn try_receive<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    from: HumanAddr,
    token_ids: &[String],
    msg: Option<Binary>,
) -> HandleResult {
    //let include_expired = None;
    //example message tradeready&&value&&owner
    let block_size = 256;
    let mut iter = token_ids.iter();
    let mut state: State = load(&deps.storage, CONFIG_KEY)?;
    for tokenid in iter {
        let metadata = nft_info_query(
            &deps.querier,
            tokenid.to_string(),
            block_size,
            state.code_hash.clone(),
            state.nft.clone(),
        )?;

        //let item = creat_item_from_metadata(&mut deps.storage, metadata, tokenid.to_string());
        // Updates user currencie with items value
        state.items.push(creat_item_from_metadata(
            &mut deps.storage,
            metadata,
            tokenid.to_string(),
        ));
        //procces value
        /*
            tradeRady:date when trade ready
            name:name of skin and price
            adress: get it from "previos owner" aka from:humanAddr

        */
    }
    save(&mut deps.storage, CONFIG_KEY, &state)?;
    return Ok(HandleResponse::default());
}
pub fn change_funds<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    ammount: i32,
) -> StdResult<HandleResponse> {
    change_currency(
        &mut deps.storage,
        env.message.sender.as_str().to_string(),
        ammount,
    );
    Ok(HandleResponse::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, HumanAddr, StdError};
    use secret_toolkit::snip721::{
        batch_transfer_nft_msg, nft_info_query, register_receive_nft_msg, Metadata,
    };
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
        init(&mut deps, env, msg).unwrap();
        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetSettings {}).unwrap();
        let value: IniDataResponse = from_binary(&res).unwrap();
        //assert_eq!(HumanAddr("creator".to_string()), value.address);
        assert_eq!("testhash".to_string(), value.code_hash);
        //assert_eq!(HumanAddr("creator".to_string()), value.owner);
    }

    #[test]
    fn change_account_funds() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {
            address: HumanAddr("testadress".to_string()),
            code_hash: "testhash".to_string(),
        };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        let auth_env = mock_env("creator", &coins(2, "token"));
        let msg = HandleMsg::AddFunds { ammount: 11 };
        let _res = handle(&mut deps, auth_env, msg).unwrap();

        let auth_env = mock_env("creator", &coins(2, "token"));
        //let msg = HandleMsg::AddFunds { ammount: 0 };
        //let _res = handle(&mut deps, auth_env, msg).unwrap();

        let res = query(
            &deps,
            QueryMsg::QueryAccount {
                adress: HumanAddr("creator".to_string()),
            },
        )
        .unwrap();
        let value: QueryAccountResponse = from_binary(&res).unwrap();
        assert_eq!(11, value.funds);
    }
    #[test]
    fn creat_nft() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {
            address: HumanAddr("testadress".to_string()),
            code_hash: "testhash".to_string(),
        };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();
        let a = Metadata {
            name: Some("name".to_string()),
            image: Some("image".to_string()),
            //timeready%%value%%owner
            description: Some("1942-09-06%%100%%55%%creator".to_string()),
        };
        let item = creat_item_from_metadata(&mut deps.storage, a, "0".to_string());
        let res = query(
            &deps,
            QueryMsg::QueryAccount {
                adress: HumanAddr("creator".to_string()),
            },
        )
        .unwrap();
        assert_eq!(
            Item {
                name: "name".to_string(),
                value: 100,
                tradeReady: "1942-09-06".to_string(),
                tokenid: "0".to_string()
            },
            item
        );
    }
}
