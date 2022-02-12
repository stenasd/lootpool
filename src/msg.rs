use crate::state::Item;
use cosmwasm_std::{Binary, HumanAddr};
use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub address: HumanAddr,
    pub code_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddFunds {
        ammount: i32,
    },
    BatchReceiveNft {
        /// address that sent the NFTs
        sender: HumanAddr,
        /// previous owner of the NFTs
        from: HumanAddr,
        /// list of NFTs sent from the previous owner
        token_ids: Vec<String>,
        /// msg specified when sending
        msg: Option<Binary>,
    },
    StartLootPool{
        // how many times to enter lottery
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetSettings {},
    QueryAccount { adress: HumanAddr },
    QueryPool {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IniDataResponse {
    pub nft: HumanAddr,
    pub code_hash: String,
    pub admin: HumanAddr,
    pub items: Vec<Item>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LootPoolResponse {
    pub buyin: i32,
    pub items: Vec<Item>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryAccountResponse {
    pub adress: HumanAddr,
    pub funds: i32,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    WonItem{ item: Item }
}