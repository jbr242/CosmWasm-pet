use schemars::JsonSchema;
use secret_toolkit::storage::{Item, Keymap};
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CanonicalAddr, StdResult, Storage, Timestamp};

pub static ADMIN_KEY: &[u8] = b"admin";
pub static ADMIN: Item<CanonicalAddr> = Item::new(ADMIN_KEY);

pub static AUCTION_KEY: &[u8] = b"auction";
pub static AUCTION: Item<Auction> = Item::new(AUCTION_KEY);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Auction {
    pub end_time: Timestamp,
    pub minimum_bid: u128,
    pub secret: String,
}

pub static AUCTION_STARTED_KEY: &[u8] = b"started";
pub static AUCTION_STARTED: Item<bool> = Item::new(AUCTION_STARTED_KEY);

pub static SALE_COMPLETED_KEY: &[u8] = b"complete";
pub static SALE_COMPLETED: Item<bool> = Item::new(SALE_COMPLETED_KEY);

pub static HIGHEST_BID_KEY: &[u8] = b"highest";
pub static HIGHEST_BID: Item<HighestBid> = Item::new(HIGHEST_BID_KEY);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct HighestBid {
    pub bidder: CanonicalAddr,
    pub amount: u128,
}

pub static BIDS_KEY: &[u8] = b"bids";
pub static BIDS: Keymap<CanonicalAddr, u128> = Keymap::new(BIDS_KEY);

pub fn add_to_bid(
    storage: &mut dyn Storage,
    bidder: &CanonicalAddr,
    amount: u128,
) -> StdResult<u128> {
    let new_bid;
    if let Some(current_bid) = BIDS.get(storage, bidder) {
        new_bid = current_bid + amount;
    } else {
        new_bid = amount;
    }
    BIDS.insert(storage, bidder, &new_bid)?;
    Ok(new_bid)
}