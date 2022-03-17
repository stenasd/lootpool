use crate::msg::{HandleMsg, IniDataResponse, InitMsg, QueryAccountResponse, QueryMsg};
use crate::state::{self, load, may_load, remove, save, Item, State, User, PREFIX_VIEW_KEY};
use cosmwasm_std::{StdError, StdResult, Storage};
use secret_toolkit::snip721::Metadata;
pub const CONFIG_KEY: &[u8] = b"config";
pub fn change_currency<S: Storage>(storage: &mut S, sender: String, amm: i32) -> StdResult<()> {
    let user: Option<User> = may_load(storage, sender.as_bytes())?;
    let mut acc = user.unwrap_or_else(|| User { currency: 0 });
    acc.currency += amm;
    if acc.currency >= 0 {
        save(storage, sender.as_bytes(), &acc)?;
    } else {
        return Err(StdError::Unauthorized { backtrace: None });
    }
    Ok(())
}

pub fn creat_item_from_metadata<S: Storage>(
    storage: &mut S,
    metadata: Metadata,
    tokenid: String,
) -> Item {
    let disc = metadata.description.unwrap();
    let mut disc1 = disc.split("%%");
    //disc tradeready&&value&&owner
    let tradedate = disc1.next().unwrap().to_string();
    let val = disc1.next().unwrap().to_string();
    let item: Item = Item {
        name: metadata.name.unwrap(),
        tradeReady: tradedate,
        value: val.parse::<i32>().unwrap(),
        tokenid: tokenid,
    };
    change_currency(
        storage,
        disc1.next().unwrap().to_string(),
        item.value.clone(),
    );
    return item;
}

pub fn get_buyin_price2(state: State) -> i32 {
    let items = state.items.clone();
    //let mut v: Vec<i32> = state.items.into_iter().map(|x| x.value).rev().collect();
    let mut vec_key: Vec<i32> = items.into_iter().map(|p| p.value).collect();
    vec_key.sort();
    return vec_key[vec_key.len() / 2];
}
pub fn get_buyin_price(state: State) -> i32 {
    let mut avg: i32;
    let mut sum: i32 = 0;
    let items = state.items.clone();
    //let mut v: Vec<i32> = state.items.into_iter().map(|x| x.value).rev().collect();
    let mut vec_key: Vec<i32> = items.into_iter().map(|p| p.value).collect();

    for x in &vec_key {
        sum = sum + x;
    }
    avg = sum as i32 / vec_key.len() as i32;
    //to ensure its allways rounded up
    avg = avg + 1;
    return avg as i32;
}
