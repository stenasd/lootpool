use cosmwasm_std::{Api, CanonicalAddr, HumanAddr, ReadonlyStorage, StdError, StdResult, Storage};

use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{any::type_name, collections::HashSet};

use secret_toolkit::{
    serialization::{Bincode2, Serde},
    snip721::Metadata,
    storage::{AppendStore, AppendStoreMut},
};

pub const PREFIX_VIEW_KEY: &[u8] = b"viewkey";

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct State {
    pub admin: HumanAddr,
    pub code_hash: String,
    pub nft: HumanAddr,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Item {
    pub name: String,
    pub tradeReady: String,
    pub value: i32,
    pub tokenid:String
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct User {
    // wich block items is tradeable
    pub currency: i32,
}

/*
    let message_sender = &deps.api.canonical_address(&env.message.sender)?;
    let mut key_store = PrefixedStorage::new(PREFIX_VIEW_KEY, &mut deps.storage);
    save(&mut key_store, message_sender.as_slice(), &vk.to_hashed())?;
*/

//sten saves the stuct with the key
pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(key, &Bincode2::serialize(value)?);
    Ok(())
}

pub fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}
//sten load data to struct
pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}
//Use with adress as key to store
pub fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Bincode2::deserialize(&value).map(Some),
        None => Ok(None),
    }
}
