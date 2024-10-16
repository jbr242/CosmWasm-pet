use schemars::JsonSchema;
use secret_toolkit::storage::Item;
use serde::{Deserialize, Serialize};
use cosmwasm_std::CanonicalAddr;

pub static OWNER_KEY: &[u8] = b"OWNER";
pub static OWNER: Item<CanonicalAddr> = Item::new(OWNER_KEY);

pub static PET_KEY: &[u8] = b"pet";
pub static PET: Item<Pet> = Item::new(PET_KEY);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pet {
    pub name: String,
    pub hunger_level: u8, // 0-10
    pub happiness_level: u8, // 0-10
    pub energy_level: u8, // 0-10
    pub last_action_block: u64,
}

pub static PASWORD_KEY: &[u8] = b"password";
pub static PASSWORD: Item<String> = Item::new(PASWORD_KEY);
