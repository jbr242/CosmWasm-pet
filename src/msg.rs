use cosmwasm_std::{Addr, Uint128, Uint64};
use schemars::JsonSchema;
use secret_toolkit::permit::Permit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Admin-only
    SetAuction {
        secret: String,
        minimum_bid: Uint128,
        end_time: Uint64,
    },
    StartAuction { },

    // Any user
    Bid { },
    Withdraw { },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AuctionInfo { },
    WithPermit {
        /// permit used to verify querier identity
        permit: Permit,
        /// query to perform
        query: QueryWithPermit,
    },
}

/// queries using permits
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryWithPermit {
    GetSecret { },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    AuctionInfo {
        started: bool,
        minimum_bid: Option<Uint128>,
        end_time: Option<Uint64>,
    },
    GetSecret {
        secret: String,
    },
}