use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// InstantiateMsg is used to initialise the contract with the pet's name and owner.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub owner: Option<Addr>, 
}

// ExecuteMsg is used to perform actions on the pet.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetPassword {
        password: String,
    },
    Feed {
        amount: u8,
    },
    Play {
        amount: u8,
    },
    Rest {
        amount: u8,
    },
    Transfer {
        new_owner: String,
    },
}

//Read-only queries require a password that is set by calling set_password.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Checks if the pet is hungry if hunger_level >= 7.
    IsHungry {
        password: String,
    },
    GetStatus {
        password: String,
    },
}

// Query answer for read-only queries.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    IsHungry {
        is_hungry: bool,
    },
    GetStatus {
        name: String,
        hunger_level: u8,
        happiness_level: u8,
        energy_level: u8,
    },
}
