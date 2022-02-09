use crate::msg::{HandleMsg, IniDataResponse, InitMsg, QueryAccountResponse, QueryMsg};
use crate::state::{load, may_load, remove, save, Item, State, User, PREFIX_VIEW_KEY};
use cosmwasm_std::{StdError, StdResult, Storage};
use secret_toolkit::snip721::Metadata;

pub fn change_currency<S: Storage>(
    storage: &mut S,
    sender: String,
    amm: i32,
) -> StdResult<()> {
    let user: Option<User> = may_load(storage, sender.as_bytes())?;
    let mut acc = user.unwrap_or_else(|| User {
        currency: 0,
    });
    acc.currency += amm;
    if acc.currency < 0 {
            return Err(StdError::Unauthorized { backtrace: None });
    }
    save(storage, sender.as_bytes(), &acc)?;
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
