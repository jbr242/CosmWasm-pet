
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Storage,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryAnswer, QueryMsg};
use crate::state::{Pet, PET, PASSWORD, OWNER};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let owner = msg.owner.unwrap_or(info.sender);
    let owner = deps.api.addr_canonicalize(owner.as_str())?;
    OWNER.save(deps.storage, &owner)?;

    if msg.name.len() < 1 {
        return Err(StdError::generic_err("Name cannot be empty"));
    }

    let pet = Pet {
        name: msg.name,
        hunger_level: 0,
        happiness_level: 10,
        energy_level: 10,
        last_action_block: env.block.height,
    };

    PET.save(deps.storage, &pet)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetPassword { password } => try_set_password(deps, info, password),
        ExecuteMsg::Feed { amount } => try_feed(deps, env, info, amount),
        ExecuteMsg::Play { amount } => try_play(deps, env, info, amount),
        ExecuteMsg::Rest { amount } => try_rest(deps, env, info, amount),
        ExecuteMsg::Transfer { new_owner } => try_transfer(deps, info, new_owner),
    }
}

pub fn try_set_password(deps: DepsMut, info: MessageInfo, password: String) -> StdResult<Response> {
    let owner = deps.api.addr_humanize(&OWNER.load(deps.storage)?)?;
    if info.sender != owner {
        return Err(StdError::generic_err("Unauthorised"));
    }
    PASSWORD.save(deps.storage, &password)?;

    Ok(Response::default())
}

fn update_state(storage: &mut dyn Storage, env: Env) -> StdResult<Pet> {
    let mut pet = PET.load(storage)?;

    let mut blocks_passed = env.block.height - pet.last_action_block;
    //convert blocks_passed to be max 10
    blocks_passed = blocks_passed.min(10);

    
    // if hunger level above 10, make it 10 with min func
    pet.hunger_level = (pet.hunger_level + blocks_passed as u8).min(10);
    
    // if reducing happiness level would make it negative, make it 0
    if pet.happiness_level >= blocks_passed as u8 {
        pet.happiness_level -= blocks_passed as u8;
    } else {
        pet.happiness_level = 0;
    }

    if pet.energy_level > 10 {
        pet.energy_level = 10;
    }

    pet.last_action_block = env.block.height;

    PET.save(storage, &pet)?;

    Ok(pet)
}



pub fn try_feed(deps: DepsMut, env: Env, info: MessageInfo, amount: u8) -> StdResult<Response> {
    let owner = deps.api.addr_humanize(&OWNER.load(deps.storage)?)?;
    if info.sender != owner {
        return Err(StdError::generic_err("Unauthorised"));
    }

    if amount > 10 {
        return Err(StdError::generic_err("Amount must be between 0 and 10"));
    }

    let mut pet = update_state(deps.storage, env).unwrap();
    pet.hunger_level = (pet.hunger_level + amount).min(10);
    PET.save(deps.storage, &pet)?;

    Ok(Response::default())
}

pub fn try_play(deps: DepsMut, env: Env, info: MessageInfo, amount: u8) -> StdResult<Response> {
    let owner = deps.api.addr_humanize(&OWNER.load(deps.storage)?)?;
    if info.sender != owner {
        return Err(StdError::generic_err("Unauthorised"));
    }

    //Increase happiness_level by amount (ensure it doesn’t exceed 10).
    if amount > 10 {
        return Err(StdError::generic_err("Amount must be between 0 and 10"));
    }

    let mut pet = update_state(deps.storage, env).unwrap();
    //Decrease energy_level by 1 (ensure it doesn’t go below 0).
    if pet.energy_level < 1 {
        return Err(StdError::generic_err("Not enough energy"));
    }
    pet.happiness_level = (pet.happiness_level + amount).min(10);
    pet.energy_level -= 1;
    PET.save(deps.storage, &pet)?;

    Ok(Response::default())
}

pub fn try_rest(deps: DepsMut, env: Env, info: MessageInfo, amount: u8) -> StdResult<Response> {
    let owner = deps.api.addr_humanize(&OWNER.load(deps.storage)?)?;
    if info.sender != owner {
        return Err(StdError::generic_err("Unauthorised"));
    }

    //Increase energy_level by amount (ensure it doesn’t exceed 10).
    if amount > 10 {
        return Err(StdError::generic_err("Amount must be between 0 and 10"));
    }

    let mut pet = update_state(deps.storage, env).unwrap();
    pet.energy_level = (pet.energy_level + amount).min(10);
    PET.save(deps.storage, &pet)?;

    Ok(Response::default())
}

pub fn try_transfer(deps: DepsMut, info: MessageInfo, new_owner: String) -> StdResult<Response> {
    let owner = deps.api.addr_humanize(&OWNER.load(deps.storage)?)?;
    if info.sender != owner {
        return Err(StdError::generic_err("Unauthorised"));
    }

    let new_owner = deps.api.addr_validate(&new_owner)?;
    let new_owner = deps.api.addr_canonicalize(new_owner.as_str())?;
    OWNER.save(deps.storage, &new_owner)?;

    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsHungry { password } => try_is_hungry(deps, password, env),
        QueryMsg::GetStatus { password } => try_get_status(deps, password),
    }
}

fn check_password(deps: Deps, password: String) -> StdResult<()> {
    let stored_password = PASSWORD.load(deps.storage)?;
    if stored_password != password {
        return Err(StdError::generic_err("Incorrect Password"));
    }
    Ok(())
}

pub fn try_is_hungry(deps: Deps, password: String, env: Env) -> StdResult<Binary> {
    check_password(deps, password)?;
    let pet = PET.load(deps.storage)?;
    let hunger = pet.hunger_level;
    let last_action_block = pet.last_action_block;
    let blocks_passed = env.block.height - last_action_block;
    let hunger = (hunger + blocks_passed as u8).min(10);
    Ok(
        to_binary(&QueryAnswer::IsHungry { is_hungry: (hunger >= 7) })?
    )
}

pub fn try_get_status(deps: Deps, password: String) -> StdResult<Binary> {
    check_password(deps, password)?;
    let pet = PET.load(deps.storage)?;
    Ok(
        to_binary(&QueryAnswer::GetStatus {
            name: pet.name,
            hunger_level: pet.hunger_level,
            happiness_level: pet.happiness_level,
            energy_level: pet.energy_level,
        })?
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{testing::*, Api};
    

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let name = "Fluffy".to_string();
        let owner = Some("creator".to_string());

        let msg = InstantiateMsg {
            name,
            owner: owner.map(|o| deps.api.addr_validate(&o).unwrap()),
        };
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}


